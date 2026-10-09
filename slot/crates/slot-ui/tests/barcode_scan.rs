use std::process::Command;

use slot_ui::{code39, CODE39_NARROW, CODE39_WIDE};

const SCALE: f32 = 4.0;

fn render(payload: &str, to: &std::path::Path) {
    let run = code39(&format!("*{payload}*")).expect("payload is in the alphabet");
    let narrow = (CODE39_NARROW * SCALE) as usize;
    let wide = (CODE39_WIDE * SCALE) as usize;
    let quiet = narrow * 12;
    let elements: usize = run.iter().map(|w| if *w { wide } else { narrow }).sum();
    let gaps = (run.len() / 9) * narrow;
    let (w, h) = (quiet * 2 + elements + gaps, 120usize);

    let mut px = vec![255u8; w * h];
    let mut x = quiet;
    for chunk in run.chunks_exact(9) {
        for (n, is_wide) in chunk.iter().enumerate() {
            let run_w = if *is_wide { wide } else { narrow };
            if n.is_multiple_of(2) {
                for row in 0..h {
                    px[row * w + x..row * w + x + run_w].fill(0);
                }
            }
            x += run_w;
        }
        x += narrow;
    }

    let f = std::fs::File::create(to).expect("create png");
    let mut e = png::Encoder::new(std::io::BufWriter::new(f), w as u32, h as u32);
    e.set_color(png::ColorType::Grayscale);
    e.set_depth(png::BitDepth::Eight);
    e.write_header().unwrap().write_image_data(&px).unwrap();
}

#[test]
fn the_barcode_reads_back_as_what_was_encoded() {
    if Command::new("zbarimg").arg("--version").output().is_err() {
        eprintln!("skipped: zbarimg not installed (brew install zbar)");
        return;
    }
    let dir = tempfile::tempdir().expect("tempdir");
    for payload in ["0123456789ABCDEF", "0473885", "DEADBEEF", "F"] {
        let png = dir.path().join(format!("{payload}.png"));
        render(payload, &png);
        let out = Command::new("zbarimg")
            .args(["--quiet", "--raw"])
            .arg(&png)
            .output()
            .expect("run zbarimg");
        let got = String::from_utf8_lossy(&out.stdout).trim().to_string();
        assert_eq!(got, payload, "the reader disagrees with the encoder");
    }
}
