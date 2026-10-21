#[cfg(all(target_os = "linux", feature = "test-support"))]
#[test]
fn g350_ui_fills_native_panel_but_gba_retains_its_aspect() {
    use slot_gfx::{Compositor, Draw, HeadlessSurface};
    let surface = HeadlessSurface::new().expect("surfaceless EGL");
    let mut c = Compositor::new(&surface).unwrap();
    c.begin_frame();
    c.draw_list(&[Draw::Rect {
        x: 0.0,
        y: 0.0,
        w: 720.0,
        h: 480.0,
        colour: [1.0, 0.0, 0.0, 1.0],
    }]);
    c.set_fill_panel(true);
    c.end_frame((640, 480));
    let mut image = vec![0u8; 640 * 480 * 4];
    unsafe {
        gl::ReadPixels(
            0,
            0,
            640,
            480,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            image.as_mut_ptr().cast(),
        );
    }
    for (x, y) in [(0, 0), (639, 0), (0, 479), (639, 479), (320, 240)] {
        let o = (y * 640 + x) * 4;
        assert_eq!(&image[o..o + 3], &[255, 0, 0], "UI border at {x},{y}");
    }
    // A GBA canvas stays 3:2 and therefore legitimately has unused panel rows.
    c.set_fill_panel(false);
    c.end_frame((640, 480));
    unsafe {
        gl::ReadPixels(
            0,
            0,
            640,
            480,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            image.as_mut_ptr().cast(),
        );
    }
    assert_eq!(&image[..3], &[0, 0, 0]);
    let o = (240 * 640 + 320) * 4;
    assert_eq!(&image[o..o + 3], &[255, 0, 0]);
    assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);
}
