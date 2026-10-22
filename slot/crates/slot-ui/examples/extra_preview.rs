//! Original fictional labels through the actual cartridge renderer; no game art.
use slot_store::{Cart,Platform};
fn main() {
 let (w,h)=(1140u32,840u32);let mut image=vec![0u8;(w*h*4) as usize];
 for px in image.chunks_exact_mut(4){px.copy_from_slice(&[23,25,30,255]);}
 for (i,p) in [Platform::Nes,Platform::MegaDrive,Platform::MasterSystem,Platform::GameGear,Platform::PcEngine,Platform::Arcade].into_iter().enumerate(){
  let c=Cart{platform:p,stem:format!("{} Demo",p.dir_name()),title:format!("{} Demo",p.dir_name()),rom:"unused".into(),label:None,code:String::new(),shell:None};
  let f=slot_ui::cart_face(&c);let ox=(i as u32%3)*380+(380-f.w)/2;let oy=(i as u32/3)*420+(420-f.h)/2;
  for y in 0..f.h {for x in 0..f.w {let src=((y*f.w+x)*4) as usize;let dest=(((y+oy)*w+x+ox)*4) as usize;let a=f.rgba[src+3] as u32;for k in 0..3{image[dest+k]=((f.rgba[src+k] as u32*a+image[dest+k] as u32*(255-a))/255) as u8;}}}
 }
 let out=std::fs::File::create(std::env::args().nth(1).expect("output PNG path")).unwrap();
 let mut encoder=png::Encoder::new(out,w,h);encoder.set_color(png::ColorType::Rgba);encoder.set_depth(png::BitDepth::Eight);encoder.write_header().unwrap().write_image_data(&image).unwrap();
}
