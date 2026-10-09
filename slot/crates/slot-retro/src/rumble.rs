use std::ffi::c_uint;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;

use crate::ffi::{RUMBLE_STRONG, RUMBLE_WEAK};

#[derive(Clone, Default)]
pub struct Rumble(Arc<Motors>);

#[derive(Default)]
struct Motors {
    strong: AtomicU16,
    weak: AtomicU16,
}

impl Rumble {
    pub fn set(&self, port: c_uint, effect: c_uint, strength: u16) -> bool {
        if port != 0 {
            return false;
        }
        let motor = match effect {
            RUMBLE_STRONG => &self.0.strong,
            RUMBLE_WEAK => &self.0.weak,
            _ => return false,
        };
        motor.store(strength, Ordering::Relaxed);
        true
    }

    pub fn strength(&self) -> u16 {
        self.0
            .strong
            .load(Ordering::Relaxed)
            .max(self.0.weak.load(Ordering::Relaxed))
    }
}
