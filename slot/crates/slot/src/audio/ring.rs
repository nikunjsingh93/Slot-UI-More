use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use crate::drc::drc_target;

pub fn ring_capacity(sample_rate: u32) -> usize {
    sample_rate as usize * 8 / 60
}

struct Inner {
    buf: Vec<i16>,
    head: usize,
    len: usize,
    pending: Vec<i16>,
    placed: usize,
    pending_at: u64,
    drained: u64,
}

pub struct Ring {
    inner: Mutex<Inner>,
    room: Condvar,
    queued: AtomicUsize,
    capacity: AtomicUsize,
    rate: AtomicU32,
    muted: AtomicBool,
    idle: AtomicBool,
    overruns: AtomicU64,
    underruns: AtomicU64,
    consuming: AtomicBool,
    priming: AtomicBool,
    wait_ms: AtomicU32,
}

impl Ring {
    pub fn new(capacity_frames: usize) -> Self {
        Ring {
            inner: Mutex::new(Inner {
                buf: vec![0; capacity_frames * 2],
                head: 0,
                len: 0,
                pending: Vec::new(),
                placed: 0,
                pending_at: 0,
                drained: 0,
            }),
            room: Condvar::new(),
            queued: AtomicUsize::new(0),
            idle: AtomicBool::new(false),
            capacity: AtomicUsize::new(capacity_frames),
            rate: AtomicU32::new(0),
            muted: AtomicBool::new(false),
            overruns: AtomicU64::new(0),
            underruns: AtomicU64::new(0),
            consuming: AtomicBool::new(true),
            priming: AtomicBool::new(false),
            wait_ms: AtomicU32::new(50),
        }
    }

    pub fn reopen(&self, sample_rate: u32) {
        let frames = ring_capacity(sample_rate);
        let mut i = self.lock();
        i.buf = vec![0; frames * 2];
        i.head = 0;
        i.pending = Vec::new();
        i.placed = 0;
        i.pending_at = 0;
        i.drained = 0;
        i.len = drc_target(frames) * 2;
        self.queued.store(i.len, Ordering::Relaxed);
        self.capacity.store(frames, Ordering::Relaxed);
        self.rate.store(sample_rate, Ordering::Relaxed);
        self.overruns.store(0, Ordering::Relaxed);
        self.underruns.store(0, Ordering::Relaxed);
        self.consuming.store(true, Ordering::Relaxed);
        self.priming.store(false, Ordering::Relaxed);
        self.wait_ms.store(50, Ordering::Relaxed);
    }

    /// Select a larger queue only for burst-producing cores; reset on system changes.
    pub fn configure_buffer_ms(&self, millis: u32) {
        let rate = self.sample_rate();
        if rate == 0 {
            return;
        }
        let frames = if millis == 0 {
            ring_capacity(rate)
        } else {
            rate as usize * millis.min(500) as usize / 1000
        };
        self.wait_ms.store(
            if millis == 0 {
                50
            } else {
                (millis / 2).clamp(50, 250)
            },
            Ordering::Relaxed,
        );
        if frames == self.capacity_frames() {
            return;
        }
        let mut i = self.lock();
        i.buf = vec![0; frames * 2];
        i.head = 0;
        i.len = drc_target(frames) * 2;
        i.pending.clear();
        i.placed = 0;
        i.pending_at = 0;
        i.drained = 0;
        self.capacity.store(frames, Ordering::Relaxed);
        self.queued.store(i.len, Ordering::Relaxed);
        self.priming.store(false, Ordering::Relaxed);
        self.consuming.store(true, Ordering::Relaxed);
        self.room.notify_all();
    }

    fn max_wait(&self) -> Duration {
        Duration::from_millis(u64::from(self.wait_ms.load(Ordering::Relaxed)))
    }

    pub fn push(&self, samples: &[i16]) {
        let mut i = self.lock();
        let lost = write_into(&mut i, samples).len();
        self.queued.store(i.len, Ordering::Relaxed);
        drop(i);
        if lost > 0 && !self.muted() && self.capacity_frames() > 0 {
            self.overruns.fetch_add(lost as u64, Ordering::Relaxed);
        }
    }

    pub fn push_blocking(&self, samples: &[i16]) {
        if self.muted() || self.capacity_frames() == 0 || !self.consuming.load(Ordering::Relaxed) {
            self.push(samples);
            return;
        }
        let deadline = Instant::now() + self.max_wait();
        let mut rest = samples;
        let mut i = self.lock();
        loop {
            rest = write_into(&mut i, rest);
            self.queued.store(i.len, Ordering::Relaxed);
            if rest.is_empty() || self.muted() {
                return;
            }
            let Some(left) = deadline.checked_duration_since(Instant::now()) else {
                self.consuming.store(false, Ordering::Relaxed);
                self.overruns
                    .fetch_add(rest.len() as u64, Ordering::Relaxed);
                return;
            };
            i = self
                .room
                .wait_timeout(i, left)
                .map(|(guard, _)| guard)
                .unwrap_or_else(|e| e.into_inner().0);
        }
    }

    /// Wait for the physical audio consumer, without coupling it to screen refresh.
    /// Return false for a missing/stalled consumer so shutdown and test sinks stay bounded.
    pub fn wait_below(&self, target_frames: usize) -> bool {
        if self.sample_rate() == 0 || target_frames == 0 || self.muted() {
            return false;
        }
        let until = Instant::now() + self.max_wait();
        let mut i = self.lock();
        while i.len / 2 > target_frames && !self.muted() {
            let Some(left) = until.checked_duration_since(Instant::now()) else {
                return false;
            };
            i = self
                .room
                .wait_timeout(i, left)
                .map(|(guard, _)| guard)
                .unwrap_or_else(|e| e.into_inner().0);
        }
        !self.muted()
    }

