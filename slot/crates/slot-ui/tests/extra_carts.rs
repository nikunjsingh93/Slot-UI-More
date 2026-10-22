use slot_store::{Cart,Platform};
#[test]
fn new_shells_have_their_own_geometry_and_opaque_print() {
 for (p,w,h) in [(Platform::Nes,300,350),(Platform::MegaDrive,360,220),(Platform::MasterSystem,300,310),(Platform::GameGear,250,260),(Platform::PcEngine,300,190),(Platform::Arcade,360,260)] {
  let c=Cart{platform:p,stem:"Original test title".into(),title:"Original test title".into(),rom:"unused".into(),label:None,code:String::new(),shell:None};
  let face=slot_ui::cart_face(&c);assert_eq!((face.w,face.h),(w,h));assert_eq!(slot_ui::cart_box(p),(w,h));
  let (sw,sh)=slot_ui::seated_box(p);assert!(((sw as f32/sh as f32)-(w as f32/h as f32)).abs()<0.01);
  assert_eq!(face.rgba.len(),(w*h*4) as usize);
  assert!(face.rgba.chunks_exact(4).any(|p|p[3]==0));assert!(face.rgba.chunks_exact(4).filter(|p|p[3]==255).count()>(w*h/2) as usize);
 }
}
