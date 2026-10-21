mod wav;

pub use wav::{read_wav, WavError};

pub const HZ: f32 = 48_000.0;

pub const INSERT_LEN: usize = 11_520;
pub const EJECT_LEN: usize = 15_120;

pub const INSERT_LEAD: f32 = 0.097;
pub const EJECT_LEAD: f32 = 0.021;

pub const INSERT_PEAK: f32 = 11_397.0;
pub const EJECT_PEAK: f32 = 12_000.0;

#[derive(Clone, Copy, Debug)]
pub struct Take {
    pub at: f32,
    pub peak: f32,
    pub clear_before: f32,
    pub clear_after: f32,
}

impl Take {
    pub fn fits(&self, lead: f32, len: usize) -> bool {
        let after = len as f32 / HZ - lead;
        self.clear_before >= lead && self.clear_after >= after
    }
}

pub fn takes(pcm: &[f32]) -> Vec<Take> {
    let rms: Vec<f32> = (0..pcm.len().saturating_sub(WIN))
        .step_by(HOP)
        .map(|i| {
            let w = &pcm[i..i + WIN];
            (w.iter().map(|v| v * v).sum::<f32>() / WIN as f32).sqrt()
        })
        .collect();

    let mut hits: Vec<usize> = Vec::new();
    for (k, &now) in rms.iter().enumerate().skip(HISTORY) {
        if now < FLOOR {
            continue;
        }
        let before = &rms[k - HISTORY..k];
        let quiet = before.iter().fold(f32::MAX, |m, &v| m.min(v)).max(1.0e-6);
        if 20.0 * (now / quiet).log10() < JUMP_DB {
            continue;
        }
        if hits
            .last()
            .is_some_and(|&p| (k - p) * HOP < (0.080 * HZ) as usize)
        {
            continue;
        }
        hits.push(k);
    }

    let peak_around = |centre: usize| {
        let lo = centre.saturating_sub((0.015 * HZ) as usize);
        let hi = (centre + (0.015 * HZ) as usize).min(pcm.len());
        pcm[lo..hi].iter().fold(0.0f32, |m, v| m.max(v.abs()))
    };

    let onsets: Vec<usize> = hits.iter().map(|&k| onset(pcm, k * HOP)).collect();

    let mut grouped: Vec<(usize, usize)> = Vec::new();
    for &sample in &onsets {
        match grouped.last_mut() {
            Some((anchor, last)) if (sample - *last) as f32 / HZ < GROUP => {
                *last = sample;
                if peak_around(sample) > peak_around(*anchor) {
                    *anchor = sample;
                }
            }
            _ => grouped.push((sample, sample)),
        }
    }
    let mut anchors: Vec<usize> = grouped.into_iter().map(|(a, _)| a).collect();

    let loudest = anchors
        .iter()
        .map(|&a| peak_around(a))
        .fold(0.0f32, f32::max);
    anchors.retain(|&a| peak_around(a) >= loudest * 0.10);

    let total = pcm.len() as f32 / HZ;
    anchors
        .iter()
        .enumerate()
        .map(|(n, &sample)| {
            let at = sample as f32 / HZ;
            Take {
                at,
                peak: peak_around(sample),
                clear_before: match n {
                    0 => at,
                    _ => at - anchors[n - 1] as f32 / HZ,
                },
                clear_after: match anchors.get(n + 1) {
                    Some(&next) => next as f32 / HZ - at,
                    None => total - at,
                },
            }
        })
        .collect()
}

const HOP: usize = 240;
const WIN: usize = 480;
const HISTORY: usize = 20;
const JUMP_DB: f32 = 12.0;
const FLOOR: f32 = 0.004;
const GROUP: f32 = 0.400;

fn onset(pcm: &[f32], hop_start: usize) -> usize {
    let lo = hop_start.saturating_sub(WIN);
    let hi = (hop_start + 3 * WIN).min(pcm.len());
    if lo >= hi {
        return hop_start;
    }
    let gate = pcm[lo..hi].iter().fold(0.0f32, |m, v| m.max(v.abs())) / 3.0;
    (lo..hi)
        .find(|&i| pcm[i].abs() >= gate)
        .unwrap_or(hop_start)
}

#[derive(Debug)]
pub enum CutError {
    NotEnoughBefore { want: f32, have: f32 },
    NotEnoughAfter { want: f32, have: f32 },
}

impl std::fmt::Display for CutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotEnoughBefore { want, have } => write!(
                f,
                "needs {:.0} ms before the transient, the take has {:.0} ms",
                want * 1000.0,
                have * 1000.0
            ),
            Self::NotEnoughAfter { want, have } => write!(
                f,
                "needs {:.0} ms after the transient, the take has {:.0} ms",
                want * 1000.0,
                have * 1000.0
            ),
        }
    }
}

pub fn cut(
    pcm: &[f32],
    at: f32,
    lead: f32,
    len: usize,
    peak: f32,
    lift_db: f32,
) -> Result<Vec<i16>, CutError> {
    let start = (at - lead) * HZ;
    if start < 0.0 {
        return Err(CutError::NotEnoughBefore {
            want: lead,
            have: at,
        });
    }
    let start = start as usize;
    if start + len > pcm.len() {
        return Err(CutError::NotEnoughAfter {
            want: len as f32 / HZ - lead,
            have: (pcm.len() - start.min(pcm.len())) as f32 / HZ - lead,
        });
    }
    let mut buf = pcm[start..start + len].to_vec();
    lift(&mut buf, lead, lift_db);
    Ok(finish(&mut buf, peak))
}

fn lift(buf: &mut [f32], lead: f32, db: f32) {
    if db == 0.0 {
        return;
    }
    let gain = 10.0f32.powf(db / 20.0);
    let at = (lead * HZ) as usize;
    let ramp = (0.015 * HZ) as usize;
    for (i, s) in buf.iter_mut().enumerate().take(at) {
        let u = ((at - i) as f32 / ramp as f32).min(1.0);
        *s *= 1.0 + (gain - 1.0) * raised_cosine(u);
    }
}

fn finish(buf: &mut [f32], peak: f32) -> Vec<i16> {
    const IN: usize = 96;
    const OUT: usize = 900;

    let n = buf.len();
    for (i, s) in buf.iter_mut().enumerate() {
        let head = (i as f32 / IN as f32).min(1.0);
        let tail = ((n - 1 - i) as f32 / OUT as f32).min(1.0);
        *s *= raised_cosine(head) * raised_cosine(tail);
    }

    let loudest = buf.iter().fold(0.0f32, |m, v| m.max(v.abs()));
    let gain = if loudest > 0.0 { peak / loudest } else { 0.0 };
    buf.iter()
        .map(|v| (v * gain).round().clamp(-32_768.0, 32_767.0) as i16)
        .collect()
}

fn raised_cosine(u: f32) -> f32 {
    0.5 * (1.0 - (std::f32::consts::PI * u).cos())
}

pub fn to_le_bytes(pcm: &[i16]) -> Vec<u8> {
    pcm.iter().flat_map(|s| s.to_le_bytes()).collect()
}
