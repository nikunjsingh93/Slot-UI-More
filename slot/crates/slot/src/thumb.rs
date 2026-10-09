use slot_retro::{GBA_H, GBA_W};

pub fn png(xrgb8888: &[u8]) -> Option<Vec<u8>> {
    let (w, h) = if xrgb8888.len() == 640 * 480 * 4 {
        (640, 480)
    } else if xrgb8888.len() == 256 * 192 * 4 {
        (256, 192)
    } else if xrgb8888.len() == 320 * 240 * 4 {
        (320, 240)
    } else {
        (GBA_W, GBA_H)
    };
    let n = (w * h) as usize;
    if xrgb8888.len() < n * 4 {
        return None;
    }
    let mut rgb = Vec::with_capacity(n * 3);
    for px in xrgb8888[..n * 4].chunks_exact(4) {
        rgb.extend_from_slice(&[px[2], px[1], px[0]]);
    }

    let mut out = Vec::new();
    let mut enc = png::Encoder::new(&mut out, w, h);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    let mut writer = enc.write_header().ok()?;
    writer.write_image_data(&rgb).ok()?;
    writer.finish().ok()?;
    Some(out)
}

#[cfg(test)]
mod tests {
    #[test]
    fn native_n64_thumbnail_keeps_dimensions_and_color() {
        let mut frame = vec![0; 320 * 240 * 4];
        frame[..4].copy_from_slice(&[3, 2, 1, 0]);
        let png = super::png(&frame).unwrap();
        let mut reader = png::Decoder::new(std::io::Cursor::new(png))
            .read_info()
            .unwrap();
        assert_eq!((reader.info().width, reader.info().height), (320, 240));
        let mut data = vec![0; reader.output_buffer_size()];
        reader.next_frame(&mut data).unwrap();
        assert_eq!(&data[..3], &[1, 2, 3]);
    }
}
