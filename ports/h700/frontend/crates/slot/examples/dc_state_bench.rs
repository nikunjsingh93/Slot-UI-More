//! Compare the same user's local Dreamcast save state; never bundled with ROMs.
use slot_gfx::{HeadlessSurface, Surface};
use slot_retro::{ButtonMask, LibretroCore, RetroCore};
use std::{path::Path, time::Instant};

fn main() {
    std::env::set_var("SLOT_TEST_GLES", "1");
    let mut surface = HeadlessSurface::new().unwrap();
    let args: Vec<_> = std::env::args().collect();
    let mut core = LibretroCore::open_with(Path::new(&args[1]), Path::new("/tmp/g350-v020-bios"), Path::new("/tmp/g350-v020-saves/DC")).unwrap();
    slot::core::apply_core_options(&mut core, slot_store::Core::Flycast, "", false, false);
    for option in args.iter().skip(5) {
        let (key, value) = option.split_once('=').unwrap();
        core.set_option(key, value);
    }
    let declared = core.declared_options();
    for (key, value) in core.options() {
        if key.starts_with("reicast_") {
            assert!(declared.get(&key).is_some_and(|values| values.contains(&value)), "Unsupported core option {key}={value}");
        }
    }
    std::thread::spawn(move || {
        core.load(Path::new(&args[2])).unwrap();
        let state = std::fs::read(&args[3]).unwrap();
        core.unserialize(&state).unwrap();
        // The user quit these sessions from the game's pause menu.
        for frame in 0..60 {
            core.run_frame(ButtonMask(if frame == 5 { ButtonMask::START } else if frame == 15 { ButtonMask::B } else { 0 }));
            let _ = core.take_audio();
        }
        let start = Instant::now();
        let mut samples = 0; let mut audible = 0; let mut changed = 0;
        let mut previous = Vec::new();
        for _ in 0..600 {
            core.run_frame(ButtonMask(0));
            let audio = core.take_audio();
            samples += audio.len();
            audible += audio.iter().filter(|s| **s != 0).count();
            let frame = core.video_xrgb8888();
            if frame != previous { changed += 1; previous = frame.to_vec(); }
        }
        let wall = start.elapsed().as_secs_f64();
        let size = core.video_xrgb8888().len();
        assert_eq!(size, 320 * 240 * 4);
        let lit = previous.chunks_exact(4).filter(|p| p[..3] != [0, 0, 0]).count();
        eprintln!("OUTPUT lit={lit} audible={audible} changed={changed} samples={samples} bytes={size}");
        let mut png = png::Encoder::new(std::fs::File::create(&args[4]).unwrap(), 320, 240);
        png.set_color(png::ColorType::Rgb); png.set_depth(png::BitDepth::Eight);
        let rgb: Vec<u8> = previous.chunks_exact(4).flat_map(|p| [p[2], p[1], p[0]]).collect();
        png.write_header().unwrap().write_image_data(&rgb).unwrap();
        assert!(lit > 1000 && audible > 1000 && changed > 5);
        let restored = core.serialize().unwrap(); core.unserialize(&restored).unwrap();
        println!("BENCH wall={wall:.3}s calls_per_sec={:.2} audio_seconds={:.3} audio_realtime_ratio={:.3} changed_frames={changed} audible={audible} lit={lit} state={} PASS", 600.0 / wall, samples as f64 / 88200.0, samples as f64 / 88200.0 / wall, restored.len());
    }).join().unwrap();
    surface.make_current().unwrap();
}
