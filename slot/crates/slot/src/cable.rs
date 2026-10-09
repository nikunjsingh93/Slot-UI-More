use std::collections::BTreeMap;

use slot_retro::ButtonMask;

pub const DELAY: u64 = 3;

const PACKET: usize = 11;

const KIND_MASK: u8 = 0;
const KIND_STATE: u8 = 1;
const KIND_STATE_END: u8 = 2;
const KIND_READY: u8 = 3;

const CHUNK: usize = 60_000;

pub fn encode(frame: u64, mask: ButtonMask) -> [u8; PACKET] {
    let mut out = [0u8; PACKET];
    out[0] = KIND_MASK;
    out[1..9].copy_from_slice(&frame.to_le_bytes());
    out[9..].copy_from_slice(&mask.0.to_le_bytes());
    out
}

pub fn decode(buf: &[u8]) -> Option<(u64, ButtonMask)> {
    if buf.len() != PACKET || buf[0] != KIND_MASK {
        return None;
    }
    let frame = u64::from_le_bytes(buf[1..9].try_into().ok()?);
    let mask = u16::from_le_bytes(buf[9..].try_into().ok()?);
    Some((frame, ButtonMask(mask)))
}

pub fn ready_packet() -> [u8; 1] {
    [KIND_READY]
}

pub fn state_packets(state: &[u8]) -> Vec<Vec<u8>> {
    let mut out: Vec<Vec<u8>> = state
        .chunks(CHUNK)
        .map(|c| {
            let mut p = Vec::with_capacity(c.len() + 1);
            p.push(KIND_STATE);
            p.extend_from_slice(c);
            p
        })
        .collect();
    match out.last_mut() {
        Some(last) => last[0] = KIND_STATE_END,
        None => out.push(vec![KIND_STATE_END]),
    }
    out
}

pub struct Cable {
    player: u8,
    frame: u64,
    local: BTreeMap<u64, ButtonMask>,
    remote: BTreeMap<u64, ButtonMask>,
    stalled: u32,
    primed: bool,
    incoming: Vec<u8>,
    complete: bool,
    peer_ready: bool,
}

impl Cable {
    pub fn new(player: u8) -> Self {
        let idle = ButtonMask::default();
        let seed: BTreeMap<u64, ButtonMask> = (0..DELAY).map(|f| (f, idle)).collect();
        Cable {
            player,
            frame: 0,
            local: seed.clone(),
            remote: seed,
            stalled: 0,
            primed: player == 0,
            incoming: Vec::new(),
            complete: false,
            peer_ready: player != 0,
        }
    }

    pub fn primed(&self) -> bool {
        self.primed
    }

    pub fn armed(&self) -> bool {
        self.primed && self.peer_ready
    }

    pub fn take_state(&mut self) -> Option<Vec<u8>> {
        if !self.complete {
            return None;
        }
        self.complete = false;
        Some(std::mem::take(&mut self.incoming))
    }

    pub fn prime(&mut self) {
        self.primed = true;
    }

    pub fn player(&self) -> u8 {
        self.player
    }

    pub fn frame(&self) -> u64 {
        self.frame
    }

    pub fn stalled(&self) -> u32 {
        self.stalled
    }

    pub fn sample(&mut self, mask: ButtonMask) -> [u8; PACKET] {
        let frame = self.frame + DELAY;
        let mask = *self.local.entry(frame).or_insert(mask);
        encode(frame, mask)
    }

    pub fn accept(&mut self, buf: &[u8]) -> bool {
        match buf.first() {
            Some(&KIND_READY) => {
                self.peer_ready = true;
                return true;
            }
            Some(&KIND_STATE) => {
                self.incoming.extend_from_slice(&buf[1..]);
                return true;
            }
            Some(&KIND_STATE_END) => {
                self.incoming.extend_from_slice(&buf[1..]);
                self.complete = true;
                return true;
            }
            _ => {}
        }
        let Some((frame, mask)) = decode(buf) else {
            return false;
        };
        if frame < self.frame {
            return false;
        }
        self.remote.insert(frame, mask);
        true
    }

    pub fn ready(&self) -> Option<(ButtonMask, ButtonMask)> {
        if !self.primed {
            return None;
        }
        let mine = *self.local.get(&self.frame)?;
        let theirs = *self.remote.get(&self.frame)?;
        match self.player {
            0 => Some((mine, theirs)),
            _ => Some((theirs, mine)),
        }
    }

    pub fn advance(&mut self) {
        self.local.remove(&self.frame);
        self.remote.remove(&self.frame);
        self.frame += 1;
        self.stalled = 0;
    }

    pub fn stall(&mut self) {
        self.stalled = self.stalled.saturating_add(1);
    }
}

pub const QUIET_FRAMES: u32 = 60;
