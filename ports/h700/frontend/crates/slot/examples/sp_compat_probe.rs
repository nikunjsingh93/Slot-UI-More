//! Real-core smoke test using a caller-supplied local game and temporary saves.
//! This is a compatibility check; host timings are NOT G350 benchmarks.
use slot_gfx::{HeadlessSurface, Surface};
use slot_retro::{ButtonMask, LibretroCore, RetroCore};
use slot_store::Core;
use std::{path::Path, time::Instant};

fn main() {
    std::env::set_var("SLOT_TEST_GLES", "1");
    let args: Vec<_> = std::env::args().collect();
    let mut surface = HeadlessSurface::new().unwrap();
    let temp = tempfile::tempdir().unwrap();
    let frames = args.get(3).map(|v| v.parse::<usize>().unwrap()).unwrap_or(300);
    let dc = args[1].contains("flycast");
    for session in 0..2 {
        surface.make_current().unwrap();
        let mut core = LibretroCore::open_with(Path::new(&args[1]), temp.path(), temp.path()).unwrap();
        let which = if dc { Core::Flycast } else { Core::ParallelN64 };
        slot::core::apply_core_options(&mut core, which, "", false, false);
        for key in ["parallel-n64-gfxplugin-accuracy", "parallel-n64-gfxplugin", "reicast_internal_resolution", "reicast_threaded_rendering", "reicast_texupscale"] {
            if let Some(value) = core.option(key) { println!("OPTION {key}={value}"); }
        }
        let game = args[2].clone();
        let transfer = args.get(4).cloned();
        std::thread::spawn(move || {
            core.load(Path::new(&game)).unwrap();
            if let Some(path) = transfer.as_deref().and_then(|s| s.strip_prefix("state-reject=")) {
                assert!(core.unserialize(&std::fs::read(path).unwrap()).is_err());
                println!("PASS rejected incompatible state without calling unsafe core loader");
                return;
            }
            if let Some(path) = transfer.as_deref().and_then(|s| s.strip_prefix("state-in=")) {
                core.unserialize(&std::fs::read(path).unwrap()).unwrap();
                println!("PASS cold state resume from caller-supplied state");
            }
            #[cfg(all(feature = "core-diagnostics", target_arch = "aarch64"))]
            assert!(core.check_frame_fp_abi(), "Core corrupted callee-saved floating-point registers");
            let began=Instant::now();
            let mut samples=0;
            for frame in 0..frames {
                core.set_frame_skip(frame % 2 == 0);
                core.run_frame(ButtonMask(if frame==120 || frame==240 {ButtonMask::START} else {0}));
                samples+=core.take_audio().len();
            }
            let video=core.video_xrgb8888();
            let lit=video.chunks_exact(4).filter(|p| p[..3]!=[0,0,0]).count();
            assert!(samples>0 && lit>100,"No game audio/video: samples={samples} lit={lit}");
            let state=core.serialize().unwrap();
            if session == 0 {
                if let Some(path) = transfer.as_deref().and_then(|s| s.strip_prefix("state-out=")) {
                    std::fs::write(path, &state).unwrap();
                }
            }
            core.unserialize(&state).unwrap();
            let mut resumed_audio = 0;
            for _ in 0..10 { core.run_frame(ButtonMask(0)); resumed_audio += core.take_audio().len(); }
            assert!(resumed_audio > 0, "No audio after state resume");
            println!("PASS session={session} frames={frames} samples={samples} lit={lit} state_bytes={} host_seconds={:.2}; state round trip and teardown",state.len(),began.elapsed().as_secs_f64());
        }).join().unwrap();
    }
    surface.make_current().unwrap();
}
