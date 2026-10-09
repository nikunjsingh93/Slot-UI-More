use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{Battery, Charge, LedState, Motor, Platform};

pub struct SimPlatform {
    root: PathBuf,
    offset: i64,
    stopped: Option<i64>,
    motor: Motor,
    relinks: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl SimPlatform {
    pub fn new() -> Self {
        SimPlatform::at(root_from_env())
    }

    pub fn at(root: PathBuf) -> Self {
        SimPlatform {
            root,
            offset: 0,
            stopped: None,
            motor: Motor::default(),
            relinks: std::sync::Arc::default(),
        }
    }

    pub fn stopped_at(mut self, secs: i64) -> Self {
        self.stopped = Some(secs);
        self
    }

    pub fn relinks(&self) -> std::sync::Arc<std::sync::atomic::AtomicUsize> {
        self.relinks.clone()
    }

    pub fn motor(&self) -> Motor {
        self.motor.clone()
    }
}

impl Default for SimPlatform {
    fn default() -> Self {
        SimPlatform::new()
    }
}

impl Platform for SimPlatform {
    fn set_backlight(&mut self, _step: u8) {}

    fn battery(&self) -> Option<Battery> {
        Some(Battery {
            percent: 100,
            charge: Charge::Charging,
        })
    }

    fn charge(&self) -> Charge {
        Charge::Charging
    }

    fn set_led(&mut self, _state: LedState) {}

    fn restart(&mut self) -> ! {
        std::process::exit(0)
    }

    fn poweroff(&mut self) -> ! {
        std::process::exit(0)
    }

    fn root(&self) -> &Path {
        &self.root
    }

    fn now(&self) -> i64 {
        self.stopped.unwrap_or_else(|| system_secs() + self.offset)
    }

    fn set_clock(&mut self, secs: i64) {
        match &mut self.stopped {
            Some(stopped) => *stopped = secs,
            None => self.offset = secs - system_secs(),
        }
    }

    fn relink_adb(&mut self) -> bool {
        self.relinks
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        true
    }

    fn set_rumble(&mut self, strength: u16) {
        self.motor.set(strength);
    }
}

fn system_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn root_from_env() -> PathBuf {
    std::env::var_os("SLOT_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("sdcard"))
}
