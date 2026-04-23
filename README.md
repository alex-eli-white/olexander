# bum piss

Bum piss is aiming to be a single device for a robust live show for guitarists.

Let's build this out properly.

---

**Bum Piss - Feature Specification v0.1**

---

**Looper**
- Unlimited overdub layers (RAM permitting, you have 16GB)
- Per-layer volume and mute
- Half speed / double speed per layer
- Reverse playback
- Layer fade out / decay
- Undo last overdub
- Sync to sequencer BPM or free running
- Footswitch controls: record, overdub, play/stop, undo, clear
- Pickguard switch = punch in/out while playing
- Visual waveform display per layer on touchscreen

---

**Mode System**
- Performance mode - large touch targets, locked params, nothing accidental
- Edit mode - full parameter access, pattern programming, synth design
- Scene system - discrete snapshots of the full instrument state
- Setlist mode - chain scenes with transition triggers
- Smooth crossfade between scenes
- Per-scene BPM, key, scale

---

**Drum and Sample Machine**
- 16 track step sequencer, variable length per track
- Load any WAV from Samples from Mars or your own hits
- Per-step: velocity, probability, micro-timing offset, pitch shift
- Euclidean rhythm generator per track
- Pattern mutation - small random drifts over time
- Swing as continuous parameter
- Call and response - reacts to your guitar dynamics via onset detection
- Kaoss pad drag to generate and morph patterns
- Humanization engine - gaussian micro-timing, velocity variation, ghost notes
- Scene-based pattern chaining
- Visual drum grid with warm LED aesthetic
- Sidechain signal out to duck bass/pads on kick hits

---

**Synth Engine**
- Multiple voice architecture:
    - Lead voice - pitch tracked from guitar input
    - Bass voice - independent, sequencer driven or manual
    - Pad voice - chord/drone, sustained ambient texture
    - Texture voice - noise, granular, lo-fi atmosphere
- Per voice oscillator options:
    - Detuned saw stack (BMSR bread and butter)
    - Square with pulse width modulation
    - Wavetable with morphing
    - Noise with filtering
    - Sample playback (your Mars library)
- Per voice modulation:
    - ADSR envelope
    - LFO with multiple shapes, free or tempo synced
    - Velocity sensitivity
    - Aftertouch / expression pedal mapped to any param
- Filter per voice:
    - Low pass, high pass, band pass, notch
    - Resonance with self oscillation
    - Filter envelope
    - Analog drift simulation
- Global voice routing:
    - Unison with detune spread
    - Chord mode - play one note, get a chord
    - Arpeggiator with pattern and rate

---

**Effects Chain**
- Per voice insert effects:
    - Fuzz / distortion with tone control
    - Bit crusher and sample rate reducer
    - Chorus / ensemble (very BMSR)
    - Flanger
    - Phaser
- Send effects (shared):
    - Tape delay with wow and flutter
    - Reverb (room, hall, plate, shimmer)
    - Pitch shifter / harmonizer
- Master bus:
    - Tape saturation
    - Compressor with sidechain input
    - Stereo width
    - Soft limiter
- Guitar specific:
    - Tuner
    - Gate / noise suppressor
    - Cabinet simulation for direct output to PA

---

**Touch Interface**
- Kaoss XY pad - main performance surface
    - X and Y mappable to any parameter per scene
    - Fluid color bloom on touch with slow diffusion
    - Multiple touch points for multiple parameter pairs
- Drum grid - warm LED style, drag to generate patterns
- Looper layer view - waveform strips, tap to mute
- Scene launcher - grid of scenes, swipe to transition
- Synth editor - per voice deep edit in edit mode
- Visual style:
    - Deep purple and magenta palette
    - Warm amber for active/hot elements
    - Soft glow rendering, no harsh edges
    - Slightly wobbly animations, organic feel
    - Reacts to audio level - UI breathes with the music

---

**Hardware Integration**
- Pico 2 WH owns all physical I/O:
    - 3x footswitch inputs
    - 1x expression pedal TRS
    - Pickguard switch via stereo cable or BLE
    - Sends clean events to Pi 5 over USB serial
- Behringer UMC202HD for audio I/O
- 7" touchscreen primary interface
- All GPIO debounced in Embassy/Rust on Pico

---

**Performance Goals**
- Audio latency under 10ms round trip
- UI refresh 60fps without affecting audio thread
- Audio callback isolated on dedicated core
- Lock-free communication between audio and UI threads
- Graceful overflow handling (you already wrote this for Eli)

---

That's the full picture. Anything you want to add, cut, or reprioritize before we init the workspace and write the first lines?