    pub fn mix(&self, samples: &[i16]) {
        self.priming.store(false, Ordering::Relaxed);
        let mut i = self.lock();
        i.pending.clear();
        i.placed = 0;
        if i.buf.is_empty() {
            return;
        }
        i.pending_at = i.drained;
        i.pending.extend_from_slice(samples);
        lay_pending(&mut i);
        self.queued.store(i.len, Ordering::Relaxed);
    }

    pub fn fill(&self, out: &mut [i16]) {
        let mut i = self.lock();
        lay_pending(&mut i);
        let cap = i.buf.len();
        let holding = self.priming.load(Ordering::Relaxed) && cap > 0 && i.len < cap / 2;
        self.priming.store(holding, Ordering::Relaxed);
        let n = if holding { 0 } else { out.len().min(i.len) };
        if cap > 0 && !holding {
            let first = n.min(cap - i.head);
            out[..first].copy_from_slice(&i.buf[i.head..i.head + first]);
            out[first..n].copy_from_slice(&i.buf[..n - first]);
            i.head = (i.head + n) % cap;
            i.len -= n;
            i.drained += n as u64;
        }
        out[n..].fill(0);
        self.queued.store(i.len, Ordering::Relaxed);
        drop(i);
        self.consuming.store(true, Ordering::Relaxed);
        self.room.notify_all();
        let short = out.len() - n;
        if short > 0 && !self.muted() && !self.idle.load(Ordering::Relaxed) && cap > 0 {
            self.underruns.fetch_add(short as u64, Ordering::Relaxed);
            self.priming.store(true, Ordering::Relaxed);
        }
        if self.muted() {
            out.fill(0);
        }
    }

    pub fn queued_frames(&self) -> usize {
        self.queued.load(Ordering::Relaxed) / 2
    }

    pub fn capacity_frames(&self) -> usize {
        self.capacity.load(Ordering::Relaxed)
    }

    pub fn overruns(&self) -> u64 {
        self.overruns.load(Ordering::Relaxed)
    }

    pub fn underruns(&self) -> u64 {
        self.underruns.load(Ordering::Relaxed)
    }

    pub fn clear_faults(&self) {
        self.overruns.store(0, Ordering::Relaxed);
        self.underruns.store(0, Ordering::Relaxed);
    }

    pub fn sample_rate(&self) -> u32 {
        self.rate.load(Ordering::Relaxed)
    }

    pub fn set_idle(&self, idle: bool) {
        self.idle.store(idle, Ordering::Relaxed);
    }

    pub fn set_muted(&self, muted: bool) {
        self.muted.store(muted, Ordering::Relaxed);
        self.room.notify_all();
    }

    pub fn muted(&self) -> bool {
        self.muted.load(Ordering::Relaxed)
    }

    fn lock(&self) -> MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(|e| e.into_inner())
    }
}

fn lay_pending(i: &mut Inner) {
    let cap = i.buf.len();
    if cap == 0 || i.placed >= i.pending.len() {
        return;
    }
    let Some(off) = i.pending_at.checked_sub(i.drained) else {
        i.pending.clear();
        i.placed = 0;
        return;
    };
    let off = off as usize;
    if off >= cap {
        return;
    }
    let n = (i.pending.len() - i.placed).min(cap - off);
    let len = i.len;
    for k in 0..n {
        let s = i.pending[i.placed + k];
        let at = (i.head + off + k) % cap;
        i.buf[at] = if off + k < len {
            i.buf[at].saturating_add(s)
        } else {
            s
        };
    }
    i.len = i.len.max(off + n);
    i.placed += n;
    i.pending_at += n as u64;
    if i.placed >= i.pending.len() {
        i.pending = Vec::new();
        i.placed = 0;
    }
}

fn write_into<'a>(i: &mut Inner, samples: &'a [i16]) -> &'a [i16] {
    let cap = i.buf.len();
    if cap == 0 {
        return samples;
    }
    let n = samples.len().min(cap - i.len);
    let start = (i.head + i.len) % cap;
    let first = n.min(cap - start);
    i.buf[start..start + first].copy_from_slice(&samples[..first]);
    i.buf[..n - first].copy_from_slice(&samples[first..n]);
    i.len += n;
    &samples[n..]
}

#[cfg(test)]
mod burst_tests {
    use super::*;

    fn after_render_gap(buffer_ms: u32) -> u64 {
        let ring = Ring::new(0);
        ring.reopen(48_000);
        ring.configure_buffer_ms(buffer_ms);
        // Replace startup silence with already-produced audio, then model a
        // 120 ms graphics stall while ALSA continues consuming 10 ms periods.
        ring.fill(&mut vec![0; ring.queued_frames() * 2]);
        ring.push(&vec![1000; drc_target(ring.capacity_frames()) * 2]);
        ring.clear_faults();
        for _ in 0..12 {
            ring.fill(&mut [0; 960]);
        }
        ring.underruns()
    }

    #[test]
    fn dreamcast_queue_survives_a_render_gap_that_starves_the_default_queue() {
        assert!(after_render_gap(0) > 0);
        assert_eq!(after_render_gap(320), 0);
    }

    #[test]
    fn switching_back_to_a_normal_core_restores_its_queue_and_wait_limit() {
        let ring = Ring::new(0);
        ring.reopen(48_000);
        ring.configure_buffer_ms(320);
        assert_eq!(ring.max_wait(), Duration::from_millis(160));
        ring.configure_buffer_ms(0);
        assert_eq!(ring.capacity_frames(), ring_capacity(48_000));
        assert_eq!(ring.max_wait(), Duration::from_millis(50));
    }
}
