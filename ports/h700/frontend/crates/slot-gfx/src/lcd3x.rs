use std::f32::consts::PI;

const BRIGHTEN_SCANLINES: f32 = 16.0;
const BRIGHTEN_LCD: f32 = 4.0;

pub fn lcd3x_mask() -> [[[f32; 3]; 3]; 3] {
    let mut mask = [[[0.0f32; 3]; 3]; 3];
    for (oy, row) in mask.iter_mut().enumerate() {
        let yfactor = (BRIGHTEN_SCANLINES + (PI * (oy as f32 + 0.5) * 2.0 / 3.0).sin())
            / (BRIGHTEN_SCANLINES + 1.0);
        for (ox, cell) in row.iter_mut().enumerate() {
            for (c, v) in cell.iter_mut().enumerate() {
                let xfactor = (BRIGHTEN_LCD
                    + (PI * (ox as f32 + 0.5) * 2.0 / 3.0 + c as f32 * 2.0 * PI / 3.0).sin())
                    / (BRIGHTEN_LCD + 1.0);
                *v = yfactor * xfactor;
            }
        }
    }
    mask
}

pub fn mask_texture_rgba8() -> [u8; 3 * 3 * 4] {
    let mask = lcd3x_mask();
    let mut tex = [255u8; 3 * 3 * 4];
    for (texel, cell) in tex.chunks_exact_mut(4).zip(mask.iter().flatten()) {
        for (out, v) in texel.iter_mut().zip(cell) {
            *out = (v * 255.0).round() as u8;
        }
    }
    tex
}
