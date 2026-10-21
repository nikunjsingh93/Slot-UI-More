use slot_gfx::{Compositor, Draw, HeadlessSurface, OUT_H, OUT_W};
use slot_store::{Cart, Platform};

fn main() {
    let surface = HeadlessSurface::new().unwrap();
    let mut compositor = Compositor::new(&surface).unwrap();
    let mut joined = vec![0u8; (OUT_W * 2 * OUT_H * 4) as usize];
    for (index, platform, title) in [
        (0, Platform::Ps1, "PlayStation Disc"),
        (1, Platform::Snes, "Super Nintendo"),
    ] {
        let cart = Cart {
            platform,
            stem: title.into(),
            title: String::new(),
            rom: "preview".into(),
            label: None,
            code: String::new(),
            shell: None,
        };
        let face = slot_ui::cart_face(&cart);
        let texture = compositor.create_texture(face.w, face.h, &face.rgba);
        let mut shelf = slot_ui::Shelf::new(vec![cart]);
        shelf.set_faces(vec![texture]);
        let mut draws = vec![Draw::Rect {
            x: 0.0,
            y: 0.0,
            w: OUT_W as f32,
            h: OUT_H as f32,
            colour: [0.84, 0.82, 0.79, 1.0],
        }];
        shelf.draw(0.0, &mut draws);
        compositor.begin_frame();
        compositor.draw_list(&draws);
        let pixels = compositor.read_frame();
        for y in 0..OUT_H as usize {
            let a = y * OUT_W as usize * 4;
            let b = (y * OUT_W as usize * 2 + index * OUT_W as usize) * 4;
            joined[b..b + OUT_W as usize * 4].copy_from_slice(&pixels[a..a + OUT_W as usize * 4]);
        }
    }
    let out = std::fs::File::create("/outputs/G350-PS1-SNES-preview.png").unwrap();
    let mut enc = png::Encoder::new(out, OUT_W * 2, OUT_H);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header()
        .unwrap()
        .write_image_data(&joined)
        .unwrap();
}
