//! Documentation-only captures from the real shelf renderer, with original demo labels.
//! EGL_PLATFORM=surfaceless LIBGL_ALWAYS_SOFTWARE=1 cargo run --locked -p slot-ui
//!   --example readme_gallery -- /path/to/docs/screenshots
use slot_gfx::{Compositor, Draw, HeadlessSurface, OUT_H, OUT_W};
use slot_store::{Cart, Platform};
use std::path::Path;

fn png(path: &Path, w: u32, h: u32, pixels: &[u8]) {
    let mut encoder = png::Encoder::new(std::fs::File::create(path).unwrap(), w, h);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder
        .write_header()
        .unwrap()
        .write_image_data(pixels)
        .unwrap();
}

fn demo_label(path: &Path, title: &str, hue: usize, disc: bool) {
    let palettes = [
        [24, 66, 98],
        [78, 39, 96],
        [21, 91, 79],
        [119, 58, 38],
        [39, 59, 112],
        [80, 87, 28],
        [109, 43, 65],
        [41, 76, 90],
        [64, 66, 78],
    ];
    let base = palettes[hue % palettes.len()];
    let mut pixels = vec![0u8; 512 * 512 * 4];
    for y in 0..512usize {
        for x in 0..512usize {
            let p = &mut pixels[(y * 512 + x) * 4..][..4];
            let mountain = y > 370 - ((x / 16 + hue * 3) % 20) * 5;
            let stars = (x * 17 + y * 31 + hue * 13) % 997 < 3;
            for c in 0..3 {
                p[c] = if stars {
                    225
                } else if mountain {
                    base[c] / 2
                } else {
                    base[c] + (y / 24) as u8
                };
            }
            p[3] = 255;
        }
    }
    let font = slot_ui::text::label_font().unwrap();
    let layout = slot_ui::text::fit_box(font, title, 430.0, 110.0, 2, 57.0, 25.0);
    let mut text = vec![0; 512 * 120 * 4];
    slot_ui::text::draw_centred(&mut text, 512, 120, &layout, [245, 237, 205]);
    let top = if disc { 105 } else { 190 };
    for y in 0..120usize {
        for x in 0..512usize {
            let a = (y * 512 + x) * 4;
            let b = ((y + top) * 512 + x) * 4;
            let alpha = text[a + 3] as u32;
            for c in 0..3 {
                pixels[b + c] = ((text[a + c] as u32 * alpha
                    + pixels[b + c] as u32 * (255 - alpha))
                    / 255) as u8;
            }
        }
    }
    png(path, 512, 512, &pixels);
}

fn main() {
    let output = std::env::args().nth(1).expect("output directory");
    let output = Path::new(&output);
    std::fs::create_dir_all(output).unwrap();
    let labels = std::env::temp_dir().join("slot-readme-demo-labels");
    std::fs::create_dir_all(&labels).unwrap();
    let surface = HeadlessSurface::new().unwrap();
    for (i, (system, platform, title)) in [
        ("gb", Platform::Gb, "LUNAR QUEST"),
        ("gbc", Platform::Gbc, "PRISM ISLAND"),
        ("gba", Platform::Gba, "SKY COURIER"),
        ("snes", Platform::Snes, "STAR VALLEY"),
        ("ps1", Platform::Ps1, "NIGHT CIRCUIT"),
        ("n64", Platform::N64, "SUMMIT RACER"),
        ("dc", Platform::Dreamcast, "COAST RALLY"),
        ("psp", Platform::Psp, "ORBIT LEAGUE"),
        ("ports", Platform::Ports, "OPEN HORIZON"),
    ]
    .into_iter()
    .enumerate()
    {
        let mut compositor = Compositor::new(&surface).unwrap();
        let clock = slot_ui::word_face("12:34");
        let clock = slot_ui::Printed::new(
            compositor.create_texture(clock.w, clock.h, &clock.rgba),
            clock.w,
        );
        let mut carts = Vec::new();
        let mut faces = Vec::new();
        for (j, name) in [
            "MOON HARBOR",
            "TINY VOYAGER",
            "ORBIT RUN",
            title,
            "PIXEL TRAILS",
            "DUSK RUNNER",
            "SOLAR DRIFT",
        ]
        .into_iter()
        .enumerate()
        {
            let path = labels.join(format!("{system}-{j}.png"));
            demo_label(
                &path,
                name,
                i + j,
                matches!(
                    platform,
                    Platform::Ps1 | Platform::Dreamcast | Platform::Psp | Platform::Ports
                ),
            );
            let shell = (platform == Platform::Gbc).then_some(slot_store::ShellChoice {
                outline: slot_store::Outline::Rounded,
                colour: [124, 122, 138],
                finish: slot_store::ShellFinish::Clear,
            });
            let cart = Cart {
                platform,
                stem: name.into(),
                title: name.into(),
                rom: "documentation-only".into(),
                label: Some(path),
                code: String::new(),
                shell,
            };
            let face = slot_ui::cart_face(&cart);
            faces.push(compositor.create_texture(face.w, face.h, &face.rgba));
            carts.push(cart);
        }
        let mut shelf = slot_ui::Shelf::new(carts);
        shelf.set_faces(faces);
        shelf.select(3);
        for _ in 0..120 {
            shelf.update(1.0 / 60.0);
        }
        let mut draws = Vec::<Draw>::new();
        shelf.draw(0.0, &mut draws);
        slot_ui::draw_footer(None, slot_ui::Printed::default(), None, clock, &mut draws);
        compositor.begin_frame();
        compositor.draw_list(&draws);
        png(
            &output.join(format!("{system}.png")),
            OUT_W,
            OUT_H,
            &compositor.read_frame(),
        );
        println!("Captured {system}");
    }
    std::fs::remove_dir_all(labels).unwrap();
}
