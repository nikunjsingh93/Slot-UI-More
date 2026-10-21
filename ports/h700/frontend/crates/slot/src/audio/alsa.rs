use std::ffi::{c_char, c_int, c_uint, c_void, CStr, CString};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread::JoinHandle;
use std::time::Duration;

use libloading::Library;

use super::ring::Ring;
use super::sink::{AudioError, AudioSink};

const SND_PCM_STREAM_PLAYBACK: c_int = 0;
const SND_PCM_FORMAT_S16_LE: c_int = 2;
const SND_PCM_ACCESS_RW_INTERLEAVED: c_int = 3;
const CHANNELS: c_uint = 2;

const LATENCY_US: c_uint = 40_000;

const PERIOD_FRAMES: usize = 512;

const DEVICES: [&str; 4] = ["plug:default", "default", "plughw:0,0", "hw:0,0"];

type PcmOpen = unsafe extern "C" fn(*mut *mut c_void, *const c_char, c_int, c_int) -> c_int;
type PcmSetParams =
    unsafe extern "C" fn(*mut c_void, c_int, c_int, c_uint, c_uint, c_int, c_uint) -> c_int;
type PcmWritei = unsafe extern "C" fn(*mut c_void, *const c_void, u64) -> i64;
type PcmRecover = unsafe extern "C" fn(*mut c_void, c_int, c_int) -> c_int;
type PcmDrop = unsafe extern "C" fn(*mut c_void) -> c_int;
type PcmClose = unsafe extern "C" fn(*mut c_void) -> c_int;
type StrError = unsafe extern "C" fn(c_int) -> *const c_char;

struct Alsa {
    open: PcmOpen,
    set_params: PcmSetParams,
    writei: PcmWritei,
    recover: PcmRecover,
    drop: PcmDrop,
    close: PcmClose,
    strerror: StrError,
    _lib: Library,
}

impl Alsa {
    fn load() -> Result<Self, AudioError> {
        let lib = unsafe { Library::new("libasound.so.2") }
            .or_else(|_| unsafe { Library::new("libasound.so") })
            .map_err(|e| AudioError::Device(format!("libasound: {e}")))?;
        macro_rules! get {
            ($name:literal) => {
                *lib.get(concat!($name, "\0").as_bytes())
                    .map_err(|e| AudioError::Device(format!("{}: {e}", $name)))?
            };
        }
        unsafe {
            Ok(Alsa {
                open: get!("snd_pcm_open"),
                set_params: get!("snd_pcm_set_params"),
                writei: get!("snd_pcm_writei"),
                recover: get!("snd_pcm_recover"),
                drop: get!("snd_pcm_drop"),
                close: get!("snd_pcm_close"),
                strerror: get!("snd_strerror"),
                _lib: lib,
            })
        }
    }

    fn message(&self, err: c_int) -> String {
        let text = unsafe { (self.strerror)(err) };
        match text.is_null() {
            true => format!("error {err}"),
            false => unsafe { CStr::from_ptr(text) }
                .to_string_lossy()
                .into_owned(),
        }
    }

    fn open_pcm(&self, rate: u32) -> Result<*mut c_void, AudioError> {
        let mut last = AudioError::NoDevice;
        // A board-specific PCM can bypass a stale default/dmix/Pulse configuration.
        let override_name = std::env::var("SLOT_ALSA_DEVICE").ok();
        for name in pcm_candidates(override_name.as_deref()) {
            let Ok(cname) = CString::new(name.as_str()) else {
                continue;
            };
            let mut pcm: *mut c_void = std::ptr::null_mut();
            let err = unsafe { (self.open)(&mut pcm, cname.as_ptr(), SND_PCM_STREAM_PLAYBACK, 0) };
            if err < 0 || pcm.is_null() {
                last = AudioError::Device(format!("{name}: {}", self.message(err)));
                continue;
            }
            let err = unsafe {
                (self.set_params)(
                    pcm,
                    SND_PCM_FORMAT_S16_LE,
                    SND_PCM_ACCESS_RW_INTERLEAVED,
                    CHANNELS,
                    rate,
                    1,
                    LATENCY_US,
                )
            };
            if err < 0 {
                unsafe { (self.close)(pcm) };
                last = AudioError::Config(format!("{name}: {}", self.message(err)));
                continue;
            }
            eprintln!("slot: audio {name} at {rate} Hz");
            return Ok(pcm);
        }
        Err(last)
    }
}

pub struct AlsaSink {
    ring: Arc<Ring>,
    device: Option<Device>,
}

struct Device {
    stop: Arc<AtomicBool>,
    join: JoinHandle<()>,
}

impl AlsaSink {
    pub fn new() -> Self {
        AlsaSink {
            ring: Arc::new(Ring::new(0)),
            device: None,
        }
    }

    fn close(&mut self) {
        if let Some(d) = self.device.take() {
            d.stop.store(true, Ordering::Relaxed);
            let _ = d.join.join();
        }
    }
}

impl Default for AlsaSink {
    fn default() -> Self {
        AlsaSink::new()
    }
}

impl Drop for AlsaSink {
    fn drop(&mut self) {
        self.close();
    }
}

