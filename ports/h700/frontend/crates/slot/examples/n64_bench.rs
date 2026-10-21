use slot_gfx::{HeadlessSurface, Surface};
use slot_retro::{ButtonMask, LibretroCore, RetroCore};
use std::{path::Path, time::Instant};
fn main() {
    std::env::set_var("SLOT_TEST_GLES", "1");
    let mut surface = HeadlessSurface::new().unwrap();
    let args: Vec<_> = std::env::args().collect();
    let mut core = LibretroCore::open(Path::new(&args[1])).unwrap();
    let which = if args[1].contains("parallel_n64") {
        slot_store::Core::ParallelN64
    } else {
        slot_store::Core::Mupen64plus
    };
    slot::core::apply_core_options(&mut core, which, "", false, false);
    std::thread::spawn(move || {
  core.load(Path::new(&args[2])).unwrap();
  let mut ns=0u128;let mut samples=0;let mut drawn=0;
  let mut last=Vec::new();
  for i in 0..1200 {
   core.set_frame_skip(false);
   let mask=if i>120 && i%180==0 {ButtonMask::START} else if i>120 && i%180==90 {ButtonMask::A} else {0};
   let began=Instant::now(); core.run_frame(ButtonMask(mask));
   let audio=core.take_audio();ns+=began.elapsed().as_nanos();samples+=audio.len();
   let frame=core.video_xrgb8888();
   if frame!=last {drawn+=1;last=frame.to_vec();}
  }
  let (w,h)=if last.len()==256*192*4 {(256,192)} else if last.len()==320*240*4 {(320,240)} else {(640,480)};
  let mut png=png::Encoder::new(std::fs::File::create(&args[3]).unwrap(),w,h);
  png.set_color(png::ColorType::Rgb);png.set_depth(png::BitDepth::Eight);
  let rgb:Vec<u8>=last.chunks_exact(4).flat_map(|p|[p[2],p[1],p[0]]).collect();
  png.write_header().unwrap().write_image_data(&rgb).unwrap();
  let av=core.av_info();let lit=last.chunks_exact(4).filter(|p|p[0]!=0||p[1]!=0||p[2]!=0).count();
  assert!(lit>1000 && samples>500_000);
  let state=core.serialize().unwrap();core.unserialize(&state).unwrap();
  println!("BENCH fps={:.2} ms={:.3} samples={} seconds_of_audio={:.2} changed_frames={} bytes={} lit={} av_fps={} state={}",1200.0/(ns as f64/1e9),ns as f64/1200.0/1e6,samples,samples as f64/2.0/av.sample_rate,drawn,last.len(),lit,av.fps,state.len());
 }).join().unwrap();
    surface.make_current().unwrap();
}
