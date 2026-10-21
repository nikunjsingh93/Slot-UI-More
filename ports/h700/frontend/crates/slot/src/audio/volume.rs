pub fn gain(volume: u8) -> f32 {
    let v = volume.min(100) as f32 / 100.0;
    v * v
}

pub fn apply(samples: &mut [i16], volume: u8) {
    match volume.min(100) {
        100 => {}
        0 => samples.fill(0),
        v => {
            let g = gain(v);
            for s in samples.iter_mut() {
                *s = (*s as f32 * g) as i16;
            }
        }
    }
}
