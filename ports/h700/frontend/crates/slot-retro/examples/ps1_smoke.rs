use slot_retro::{ButtonMask, LibretroCore, RetroCore};
use std::path::Path;
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let dir = Path::new("/work/ps1-smoke-saves");
    std::fs::create_dir_all(dir).unwrap();
    let mut core = LibretroCore::open_with(Path::new(&args[1]), dir, dir).unwrap();
    core.set_option("pcsx_rearmed_memcard1", "libretro");
    core.set_option("pcsx_rearmed_memcard2", "none");
    core.load(Path::new(&args[2])).unwrap();
    if args[2].ends_with(".m3u") {
        core.next_disc().unwrap();
        assert_eq!(
            std::fs::read_to_string(dir.join("Smoke.disc")).unwrap(),
            "1"
        );
        core.next_disc().unwrap();
        assert_eq!(
            std::fs::read_to_string(dir.join("Smoke.disc")).unwrap(),
            "0"
        );
        core.next_disc().unwrap();
    }
    assert!(!core.gba_turbo());
    for _ in 0..180 {
        core.run_frame(ButtonMask(0));
        core.take_audio();
    }
    let video = core.video_xrgb8888();
    assert_eq!(video.len(), 640 * 480 * 4);
    assert!(
        video
            .chunks_exact(4)
            .any(|p| p[0] != 0 || p[1] != 0 || p[2] != 0),
        "GPU produced only black frames"
    );
    let state = core.serialize().unwrap();
    if args[2].ends_with(".m3u") {
        assert_eq!(&state[..8], b"SLPS1DSC");
        core.next_disc().unwrap();
        assert_eq!(
            std::fs::read_to_string(dir.join("Smoke.disc")).unwrap(),
            "0"
        );
    }
    core.unserialize(&state).unwrap();
    if args[2].ends_with(".m3u") {
        assert_eq!(
            std::fs::read_to_string(dir.join("Smoke.disc")).unwrap(),
            "1"
        );
    }
    let mut card = core.save_ram().unwrap();
    assert_eq!(card.len(), 128 * 1024);
    card[8192] = 0x5a;
    core.load_save_ram(&card).unwrap();
    assert_eq!(core.save_ram().unwrap()[8192], 0x5a);
    std::fs::write(dir.join("smoke.sav"), &card).unwrap();
    drop(core);
    let mut core = LibretroCore::open_with(Path::new(&args[1]), dir, dir).unwrap();
    core.set_option("pcsx_rearmed_memcard1", "libretro");
    core.set_option("pcsx_rearmed_memcard2", "none");
    core.load(Path::new(&args[2])).unwrap();
    core.load_save_ram(&std::fs::read(dir.join("smoke.sav")).unwrap())
        .unwrap();
    assert_eq!(core.save_ram().unwrap()[8192], 0x5a);
    println!("PASS: real ARM64 PCSX core, nonblack 640x480 frames, state round-trip, 128KiB memory card persisted across reload; state {} bytes",state.len());
}
