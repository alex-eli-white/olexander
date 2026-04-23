// use std::{
//     error::Error,
//     sync::mpsc,
//     thread,
//     time::{Duration, Instant},
// };

// use cpal::{
//     BufferSize, SampleRate, StreamConfig,
//     traits::{DeviceTrait, HostTrait, StreamTrait},
// };

// use ringbuf::{
//     HeapRb,
//     traits::{Consumer, Producer, Split},
// };

// fn main() -> Result<(), Box<dyn Error>> {
//     let host = cpal::default_host();

//     let input_device = host
//         .default_input_device()
//         .ok_or("no default input device")?;

//     let output_device = host
//         .default_output_device()
//         .ok_or("no default output device")?;

//     println!("input:  {}", input_device.name()?);

//     println!("output: {}", output_device.name()?);

//     let sample_rate = 48_000u32;

//     let buffer_size = 128u32;

//     let config = StreamConfig {
//         channels: 2,

//         sample_rate: SampleRate(sample_rate),

//         buffer_size: BufferSize::Fixed(buffer_size),
//     };

//     let ring = HeapRb::<f32>::new(sample_rate as usize);

//     let (mut producer, mut consumer) = ring.split();

//     let (analysis_tx, analysis_rx) = mpsc::sync_channel::<GuitarAnalysis>(64);

//     let mut engine = OlexanderEngine::new(sample_rate as f32);

//     let input_stream = input_device.build_input_stream(
//         &config,
//         move |input: &[f32], _info| {
//             let mut mono_block = Vec::with_capacity(input.len() / 2);

//             for frame in input.chunks_exact(2) {
//                 let guitar = frame[0];

//                 mono_block.push(guitar);

//                 let _ = producer.try_push(guitar);
//             }

//             let analysis = engine.analyze_block(&mono_block);

//             // Non-blocking-ish: sync_channel try_send prevents audio callback blocking.

//             let _ = analysis_tx.try_send(analysis);
//         },
//         move |err| {
//             eprintln!("input stream error: {err}");
//         },
//         None,
//     )?;

//     let output_stream = output_device.build_output_stream(
//         &config,
//         move |output: &mut [f32], _info| {
//             for frame in output.chunks_exact_mut(2) {
//                 let sample = consumer.try_pop().unwrap_or(0.0);

//                 frame[0] = sample;

//                 frame[1] = sample;
//             }
//         },
//         move |err| {
//             eprintln!("output stream error: {err}");
//         },
//         None,
//     )?;

//     input_stream.play()?;

//     output_stream.play()?;

//     println!("Olexander v1 running. Ctrl-C to quit.");

//     let mut last_print = Instant::now();

//     loop {
//         if let Ok(analysis) = analysis_rx.recv_timeout(Duration::from_millis(50)) {
//             if last_print.elapsed() >= Duration::from_millis(100) {
//                 println!(
//                     "rms={:.4} peak={:.4} env={:.4} onset={}",
//                     analysis.rms, analysis.peak, analysis.envelope, analysis.onset
//                 );

//                 last_print = Instant::now();
//             }
//         }

//         thread::sleep(Duration::from_millis(1));
//     }
// }

