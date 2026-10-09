use slot_retro::{ButtonMask, LibretroCore, RetroCore};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let core_path = Path::new(&args[1]);
    let rom = Path::new(&args[2]);
    let dir = std::env::temp_dir().join("slot-g350-snes-smoke");
    std::fs::create_dir_all(&dir).unwrap();
    let mut core = LibretroCore::open_with(core_path, &dir, &dir).unwrap();
    core.load(rom).unwrap();
    assert!(!core.gba_turbo());
    assert!(!core.supports_rewind());
    for _ in 0..120 {
        core.run_frame(ButtonMask::default());
    }
    assert_eq!(core.video_xrgb8888().len(), 640 * 480 * 4);
    assert!(core
        .video_xrgb8888()
        .chunks_exact(4)
        .any(|p| p[2] > 200 && p[0] < 20));
    assert!(core.av_info().fps > 59.0 && core.av_info().fps < 61.0);
    let state = core.serialize().unwrap();
    core.unserialize(&state).unwrap();
    let mut ram = core.save_ram().unwrap();
    assert_eq!(ram[0], 0x5a);
    let last = ram.len() - 1;
    ram[last] = 0xc7;
    core.load_save_ram(&ram).unwrap();
    drop(core);
    let mut reloaded = LibretroCore::open_with(core_path, &dir, &dir).unwrap();
    reloaded.load(rom).unwrap();
    reloaded.load_save_ram(&ram).unwrap();
    assert_eq!(reloaded.save_ram().unwrap()[last], 0xc7);
    println!("PASS: SNES complete 4:3 video, no GBA turbo/rewind, state roundtrip and {} bytes SRAM reload",ram.len());
}
