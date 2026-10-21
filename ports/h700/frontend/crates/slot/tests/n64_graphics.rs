use slot_gfx::{HeadlessSurface, Surface};
use slot_retro::{ButtonMask, LibretroCore, RetroCore};
#[test]
fn n64_gles_frame_is_read_back_upright_and_ui_context_survives() {
    let Some(core) = std::env::var_os("SLOT_N64_FIXTURE") else {
        return;
    };
    std::env::set_var("SLOT_TEST_GLES", "1");
    let mut display = HeadlessSurface::new().unwrap();
    for (width, height) in [(320, 240), (256, 192)] {
        let mut core = LibretroCore::open(std::path::Path::new(&core)).unwrap();
        core.set_option("mupen64plus-43screensize", &format!("{width}x{height}"));
        let rom = tempfile::NamedTempFile::new().unwrap();
        let (core, video) = std::thread::spawn(move || {
            core.load(rom.path()).unwrap();
            core.run_frame(ButtonMask(0));
            let frame = core.video_xrgb8888().to_vec();
            assert_eq!(frame.len(), width * height * 4);
            assert_eq!(&frame[0..3], &[0, 0, 255]);
            assert_eq!(
                &frame[((height - 1) * width * 4)..((height - 1) * width * 4 + 3)],
                &[255, 0, 0]
            );
            assert!(!core.take_audio().is_empty());
            core.set_frame_skip(true);
            core.run_frame(ButtonMask(ButtonMask::START));
            assert_eq!(
                core.video_xrgb8888(),
                frame.as_slice(),
                "first low-audio readback should be skipped"
            );
            assert!(
                !core.take_audio().is_empty(),
                "skipping readback must not skip PCM"
            );
            core.run_frame(ButtonMask(ButtonMask::START));
            assert_eq!(
                &core.video_xrgb8888()[..3],
                &[0, 255, 0],
                "second readback must update even while PCM stays low"
            );
            assert!(!core.take_audio().is_empty());
            core.set_frame_skip(false);
            core.run_frame(ButtonMask(0));
            assert_eq!(&core.video_xrgb8888()[..3], &[0, 0, 255]);
            drop(core);
            ((), frame)
        })
        .join()
        .unwrap();
        let _ = (core, video);
        display.make_current().unwrap();
    }
}
