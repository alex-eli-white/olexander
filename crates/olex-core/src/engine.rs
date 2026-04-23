use olex_proto::guitar::analysis::GuitarAnalysis;

use crate::{envelope::EnvelopeFollower, onset_detection::OnsetDetector};

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

    pub fn process_sample(&mut self, input: f32) -> (f32, GuitarAnalysis) {
        let abs = input.abs();
        let env = self.envelope.process(abs);
        let onset = self.onset.process(env);

        let analysis = GuitarAnalysis {
            rms: 0.0, // do block-level later
            peak: abs,
            envelope: env,
            onset,
        };

        // passthrough for now
        (input, analysis)
    }
}
