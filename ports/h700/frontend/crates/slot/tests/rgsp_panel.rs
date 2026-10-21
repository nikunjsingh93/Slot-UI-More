#![cfg(target_os = "linux")]
use slot::frontend::Frontend;
use slot::video_mode::{width_fraction, VideoMode};
use slot_gfx::{Compositor, HeadlessSurface, GameShader};
use slot_power::SimPlatform;
use slot_store::Platform;

#[test]
fn rgsp_native_panel_and_game_aspects_are_preserved() {
    std::env::set_var("SLOT_DEVICE", "rgsp");
    let d = tempfile::tempdir().unwrap();
    slot::root::ensure(d.path());
    for stem in ["First", "Second"] {
        std::fs::write(d.path().join(format!("Games/PS1/{stem}.chd")), [0; 32]).unwrap();
    }
    slot_store::write_slot_state(d.path(), &slot_store::SlotState { clock_set: true, ..Default::default() }).unwrap();
    let surface = HeadlessSurface::new().unwrap();
    let mut c = Compositor::new(&surface).unwrap();
    let mut ui = Frontend::boot(Box::new(SimPlatform::at(d.path().to_path_buf())));
    ui.upload_faces(&mut c);
    ui.render(&mut c, (720, 480));
    let mut viewport = [0i32; 4];
    unsafe { gl::GetIntegerv(gl::VIEWPORT, viewport.as_mut_ptr()); }
    assert_eq!(viewport, [0, 0, 720, 480]);
    // The shelf's power-on animation starts with its game rectangle closed.
    // Use a fresh compositor to measure the fully powered game presentation.
    drop(c);
    let mut c = Compositor::new(&surface).unwrap();
    c.set_game_shader(GameShader::Sharp);
    c.set_fill_panel(true);
    let cases = [
        (Platform::Gb, VideoMode::HeightFit, 240, 160, 533, 480),
        (Platform::Gbc, VideoMode::Stretch, 240, 160, 720, 480),
        (Platform::Ps1, VideoMode::Actual, 640, 480, 640, 480),
        (Platform::Snes, VideoMode::Actual, 320, 240, 640, 480),
        (Platform::N64, VideoMode::Actual, 320, 240, 640, 480),
        (Platform::Dreamcast, VideoMode::Actual, 320, 240, 640, 480),
        (Platform::Psp, VideoMode::Actual, 480, 272, 720, 408),
    ];
    for (p, mode, w, h, expected_w, expected_h) in cases {
        c.upload_game(&vec![128; w * h * 4]);
        c.set_game_width(width_fraction(p, mode));
        c.begin_frame(); c.draw_game();
        c.end_frame((720, 480));
        let mut final_viewport = [0i32; 4];
        unsafe { gl::GetIntegerv(gl::VIEWPORT, final_viewport.as_mut_ptr()); }
        assert_eq!(final_viewport, [0, 0, 720, 480]);
        let f = c.read_frame();
        let row = &f[240 * 720 * 4..241 * 720 * 4];
        let cols: Vec<_> = row.chunks_exact(4).enumerate().filter(|(_, px)| px[0] > 0).map(|(x, _)| x).collect();
        let rows: Vec<_> = (0..480).filter(|y| f[(y * 720 + 360) * 4] > 0).collect();
        assert!(cols.len().abs_diff(expected_w) <= 1, "{p:?}: width {}", cols.len());
        assert!(rows.len().abs_diff(expected_h) <= 1, "{p:?}: height {}", rows.len());
        assert!(cols[0].abs_diff(719 - cols[cols.len()-1]) <= 1);
        assert!(rows[0].abs_diff(479 - rows[rows.len()-1]) <= 1);
    }
    assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);
    // Full overrides letterboxing independently of the texture's system.
    c.set_game_stretch(true);
    c.set_game_width(1.0);
    for (w,h) in [(640,480),(320,240),(480,272),(240,160)] {
        c.upload_game(&vec![128;w*h*4]);
        c.begin_frame();c.draw_game();
        let f=c.read_frame();
        assert!(f.chunks_exact(4).all(|pixel| pixel[0]>0), "Full mode left a border for {w}x{h}");
    }
    std::env::remove_var("SLOT_DEVICE");
}
