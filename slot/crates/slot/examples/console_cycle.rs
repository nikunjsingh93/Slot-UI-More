//! Repeated real-core launches in one process, including PSP between DC sessions.
use slot_gfx::{HeadlessSurface, Surface};
use slot_retro::{ButtonMask, LibretroCore, RetroCore};
use slot_store::Core;
use std::{path::Path, time::Duration};

fn main() {
    std::env::set_var("SLOT_TEST_GLES", "1");
    let mut surface = HeadlessSurface::new().unwrap();
    let args: Vec<_> = std::env::args().collect();
    let sequence = if std::env::var_os("DC_ONLY").is_some() {
        vec![Core::Flycast; 4]
    } else {
        vec![Core::Flycast, Core::Ppsspp, Core::Flycast, Core::Flycast, Core::Ppsspp, Core::Flycast]
    };
    let mut dc_state: Option<Vec<u8>> = None;
    for (session, which) in sequence.into_iter().enumerate() {
        surface.make_current().unwrap();
        let dc = which == Core::Flycast;
        let (library, game) = if dc { (&args[1], &args[2]) } else { (&args[3], &args[4]) };
        let mut core = LibretroCore::open_with(Path::new(library), Path::new("/tmp/g350-v019-bios"), Path::new(if dc { "/tmp/g350-v019-saves/DC" } else { "/tmp/g350-v019-saves/PSP/MemoryStick" })).unwrap();
        slot::core::apply_core_options(&mut core, which, "", false, false);
        let game = game.clone();
        let resume = if dc && session % 2 == 0 { dc_state.clone() } else { None };
        println!("SESSION {session} {which:?} beginning; resume={}", resume.is_some());
        let state = std::thread::spawn(move || {
            core.load(Path::new(&game)).unwrap();
            if let Some(state) = resume { core.unserialize(&state).unwrap(); }
            let mut audible = 0;
            for frame in 0..360 {
                if !dc { std::thread::sleep(Duration::from_millis(16)); }
                let mask = if frame > 120 && frame % 90 == 0 { ButtonMask::START } else if frame > 120 && frame % 90 == 45 { ButtonMask::B } else { 0 };
                core.run_frame(ButtonMask(mask));
                audible += core.take_audio().iter().filter(|s| **s != 0).count();
            }
            let video = core.video_xrgb8888();
            assert!(video.chunks_exact(4).filter(|p| p[0] != 0 || p[1] != 0 || p[2] != 0).count() > 1000);
            let state = core.serialize().unwrap();
            core.unserialize(&state).unwrap();
            println!("PASS session {session} {which:?}: video, state round trip, {audible} audible samples, clean context teardown");
            state
        }).join().unwrap();
        if dc { dc_state = Some(state); }
    }
    println!("PASS repeated console launch and state resume without process restart");
}
