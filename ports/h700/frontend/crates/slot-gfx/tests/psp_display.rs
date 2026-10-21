#[cfg(all(target_os = "linux", feature = "test-support"))]
#[test]
fn psp_native_frame_keeps_widescreen_on_the_four_three_panel() {
    use slot_gfx::{Compositor, Draw, HeadlessSurface, OUT_W};
    let surface = HeadlessSurface::new().unwrap();
    let mut c = Compositor::new(&surface).unwrap();
    for height in [270, 272] {
        let mut frame = vec![0u8; 480 * height * 4];
        for p in frame.chunks_exact_mut(4) {
            p.copy_from_slice(&[40, 80, 180, 0]);
        }
        c.upload_game(&frame);
        c.begin_frame();
        c.draw_list(&[Draw::Game]);
        let image = c.read_frame();
        let px = |x: usize, y: usize| {
            let o = (y * OUT_W as usize + x) * 4;
            image[o..o + 3].to_vec()
        };
        assert_eq!(px(360, 240), vec![180, 80, 40]);
        assert_eq!(px(360, 10), vec![0, 0, 0]);
        assert_eq!(px(360, 470), vec![0, 0, 0]);
        assert_eq!(px(718, 240), vec![180, 80, 40]);
    }
}
