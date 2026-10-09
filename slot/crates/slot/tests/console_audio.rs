use slot::{
    audio::Ring,
    emu::{CoreState, EmuHandle, Speed},
};
use slot_retro::{AvInfo, ButtonMask, CoreError, RetroCore};
use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
struct Tone {
    rate: f64,
    fps: f64,
    phase: f64,
    audio: Vec<i16>,
    video: Vec<u8>,
}
impl RetroCore for Tone {
    fn gba_turbo(&self) -> bool {
        false
    }
    fn supports_rewind(&self) -> bool {
        false
    }
    fn load(&mut self, _: &Path) -> Result<(), CoreError> {
        Ok(())
    }
    fn run_frame(&mut self, _: ButtonMask) {
        self.phase += self.rate / self.fps;
        let n = self.phase.floor() as usize;
        self.phase -= n as f64;
        self.audio = vec![12000; n * 2];
    }
    fn video_xrgb8888(&self) -> &[u8] {
        &self.video
    }
    fn take_audio(&mut self) -> Vec<i16> {
        std::mem::take(&mut self.audio)
    }
    fn serialize(&mut self) -> Result<Vec<u8>, CoreError> {
        Ok(vec![])
    }
    fn unserialize(&mut self, _: &[u8]) -> Result<(), CoreError> {
        Ok(())
    }
    fn save_ram(&self) -> Option<Vec<u8>> {
        None
    }
    fn load_save_ram(&mut self, _: &[u8]) -> Result<(), CoreError> {
        Ok(())
    }
    fn av_info(&self) -> AvInfo {
        AvInfo {
            fps: self.fps,
            sample_rate: self.rate,
        }
    }
}
#[test]
fn snes_and_ps1_pcm_remain_continuous_when_display_is_slow_or_stalls() {
    for (rate, fps) in [(32040.5, 60.0988), (44100.0, 59.94), (44100.0, 50.0)] {
        let ring = Arc::new(Ring::new(0));
        ring.reopen(48000);
        let emu = EmuHandle::spawn(
            Box::new(Tone {
                rate,
                fps,
                phase: 0.0,
                audio: vec![],
                video: vec![0; 240 * 160 * 4],
            }),
            PathBuf::from("fixture"),
            ring.clone(),
            None,
            None,
        );
        let began = Instant::now();
        while emu.state() == CoreState::Loading {
            assert!(began.elapsed() < Duration::from_secs(2));
            std::thread::sleep(Duration::from_millis(1));
        }
        emu.set_driven(true);
        emu.set_speed(Speed::Normal);
        let r = ring.clone();
        let stop = Arc::new(AtomicBool::new(false));
        let flag = stop.clone();
        let consumer = std::thread::spawn(move || {
            let began = Instant::now();
            let mut captured = vec![];
            let mut out = vec![0; 960];
            let mut n = 0;
            while !flag.load(Ordering::Relaxed) {
                r.fill(&mut out);
                captured.extend_from_slice(&out);
                n += 1;
                if let Some(wait) =
                    (began + Duration::from_millis(n * 10)).checked_duration_since(Instant::now())
                {
                    std::thread::sleep(wait);
                }
            }
            captured
        });
        // Only 50-55 display latches/sec, with a 75ms stall every half second.
        for i in 0..100 {
            emu.tick(Duration::from_millis(18));
            let _ = emu.latest_frame();
            std::thread::sleep(Duration::from_millis(if i % 25 == 24 { 75 } else { 18 }));
        }
        stop.store(true, Ordering::Relaxed);
        let captured = consumer.join().unwrap();
        emu.set_speed(Speed::Paused);
        let starved = ring.underruns();
        drop(emu);
        let stable = &captured[48000.min(captured.len())..];
        let quiet = stable.iter().filter(|s| **s == 0).count();
        assert_eq!(starved, 0, "{rate}Hz/{fps}fps starved {starved} samples");
        assert_eq!(
            quiet, 0,
            "{rate}Hz/{fps}fps inserted {quiet} silent samples"
        );
        println!(
            "PASS {rate}Hz/{fps}fps: {} captured frames, no silent gaps despite stalled display",
            captured.len() / 2
        );
    }
}
