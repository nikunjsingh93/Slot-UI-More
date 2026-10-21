#![cfg(all(target_os = "linux", feature = "test-support"))]
use slot_gfx::{Compositor, GameShader, HeadlessSurface, OUT_W, SRC_H, SRC_W};

fn pixel(frame: &[u8], x: usize, y: usize) -> [u8; 3] {
    let at = (y * OUT_W as usize + x) * 4;
    [frame[at], frame[at + 1], frame[at + 2]]
}

#[test]
fn all_presets_render_in_gles_and_switch_without_leaking_filter_state() {
    // Deliberately fail if EGL or shader compilation is unavailable; no skipped GPU check.
    let surface = HeadlessSurface::new().unwrap();
    let mut c = Compositor::new(&surface).unwrap();
    let flat = vec![128; (SRC_W * SRC_H * 4) as usize];
    c.upload_game(&flat);
    let render = |c: &mut Compositor, mode| {
        c.set_game_shader(mode);
        c.begin_frame();
        c.draw_game();
        c.read_frame()
    };
    let original = render(&mut c, GameShader::Original);
    let lcd = render(&mut c, GameShader::Lcd);
    assert_eq!(original, lcd, "original GBA LCD appearance changed");
    let sharp = render(&mut c, GameShader::Sharp);
    assert!(sharp.chunks_exact(4).all(|p| p[..3] == [128; 3]));
    let crt = render(&mut c, GameShader::Crt);
    assert_ne!(crt, lcd);
    assert_ne!(pixel(&crt, 360, 240), pixel(&crt, 360, 241));
    assert!(pixel(&crt, 360, 240)[0] > pixel(&crt, 0, 0)[0]);
    let mut checker = vec![0; (SRC_W * SRC_H * 4) as usize];
    for y in 0..SRC_H as usize {
        for x in 0..SRC_W as usize {
            let value = if (x + y) % 2 == 0 { 0 } else { 255 };
            checker[(y * SRC_W as usize + x) * 4..][..4].fill(value);
        }
    }
    c.upload_game(&checker);
    let sharp_before = render(&mut c, GameShader::Sharp);
    assert!(sharp_before.chunks_exact(4).all(|p| p[0] == 0 || p[0] == 255));
    let smooth = render(&mut c, GameShader::Smooth);
    assert!(smooth.chunks_exact(4).any(|p| p[0] > 20 && p[0] < 235));
    assert_eq!(render(&mut c, GameShader::Sharp), sharp_before);
    // Old console default stays unfiltered; forced LCD/CRT also work on console frames.
    for (w, h) in [(640, 480), (320, 240), (480, 272)] {
        c.upload_game(&vec![128; w * h * 4]);
        assert_eq!(render(&mut c, GameShader::Original), render(&mut c, GameShader::Sharp));
        for mode in [GameShader::Smooth, GameShader::Lcd, GameShader::Crt] {
            let frame = render(&mut c, mode);
            assert!(frame.chunks_exact(4).any(|p| p[0] != 0));
        }
    }
    assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);

    // A cropped GB picture at full height: 600 logical columns become 533/534
    // physical columns on the 640x480 SP, preserving the 160:144 aspect ratio.
    c.upload_game(&flat);
    c.set_game_source_rect([40.0 / 240.0, 8.0 / 160.0, 160.0 / 240.0, 144.0 / 160.0]);
    c.set_game_width(5.0 / 6.0);
    let fitted = render(&mut c, GameShader::Sharp);
    for y in [0, 240, 479] {
        for x in 0..720 {
            assert_eq!(pixel(&fitted, x, y), if (60..660).contains(&x) { [128; 3] } else { [0; 3] });
        }
    }
    c.set_fill_panel(true);
    c.end_frame((640, 480));
    let mut panel = vec![0u8; 640 * 480 * 4];
    unsafe {
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        gl::ReadPixels(0, 0, 640, 480, gl::RGBA, gl::UNSIGNED_BYTE, panel.as_mut_ptr() as *mut _);
    }
    for y in [0, 240, 479] {
        let row = &panel[y * 640 * 4..(y + 1) * 640 * 4];
        let lit: Vec<_> = row.chunks_exact(4).enumerate().filter(|(_, p)| p[0] > 0).map(|(x, _)| x).collect();
        assert!((533..=534).contains(&lit.len()), "incorrect physical GB width: {}", lit.len());
        let left = lit[0];
        let right = 639 - lit[lit.len() - 1];
        assert!(left.abs_diff(right) <= 1, "picture is not centered");
    }
    // The first L1 mode fills the entire physical panel.
    c.set_game_width(1.0);
    let stretched = render(&mut c, GameShader::Sharp);
    assert!(stretched.chunks_exact(4).all(|p| p[..3] == [128; 3]));
    // R1 restores the existing presentation rectangle and padded source.
    c.set_fill_panel(false);
    c.set_game_source_rect([0.0, 0.0, 1.0, 1.0]);
    render(&mut c, GameShader::Original);
    c.end_frame((640, 480));
    let mut viewport = [0; 4];
    unsafe { gl::GetIntegerv(gl::VIEWPORT, viewport.as_mut_ptr()); }
    assert_eq!(viewport, [0, 26, 640, 427]);
}
