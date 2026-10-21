use slot_store::{Cart, Platform};
fn main() {
    let args:Vec<_>=std::env::args().collect();
    let cart=Cart { platform:Platform::Psp,stem:"Tekken 6".into(),title:String::new(),
        rom:"preview.iso".into(),label:args.get(1).map(Into::into),code:String::new(),shell:None };
    let face=slot_ui::cart_face(&cart);
    assert_eq!((face.w,face.h),(312,328));
    let mut enc=png::Encoder::new(std::fs::File::create(&args[2]).unwrap(),face.w,face.h);
    enc.set_color(png::ColorType::Rgba);enc.set_depth(png::BitDepth::Eight);
    enc.write_header().unwrap().write_image_data(&face.rgba).unwrap();
}
