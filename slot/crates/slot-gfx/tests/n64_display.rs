#[cfg(all(target_os = "linux", feature = "test-support"))]
#[test]
fn native_n64_upload_is_complete_without_gba_mask_and_can_switch_back() {
    use slot_gfx::{Compositor, Draw, HeadlessSurface, OUT_W};
    let surface = HeadlessSurface::new().expect("surfaceless EGL required for PS1 validation");
    let mut c = Compositor::new(&surface).unwrap();
    let mut frame = vec![0u8; 320 * 240 * 4];
    for (i, p) in frame.chunks_exact_mut(4).enumerate() {
        p.copy_from_slice(if i % 320 < 160 {
            &[30, 80, 180, 0]
        } else {
            &[160, 100, 20, 0]
        });
    }
    c.upload_game(&frame);
    c.begin_frame();
    c.draw_list(&[Draw::Game]);
    let image = c.read_frame();
    let px = |x: usize, y: usize| {
        let o = (y * OUT_W as usize + x) * 4;
        image[o..o + 3].to_vec()
    };
    assert_eq!(px(1, 1), vec![180, 80, 30]);
    assert_eq!(px(718, 478), vec![20, 100, 160]);
    assert_eq!(
        px(101, 101),
        px(102, 101),
        "N64 image has an unintended GBA LCD mask"
    );
    c.set_ps1_display(true);
    c.end_frame((640, 480));
    assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);
    c.upload_game(&vec![0u8; 240 * 160 * 4]);
    c.begin_frame();
    c.draw_list(&[Draw::Game]);
    assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);
}
