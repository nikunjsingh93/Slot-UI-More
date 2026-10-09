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
    let face = slot_ui::cart_face(&cart);
    let out = std::fs::File::create("/work/disc-preview.png").unwrap();
    let mut enc = png::Encoder::new(out, face.w, face.h);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header()
        .unwrap()
        .write_image_data(&face.rgba)
        .unwrap();
}