impl AudioSink for AlsaSink {
    fn open(&mut self, sample_rate: u32) -> Result<(), AudioError> {
        self.close();
        let sample_rate = output_rate(
            sample_rate,
            std::env::var("SLOT_DEVICE").as_deref() == Ok("g350"),
        );
        let ring = self.ring.clone();
        let stop = Arc::new(AtomicBool::new(false));
        let flag = stop.clone();
        let (ready_tx, ready_rx) = mpsc::channel();
        let join = std::thread::Builder::new()
            .name("slot-audio".into())
            .spawn(move || match play(&ring, sample_rate) {
                Ok(device) => {
                    let _ = ready_tx.send(Ok(()));
                    keep_playing(device, &ring, &flag, sample_rate);
                }
                Err(e) => {
                    let _ = ready_tx.send(Err(e));
                }
            })
            .map_err(|e| AudioError::Device(e.to_string()))?;
        match ready_rx.recv() {
            Ok(Ok(())) => {
                self.device = Some(Device { stop, join });
                Ok(())
            }
            Ok(Err(e)) => {
                let _ = join.join();
                Err(e)
            }
            Err(_) => {
                let _ = join.join();
                Err(AudioError::Device("output thread stopped".into()))
            }
        }
    }

    fn ring(&self) -> Arc<Ring> {
        self.ring.clone()
    }
}

struct Playback {
    alsa: Alsa,
    pcm: *mut c_void,
}

fn play(ring: &Arc<Ring>, sample_rate: u32) -> Result<Playback, AudioError> {
    let alsa = Alsa::load()?;
    let pcm = alsa.open_pcm(sample_rate)?;
    ring.reopen(sample_rate);
    Ok(Playback { alsa, pcm })
}

// The audio consumer must not silently disappear after an unrecoverable xrun.
fn keep_playing(mut device: Playback, ring: &Arc<Ring>, stop: &AtomicBool, rate: u32) {
    loop {
        device.run(ring, stop);
        drop(device);
        if stop.load(Ordering::Relaxed) {
            return;
        }
        eprintln!("slot: audio output failed; retrying PCM");
        loop {
            // Bounded backoff allows shutdown to join promptly even if the codec vanished.
            for _ in 0..10 {
                if stop.load(Ordering::Relaxed) {
                    return;
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            match play(ring, rate) {
                Ok(reopened) => {
                    device = reopened;
                    break;
                }
                Err(e) => eprintln!("slot: audio reopen: {e}"),
            }
        }
    }
}

fn output_rate(requested: u32, g350: bool) -> u32 {
    if g350 {
        48_000
    } else {
        requested
    }
}

impl Playback {
    fn run(&self, ring: &Ring, stop: &AtomicBool) {
        let mut buf = vec![0i16; PERIOD_FRAMES * CHANNELS as usize];
        while !stop.load(Ordering::Relaxed) {
            ring.fill(&mut buf);
            let mut written = 0;
            while written < PERIOD_FRAMES {
                let at = written * CHANNELS as usize;
                let frames = unsafe {
                    (self.alsa.writei)(
                        self.pcm,
                        buf[at..].as_ptr() as *const c_void,
                        (PERIOD_FRAMES - written) as u64,
                    )
                };
                if frames < 0 {
                    let err = unsafe { (self.alsa.recover)(self.pcm, frames as c_int, 1) };
                    if err < 0 {
                        eprintln!("slot: audio: {}", self.alsa.message(err));
                        return;
                    }
                    continue;
                }
                if frames == 0 {
                    eprintln!("slot: audio: PCM made no progress");
                    return;
                }
                written += frames as usize;
            }
        }
    }
}

impl Drop for Playback {
    fn drop(&mut self) {
        unsafe {
            (self.alsa.drop)(self.pcm);
            (self.alsa.close)(self.pcm);
        }
    }
}

fn pcm_candidates(preferred: Option<&str>) -> Vec<String> {
    let mut names = Vec::new();
    if let Some(name) = preferred.filter(|s| !s.trim().is_empty() && !s.contains('\0')) {
        names.push(name.trim().to_owned());
    }
    for name in DEVICES {
        if !names.iter().any(|n| n == name) {
            names.push(name.to_owned());
        }
    }
    names
}

#[cfg(test)]
mod tests {
    use super::{output_rate, pcm_candidates};
    #[test]
    fn g350_uses_standard_hardware_rate_without_changing_other_devices() {
        assert_eq!(output_rate(32768, true), 48000);
        assert_eq!(output_rate(44100, true), 48000);
        assert_eq!(output_rate(32768, false), 32768);
    }
    #[test]
    fn board_pcm_precedes_defaults_and_is_not_duplicated() {
        assert_eq!(
            pcm_candidates(Some("plughw:0,0")),
            ["plughw:0,0", "plug:default", "default", "hw:0,0"]
        );
        assert_eq!(pcm_candidates(Some("slot_test"))[0], "slot_test");
        assert_eq!(pcm_candidates(Some("  ")), pcm_candidates(None));
        assert_eq!(pcm_candidates(Some("bad\0name")), pcm_candidates(None));
    }
}
