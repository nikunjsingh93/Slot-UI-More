use std::sync::Arc;

use super::ring::{ring_capacity, Ring};
use super::sink::{AudioError, AudioSink};

#[derive(Clone)]
pub struct StubSink {
    ring: Arc<Ring>,
}

impl StubSink {
    pub fn new() -> Self {
        StubSink {
            ring: Arc::new(Ring::new(ring_capacity(32768))),
        }
    }

    pub fn draining() -> Self {
        let sink = StubSink::new();
        let ring = sink.ring.clone();
        std::thread::spawn(move || loop {
            let frames = ring.queued_frames();
            ring.fill(&mut vec![0i16; frames * 2]);
            std::thread::sleep(std::time::Duration::from_millis(2));
        });
        sink
    }

    pub fn muted(&self) -> bool {
        self.ring.muted()
    }

    pub fn device_read(&self, frames: usize) -> Vec<i16> {
        let mut out = vec![0i16; frames * 2];
        self.ring.fill(&mut out);
        out
    }

    pub fn device_drain(&self) -> usize {
        let frames = self.ring.queued_frames();
        self.ring.fill(&mut vec![0i16; frames * 2]);
        frames
    }
}

impl Default for StubSink {
    fn default() -> Self {
        StubSink::new()
    }
}

impl AudioSink for StubSink {
    fn open(&mut self, sample_rate: u32) -> Result<(), AudioError> {
        self.ring.reopen(sample_rate);
        Ok(())
    }

    fn ring(&self) -> Arc<Ring> {
        self.ring.clone()
    }
}
