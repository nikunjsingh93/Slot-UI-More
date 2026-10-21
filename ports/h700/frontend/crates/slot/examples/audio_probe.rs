//! Exercise the real device ALSA sink with an independently generated audible tone.
use slot::audio::{AlsaSink, AudioSink};
use std::time::Duration;

fn main() {
    let mut sink = AlsaSink::new();
    sink.open(32768).expect("open ALSA PCM");
    let ring = sink.ring();
    let rate = ring.sample_rate();
    for block in 0..32 {
        let mut samples = Vec::with_capacity(2048);
        for frame in 0..1024 {
            let phase = (block * 1024 + frame) as f32 * 440.0 / rate as f32;
            let sample = (phase * std::f32::consts::TAU).sin() * 4000.0;
            samples.extend([sample as i16, sample as i16]);
        }
        ring.push(&samples);
        std::thread::sleep(Duration::from_secs_f64(1024.0 / rate as f64));
    }
    std::thread::sleep(Duration::from_millis(100));
    println!("ALSA probe: 440Hz stereo samples submitted");
}