use std::{
    error::Error,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use cpal::{
    BufferSize, SampleRate, StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use ringbuf::{
    HeapRb,
    traits::{Consumer, Producer, Split},
};

#[derive(Debug, Clone, Copy)]
pub struct GuitarAnalysis {
    pub rms: f32,
    pub peak: f32,
    pub envelope: f32,
    pub onset: bool,
}

pub struct EnvelopeFollower {
    attack_coeff: f32,
    release_coeff: f32,
    value: f32,
}

impl EnvelopeFollower {
    pub fn new(sample_rate: f32, attack_ms: f32, release_ms: f32) -> Self {
        fn coeff(sample_rate: f32, ms: f32) -> f32 {
            (-1.0 / (0.001 * ms * sample_rate)).exp()
        }

        Self {
            attack_coeff: coeff(sample_rate, attack_ms),
            release_coeff: coeff(sample_rate, release_ms),
            value: 0.0,
        }
    }

    pub fn process(&mut self, input_abs: f32) -> f32 {
        let coeff = if input_abs > self.value {
            self.attack_coeff
        } else {
            self.release_coeff
        };

        self.value = input_abs + coeff * (self.value - input_abs);
        self.value
    }
}

pub struct OnsetDetector {
    prev_block_env: f32,
    cooldown_blocks: usize,
    cooldown_remaining: usize,
}

impl OnsetDetector {
    pub fn new() -> Self {
        Self {
            prev_block_env: 0.0,
            cooldown_blocks: 8,
            cooldown_remaining: 0,
        }
    }

    pub fn process(&mut self, env: f32, peak: f32) -> bool {
        if self.cooldown_remaining > 0 {
            self.cooldown_remaining -= 1;
            self.prev_block_env = env;
            return false;
        }

        let jump = env - self.prev_block_env;
        self.prev_block_env = env;

        if peak > 0.05 && jump > 0.04 {
            self.cooldown_remaining = self.cooldown_blocks;
            true
        } else {
            false
        }
    }
}

pub struct OlexanderEngine {
    envelope: EnvelopeFollower,
    onset: OnsetDetector,
}

impl OlexanderEngine {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            envelope: EnvelopeFollower::new(sample_rate, 5.0, 80.0),
            onset: OnsetDetector::new(),
        }
    }

    pub fn analyze_block(&mut self, mono: &[f32]) -> GuitarAnalysis {
        let mut peak: f32 = 0.0;
        let mut sum_sq = 0.0;
        let mut envelope = 0.0;
        let mut onset = false;

        for &sample in mono {
            let abs = sample.abs();
            peak = peak.max(abs);
            sum_sq += sample * sample;

            envelope = self.envelope.process(abs);

            if self.onset.process(envelope, peak) {
                onset = true;
            }
        }

        let rms = if mono.is_empty() {
            0.0
        } else {
            (sum_sq / mono.len() as f32).sqrt()
        };

        GuitarAnalysis {
            rms,
            peak,
            envelope,
            onset,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_channel: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    println!("using input channel: {}", input_channel);
    let host = cpal::default_host();

    let input_device = host
        .default_input_device()
        .ok_or("no default input device")?;

    let output_device = host
        .default_output_device()
        .ok_or("no default output device")?;

    println!("input:  {}", input_device.description()?);
    println!("output: {}", output_device.description()?);

    let sample_rate = 48_000u32;
    let buffer_size = 128u32;

    let config = StreamConfig {
        channels: 2,
        sample_rate,
        buffer_size: BufferSize::Fixed(buffer_size),
    };

    let ring = HeapRb::<f32>::new(sample_rate as usize);
    let (mut producer, mut consumer) = ring.split();

    let (analysis_tx, analysis_rx) = mpsc::sync_channel::<GuitarAnalysis>(64);

    let mut engine = OlexanderEngine::new(sample_rate as f32);

    let input_stream = input_device.build_input_stream(
        &config,
        move |input: &[f32], _info| {
            let mut ch0_peak = 0.0f32;
            let mut ch1_peak = 0.0f32;

            for frame in input.chunks_exact(2) {
                ch0_peak = ch0_peak.max(frame[0].abs());
                ch1_peak = ch1_peak.max(frame[1].abs());
            }

            let guitar_channel = 0; // change to 1 if input 2 is your guitar

            let mut peak: f32 = 0.0;
            let mut sum_sq = 0.0;
            let mut envelope = 0.0;
            let mut onset = false;
            let mut count = 0usize;

            for frame in input.chunks_exact(2) {
                let guitar = frame[guitar_channel];

                let _ = producer.try_push(guitar);

                let abs = guitar.abs();
                peak = peak.max(abs);
                sum_sq += guitar * guitar;
                count += 1;

                envelope = engine.envelope.process(abs);
                let mut onset = engine.onset.process(envelope, peak);
                if engine.onset.process(envelope, peak) {
                    onset = true;
                }
            }

            let rms = if count == 0 {
                0.0
            } else {
                (sum_sq / count as f32).sqrt()
            };

            let _ = analysis_tx.try_send(GuitarAnalysis {
                rms,
                peak,
                envelope,
                onset,
            });

            // Optional: temporary channel debug, but DO NOT println here.
            // Send this through another channel if needed.
        },
        move |err| {
            eprintln!("input stream error: {err}");
        },
        None,
    )?;

    let output_stream = output_device.build_output_stream(
        &config,
        move |output: &mut [f32], _info| {
            for frame in output.chunks_exact_mut(2) {
                let sample = consumer.try_pop().unwrap_or(0.0);

                frame[0] = sample;
                frame[1] = sample;
            }
        },
        move |err| {
            eprintln!("output stream error: {err}");
        },
        None,
    )?;

    input_stream.play()?;
    output_stream.play()?;

    println!("Olexander v1 running. Ctrl-C to quit.");

    let mut last_print = Instant::now();

    loop {
        if let Ok(analysis) = analysis_rx.recv_timeout(Duration::from_millis(50)) {
            if last_print.elapsed() >= Duration::from_millis(300) {
                println!(
                    "rms={:.4} peak={:.4} env={:.4} onset={} clipped={}",
                    analysis.rms,
                    analysis.peak,
                    analysis.envelope,
                    analysis.onset,
                    analysis.peak >= 0.999,
                );

                last_print = Instant::now();
            }
        }

        thread::sleep(Duration::from_millis(1));
    }
}
