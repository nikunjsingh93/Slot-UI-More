#[cfg(target_os = "linux")]
#[test]
fn g350_console_and_native_port_discs_are_round_after_full_panel_blit() {
    use slot::frontend::correct_disc_aspect;
    use slot_gfx::{Compositor, Draw, HeadlessSurface};
    use slot_store::{Cart, Platform};
    let surface = HeadlessSurface::new().expect("surfaceless EGL");
    let mut c = Compositor::new(&surface).unwrap();
    for platform in [Platform::Ps1, Platform::Dreamcast, Platform::Ports] {
        let cart = Cart {
            platform,
            stem: "Disc test".into(),
            title: String::new(),
            rom: "test.chd".into(),
            label: None,
            code: String::new(),
            shell: None,
        };
        let face = slot_ui::cart_face(&cart);
        let tex = c.create_texture(face.w, face.h, &face.rgba);
        let other = c.create_texture(1, 1, &[0, 0, 0, 0]);
        // Includes full-size and side/animation-size draws; centres must stay put.
        for size in [312.0, 240.0, 160.0] {
            let mut draws = vec![
                Draw::Tex {
                    x: 360.0 - size / 2.0,
                    y: 240.0 - size / 2.0,
                    w: size,
                    h: size,
                    tex,
                    alpha: 1.0,
                },
                Draw::Tex {
                    x: 0.0,
                    y: 0.0,
                    w: 15.0,
                    h: 15.0,
                    tex: other,
                    alpha: 1.0,
                },
            ];
            let untouched = draws[1];
            correct_disc_aspect(&mut draws, &[tex], 720.0 / 640.0);
            assert_eq!(draws[1], untouched);
            c.set_fill_panel(true);
            c.begin_frame();
            c.draw_list(&draws);
            c.end_frame((640, 480));
            // Read the actual presentation framebuffer, not the 720x480 logical canvas.
            let mut pixels = vec![0u8; 640 * 480 * 4];
            unsafe {
                gl::ReadPixels(
                    0,
                    0,
                    640,
                    480,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    pixels.as_mut_ptr().cast(),
                );
            }
            let (mut left, mut right, mut top, mut bottom) = (640, 0, 480, 0);
            for y in 0..480 {
                for x in 0..640 {
                    let o = (y * 640 + x) * 4;
                    // Measure the opaque metallic rim, excluding a subpixel filter fringe.
                    if pixels[o..o + 3].iter().any(|v| *v > 127) {
                        left = left.min(x);
                        right = right.max(x);
                        top = top.min(y);
                        bottom = bottom.max(y);
                    }
                }
            }
            let width = right - left + 1;
            let height = bottom - top + 1;
            assert!(
                width.abs_diff(height) <= 1,
                "disc size {size}: {width}x{height}"
            );
            assert!(
                (left + right).abs_diff(639) <= 1,
                "disc shifted horizontally"
            );
            assert!((top + bottom).abs_diff(479) <= 1, "disc shifted vertically");
        }
    }
}
