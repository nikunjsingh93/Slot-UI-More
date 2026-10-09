use std::sync::atomic::{AtomicU16, AtomicU64, Ordering};
use std::sync::OnceLock;
use std::time::Instant;

pub const TRACE_VAR: &str = "SLOT_TRACE_LATENCY";
pub const FLAG_FILE: &str = "System/trace-latency.on";

static READ: AtomicU64 = AtomicU64::new(0);
static APPLIED: AtomicU64 = AtomicU64::new(0);
static EMU: AtomicU64 = AtomicU64::new(0);
static PUBLISHED: AtomicU64 = AtomicU64::new(0);
static TAKEN: AtomicU64 = AtomicU64::new(0);
static BITS: AtomicU16 = AtomicU16::new(0);

fn on() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| {
        let card = std::env::var_os("SLOT_ROOT").unwrap_or_else(|| "/mnt/sdcard".into());
        std::env::var_os(TRACE_VAR).is_some_and(|v| v != "0")
            || std::path::Path::new(&card).join(FLAG_FILE).is_file()
    })
}

fn now() -> u64 {
    static EPOCH: OnceLock<Instant> = OnceLock::new();
    EPOCH.get_or_init(Instant::now).elapsed().as_nanos() as u64 + 1
}

fn stamp(slot: &AtomicU64, after: &AtomicU64) -> bool {
    if after.load(Ordering::Acquire) == 0 {
        return false;
    }
    slot.compare_exchange(0, now(), Ordering::AcqRel, Ordering::Relaxed)
        .is_ok()
}

pub fn read() {
    if on() {
        let _ = READ.compare_exchange(0, now(), Ordering::AcqRel, Ordering::Relaxed);
    }
}

pub fn applied(before: u16, after: u16) {
    let added = after & !before;
    if on() && added != 0 && READ.load(Ordering::Acquire) != 0 {
        BITS.store(added, Ordering::Release);
        stamp(&APPLIED, &READ);
    }
}

pub fn emu_frame(mask: u16) {
    if on() && mask & BITS.load(Ordering::Acquire) != 0 {
        stamp(&EMU, &APPLIED);
    }
}

pub fn published() {
    if on() {
        stamp(&PUBLISHED, &EMU);
    }
}

pub fn taken() {
    if on() {
        stamp(&TAKEN, &PUBLISHED);
    }
}

pub fn swapped(swap_ms: f64) {
    if !on() || TAKEN.load(Ordering::Acquire) == 0 {
        return;
    }
    let end = now();
    let [r, a, e, p, t] =
        [&READ, &APPLIED, &EMU, &PUBLISHED, &TAKEN].map(|s| s.swap(0, Ordering::AcqRel));
    BITS.store(0, Ordering::Release);
    let ms = |from: u64, to: u64| to.saturating_sub(from) as f64 / 1e6;
    eprintln!(
        "slot: latency: read>pad {:.1} pad>emu {:.1} emu>pub {:.1} pub>take {:.1} take>swapped {:.1} (swap {:.1}) total {:.1} ms",
        ms(r, a),
        ms(a, e),
        ms(e, p),
        ms(p, t),
        ms(t, end),
        swap_ms,
        ms(r, end)
    );
}

pub fn tracing() -> bool {
    on()
}
