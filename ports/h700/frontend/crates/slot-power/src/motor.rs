use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct Motor(Arc<AtomicU16>);

impl Motor {
    pub fn set(&self, strength: u16) {
        self.0.store(strength, Ordering::Relaxed);
    }

    pub fn last(&self) -> u16 {
        self.0.load(Ordering::Relaxed)
    }
}
