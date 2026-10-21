use slot_gfx::{Compositor, Draw, HeadlessSurface, OUT_H, OUT_W};
use slot_store::{Cart, Platform};
fn main() {
    let cart = Cart {
        platform: Platform::Ps1,
        stem: "PlayStation Disc".into(),
        title: String::new(),
        rom: "preview.chd".into(),
        label: None,
        code: String::new(),
        shell: None,
    };
    let surface = HeadlessSurface::new().unwrap();
    let mut compositor = Compositor::new(&surface).unwrap();
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
    let out = std::fs::File::create("/work/ps1-shelf-preview.png").unwrap();
    let mut enc = png::Encoder::new(out, OUT_W, OUT_H);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header()
        .unwrap()
        .write_image_data(&pixels)
        .unwrap();
}
