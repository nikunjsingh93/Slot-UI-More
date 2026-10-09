use slot_store::gb::Class;
use slot_store::{Cart, Outline, Platform};

use crate::art;
use crate::shell::{shell_for, Finish, Shell};
use crate::silhouette::{
    cart_depth, cart_mask, detail_mask, gb_cart_depth, gb_cart_mask, gb_detail_mask, Detail,
    GbShell,
};
use crate::text;

pub const CART_W: u32 = 360;
pub const CART_H: u32 = (CART_W * SEATED_H + SEATED_W / 2) / SEATED_W;

pub const GB_CART_W: u32 = 312;

pub const GB_CART_H: u32 = (GB_CART_W * GB_SEATED_H + SEATED_W / 2) / SEATED_W;

pub const fn label_panel(w: u32, h: u32) -> (u32, u32, u32, u32) {
    (
        (w * 135 + 500) / 1000,
        (h * 231 + 500) / 1000,
        (w * 875 + 500) / 1000,
        (h * 882 + 500) / 1000,
    )
}

pub const LABEL_X: u32 = label_panel(CART_W, CART_H).0;
pub const LABEL_Y: u32 = label_panel(CART_W, CART_H).1;
pub const LABEL_W: u32 = label_panel(CART_W, CART_H).2 - LABEL_X;
pub const LABEL_H: u32 = label_panel(CART_W, CART_H).3 - LABEL_Y;

pub const fn gb_label_panel(w: u32, h: u32) -> (u32, u32, u32, u32) {
    (
        (w * 147 + 500) / 1000,
        (h * 291 + 500) / 1000,
        (w * 854 + 500) / 1000,
        (h * 868 + 500) / 1000,
    )
}

pub const GB_LABEL_X: u32 = gb_label_panel(GB_CART_W, GB_CART_H).0;
pub const GB_LABEL_Y: u32 = gb_label_panel(GB_CART_W, GB_CART_H).1;
pub const GB_LABEL_W: u32 = gb_label_panel(GB_CART_W, GB_CART_H).2 - GB_LABEL_X;
pub const GB_LABEL_H: u32 = gb_label_panel(GB_CART_W, GB_CART_H).3 - GB_LABEL_Y;

const PAD: u32 = 10;
const MAX_LINES: usize = 3;
const MAX_PX: f32 = LABEL_H as f32 / (MAX_LINES as f32 * 1.36);
const GB_MAX_PX: f32 = (GB_LABEL_H - 2 * PAD) as f32 / MAX_LINES as f32;
const MIN_PX: f32 = 10.0;

const RIM: u32 = (4 * CART_W + SEATED_W / 2) / SEATED_W;
const GB_RIM: u32 = (7 * GB_CART_W + SEATED_W / 2) / SEATED_W;

pub struct CartFace {
    pub rgba: Vec<u8>,
    pub w: u32,
    pub h: u32,
}

struct Spec {
    w: u32,
    h: u32,
    label: (u32, u32, u32, u32),
    mask: &'static [u8],
    depth: &'static [u8],
    detail: &'static Detail,
    max_px: f32,
    max_h: f32,
    rim: u32,
    board: Board,
}

struct Board {
    x: (f32, f32),
    top: f32,
    pins: (f32, f32),
    contacts_from: f32,
    traces_from: f32,
}

enum Shape {
    Gba,
    Gb(GbShell),
}

pub fn gb_shell_of(cart: &Cart) -> Option<GbShell> {
    match cart.platform {
        Platform::Gba
        | Platform::Ps1
        | Platform::Snes
        | Platform::N64
        | Platform::Dreamcast
        | Platform::Psp => None,
        Platform::Gb | Platform::Gbc => Some(match cart.shell.map(|c| c.outline) {
            Some(Outline::Notched) => GbShell::Notched,
            Some(Outline::Rounded) => GbShell::Rounded,
            _ => match slot_store::gb::class(&cart.rom) {
                Class::ColourOnly => GbShell::Rounded,
                Class::Original | Class::DualMode => GbShell::Notched,
            },
        }),
    }
}

fn shape_of(cart: &Cart) -> Shape {
    match gb_shell_of(cart) {
        None => Shape::Gba,
        Some(shell) => Shape::Gb(shell),
    }
}

fn spec(shape: Shape) -> Spec {
    match shape {
        Shape::Gba => Spec {
            w: CART_W,
            h: CART_H,
            label: (LABEL_X, LABEL_Y, LABEL_W, LABEL_H),
            mask: cart_mask(),
            depth: cart_depth(),
            detail: detail_mask(),
            max_px: MAX_PX,
            max_h: f32::INFINITY,
            rim: RIM,
            board: Board {
                x: (0.135, 0.875),
                top: LABEL_Y as f32 / CART_H as f32,
                pins: (0.17, 0.84),
                contacts_from: 0.85,
                traces_from: 0.76,
            },
        },
        Shape::Gb(shell) => Spec {
            w: GB_CART_W,
            h: GB_CART_H,
            label: (GB_LABEL_X, GB_LABEL_Y, GB_LABEL_W, GB_LABEL_H),
            mask: gb_cart_mask(shell),
            depth: gb_cart_depth(shell),
            detail: gb_detail_mask(shell),
            max_px: GB_MAX_PX,
            max_h: (GB_LABEL_H - 2 * PAD) as f32,
            rim: GB_RIM,
            board: Board {
                x: (0.075, 0.925),
                top: 0.07,
                pins: (0.1, 0.9),
                contacts_from: 0.905,
                traces_from: 0.84,
            },
        },
    }
}

pub const SEATED_W: u32 = 240;
const SEATED_H: u32 = 141;
const GB_SEATED_H: u32 = 259;

pub fn seated_box(platform: Platform) -> (u32, u32) {
    match platform {
        Platform::Ps1 | Platform::Dreamcast => (312, 312),
        Platform::Psp => (312, 224),
        Platform::Snes => (240, 147),
        Platform::N64 => (240, 155),
        Platform::Gba => (SEATED_W, SEATED_H),
        Platform::Gb | Platform::Gbc => (SEATED_W, GB_SEATED_H),
    }
}

pub fn cart_box(platform: Platform) -> (u32, u32) {
    if platform == Platform::N64 {
        return (360, 232);
    }
    if platform == Platform::Snes {
        return (360, 220);
    }
    if matches!(platform, Platform::Ps1 | Platform::Dreamcast) {
        return (312, 312);
    }
    if platform == Platform::Psp {
        return (312, 224);
    }
    let s = spec(match platform {
        Platform::Gba
        | Platform::Ps1
        | Platform::Snes
        | Platform::N64
        | Platform::Dreamcast
        | Platform::Psp => Shape::Gba,
        Platform::Gb | Platform::Gbc => Shape::Gb(GbShell::Notched),
    });
    (s.w, s.h)
}

pub fn cart_shadow() -> CartFace {
    shadow(CART_W, CART_H, cart_mask())
}

pub fn gb_cart_shadow(shell: GbShell) -> CartFace {
    shadow(GB_CART_W, GB_CART_H, gb_cart_mask(shell))
}

fn shadow(w: u32, h: u32, mask: &[u8]) -> CartFace {
    let mut rgba = Vec::with_capacity((w * h * 4) as usize);
    for cover in mask {
        rgba.extend_from_slice(&[0, 0, 0, *cover]);
    }
    CartFace { rgba, w, h }
}

pub fn cart_face(cart: &Cart) -> CartFace {
    if cart.platform == Platform::N64 {
        return n64_face(cart);
    }
    if cart.platform == Platform::Snes {
        return snes_face(cart);
    }
    if matches!(cart.platform, Platform::Ps1 | Platform::Dreamcast) {
        return disc_face(cart);
    }
    if cart.platform == Platform::Psp {
        return umd_face(cart);
    }
    let s = spec(shape_of(cart));
    let shell = shell_for(cart);
    let mut face = shell_face(&s, &shell);
    let (_, _, lw, lh) = s.label;
    let label = match cart.label.as_deref().and_then(|p| art::cover(p, lw, lh)) {
        Some(rgba) => rgba,
        None => generated_label(&s, &label_text(cart)),
    };
    mould_detail(&s, &mut face);
    recess_label(&s, &mut face, &shell);
    paste_label(&s, &mut face, &label);
    clip_to_silhouette(&s, &mut face);
    face
}

fn clip_to_silhouette(s: &Spec, face: &mut CartFace) {
    for (px, cover) in face.rgba.chunks_exact_mut(4).zip(s.mask) {
        px[3] = ((px[3] as u32 * *cover as u32 + 127) / 255) as u8;
    }
}

pub fn label_colour(title: &str) -> [u8; 3] {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in title.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hsv_to_rgb((h % 360) as f32, 0.52, 0.74)
}

pub fn label_text(cart: &Cart) -> String {
    clean_label(&cart.stem)
}

pub fn clean_label(stem: &str) -> String {
    let mut bare = String::with_capacity(stem.len());
    let mut depth = 0u32;
    for ch in stem.chars() {
        match ch {
            '(' | '[' => depth += 1,
            ')' | ']' => depth = depth.saturating_sub(1),
            _ if depth == 0 => bare.push(ch),
            _ => {}
        }
    }

    let mut out = String::with_capacity(bare.len());
    for word in bare.split_whitespace().filter(|w| *w != "-") {
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(word);
    }
    if out.is_empty() {
        stem.to_string()
    } else {
        out
    }
}

pub fn label_tags(stem: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0u32;
    let mut cur = String::new();
    for ch in stem.chars() {
        match ch {
            '(' | '[' => {
                depth += 1;
                if depth == 1 {
                    cur.clear();
                    continue;
                }
            }
            ')' | ']' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    let t = cur.trim();
                    if !t.is_empty() {
                        out.push(t.to_string());
                    }
                    continue;
                }
            }
            _ => {}
        }
        if depth >= 1 {
            cur.push(ch);
        }
    }
    out
}

fn shell_face(s: &Spec, shell: &Shell) -> CartFace {
    let mut rgba = Vec::with_capacity((s.w * s.h * 4) as usize);
    let edge = rim_colour(through(shell.colour, shell.colour));
    for (i, depth) in s.depth.iter().enumerate() {
        let (x, y) = (i as u32 % s.w, i as u32 / s.w);
        let c = match shell.finish {
            Finish::Solid => shell.colour,
            Finish::Translucent | Finish::Glitter => {
                let body = through(shell.colour, inside(s, x, y).unwrap_or(shell.colour));
                let c = lerp(edge, body, (*depth as u32).min(s.rim), s.rim);
                if shell.finish == Finish::Glitter && fleck(x, y) {
                    lerp(c, [0xff; 3], 100, 255)
                } else {
                    c
                }
            }
        };
        rgba.extend_from_slice(&[c[0], c[1], c[2], 255]);
    }
    CartFace {
        rgba,
        w: s.w,
        h: s.h,
    }
}

const SURFACE: u32 = 150;

const BOARD: [u8; 3] = [0x2c, 0x96, 0x52];
const TRACE: [u8; 3] = [0x5a, 0xb4, 0x74];
const GOLD: [u8; 3] = [0xe6, 0xb4, 0x46];

fn through(plastic: [u8; 3], behind: [u8; 3]) -> [u8; 3] {
    let filter =
        |a: [u8; 3], b: [u8; 3]| [0, 1, 2].map(|c| (a[c] as u32 * b[c] as u32 / 255) as u8);
    lerp(
        filter(plastic, behind),
        filter(plastic, plastic),
        SURFACE,
        255,
    )
}

fn inside(s: &Spec, x: u32, y: u32) -> Option<[u8; 3]> {
    let b = &s.board;
    let (fx, fy) = (x as f32 / s.w as f32, y as f32 / s.h as f32);
    if !(b.x.0..b.x.1).contains(&fx) || fy < b.top {
        return None;
    }
    let (c0, c1) = b.pins;
    let pitch = (c1 - c0) / 32.0;
    let on_contact = (c0..c1).contains(&fx) && ((fx - c0) % pitch) < pitch * 0.6;
    if on_contact && fy >= b.contacts_from {
        return Some(GOLD);
    }
    if on_contact && ((fx - c0) % pitch) < pitch * 0.2 && fy >= b.traces_from {
        Some(TRACE)
    } else {
        Some(BOARD)
    }
}

fn fleck(x: u32, y: u32) -> bool {
    let mut h = x.wrapping_mul(0x9e37_79b9) ^ y.wrapping_mul(0x85eb_ca6b);
    h ^= h >> 15;
    h = h.wrapping_mul(0x2c1b_3c6d);
    h ^= h >> 12;
    h.is_multiple_of(100)
}

fn rim_colour(base: [u8; 3]) -> [u8; 3] {
    let mean = (base[0] as u16 + base[1] as u16 + base[2] as u16) / 3;
    base.map(|c| {
        let grey = (3 * c as u16 + mean) / 4;
        (grey + (255 - grey) * 2 / 5) as u8
    })
}

fn lerp(a: [u8; 3], b: [u8; 3], num: u32, den: u32) -> [u8; 3] {
    let mut out = [0u8; 3];
    for c in 0..3 {
        out[c] = ((a[c] as u32 * (den - num) + b[c] as u32 * num) / den) as u8;
    }
    out
}

const BEVEL: u32 = 3;

fn mould_detail(s: &Spec, face: &mut CartFace) {
    let mix = |px: &mut [u8], to: [u8; 3], a: u32| {
        for c in 0..3 {
            px[c] = ((to[c] as u32 * a + px[c] as u32 * (255 - a) + 127) / 255) as u8;
        }
    };
    let sides = s.detail.shadow.iter().zip(&s.detail.highlight);
    for (px, (shade, light)) in face.rgba.chunks_exact_mut(4).zip(sides) {
        let under = [px[0], px[1], px[2]];
        if *shade > 0 {
            mix(px, under.map(|c| (c as f32 * 0.62) as u8), *shade as u32);
        }
        if *light > 0 {
            mix(
                px,
                under.map(|c| c + ((255 - c) as f32 * 0.24) as u8),
                *light as u32,
            );
        }
    }
}

fn recess_label(s: &Spec, face: &mut CartFace, shell: &Shell) {
    let shade = |c: [u8; 3], f: f32| -> [u8; 3] {
        [
            (c[0] as f32 * f).clamp(0.0, 255.0) as u8,
            (c[1] as f32 * f).clamp(0.0, 255.0) as u8,
            (c[2] as f32 * f).clamp(0.0, 255.0) as u8,
        ]
    };
    let dark = shade(shell.colour, 0.55);
    let lit = shade(shell.colour, 1.45);

    let (lx, ly, lw, lh) = s.label;
    let (w, h) = (s.w, s.h);
    let bevel = (BEVEL * w + SEATED_W / 2) / SEATED_W;
    let (x0, y0) = (lx - bevel, ly - bevel);
    let (x1, y1) = (lx + lw + bevel, ly + lh + bevel);
    let clear = shell.finish != Finish::Solid;
    let mut put = |x: u32, y: u32, upper: bool| {
        if x >= w || y >= h {
            return;
        }
        let d = ((y * w + x) * 4) as usize;
        let c = if clear {
            let under = [face.rgba[d], face.rgba[d + 1], face.rgba[d + 2]];
            shade(under, if upper { 0.7 } else { 1.18 })
        } else if upper {
            dark
        } else {
            lit
        };
        face.rgba[d] = c[0];
        face.rgba[d + 1] = c[1];
        face.rgba[d + 2] = c[2];
    };
    for y in y0..y1 {
        for x in x0..x1 {
            let inside = (lx..lx + lw).contains(&x) && (ly..ly + lh).contains(&y);
            if inside {
                continue;
            }
            let from_top = y.saturating_sub(y0);
            let from_left = x.saturating_sub(x0);
            let from_bottom = y1.saturating_sub(y + 1);
            let from_right = x1.saturating_sub(x + 1);
            let upper = from_top.min(from_left);
            let lower = from_bottom.min(from_right);
            put(x, y, upper <= lower);
        }
    }
}

fn paste_label(s: &Spec, face: &mut CartFace, label: &[u8]) {
    let (lx, ly, lw, lh) = s.label;
    for y in 0..lh {
        for x in 0..lw {
            let src = ((y * lw + x) * 4) as usize;
            let a = label[src + 3] as u32;
            if a == 0 {
                continue;
            }
            let d = (((y + ly) * s.w + x + lx) * 4) as usize;
            for c in 0..3 {
                face.rgba[d + c] =
                    ((label[src + c] as u32 * a + face.rgba[d + c] as u32 * (255 - a) + 127) / 255)
                        as u8;
            }
        }
    }
}

fn generated_label(s: &Spec, title: &str) -> Vec<u8> {
    let (_, _, lw, lh) = s.label;
    let bg = label_colour(title);
    let mut rgba = Vec::with_capacity((lw * lh * 4) as usize);
    for _ in 0..lw * lh {
        rgba.extend_from_slice(&[bg[0], bg[1], bg[2], 255]);
    }

    if let Some(font) = text::label_font() {
        let layout = text::fit_box(
            font,
            title,
            (lw - 2 * PAD) as f32,
            s.max_h,
            MAX_LINES,
            s.max_px,
            MIN_PX,
        );
        text::draw_centred(&mut rgba, lw, lh, &layout, ink(bg));
    }
    rgba
}

fn ink(bg: [u8; 3]) -> [u8; 3] {
    let luma = 0.2126 * bg[0] as f32 + 0.7152 * bg[1] as f32 + 0.0722 * bg[2] as f32;
    if luma > 140.0 {
        [0x1a, 0x18, 0x16]
    } else {
        [0xf4, 0xf1, 0xea]
    }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> [u8; 3] {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (r, g, b) = match (h / 60.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    [
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    ]
}

/// A printed compact disc with a transparent spindle hole and metallic hub.
fn disc_face(cart: &Cart) -> CartFace {
    let w = 312u32;
    let title = label_text(cart);
    let base = label_colour(&title);
    let artwork = cart.label.as_deref().and_then(|p| art::cover(p, w, w));
    // The studio marks original disc scans with a transparent spindle hole.
    let scanned_disc = artwork
        .as_ref()
        .is_some_and(|a| a[3] < 8 && a[((156 * w + 156) * 4 + 3) as usize] < 8);
    let mut print = vec![0u8; 220 * 54 * 4];
    if artwork.is_none() {
        if let Some(font) = text::label_font() {
            let layout = text::fit_box(font, &title, 212.0, 50.0, 2, 22.0, 10.0);
            text::draw_centred(&mut print, 220, 54, &layout, ink(base));
        }
    }
    let mut rgba = vec![0u8; (w * w * 4) as usize];
    for y in 0..w {
        for x in 0..w {
            let dx = x as f32 + 0.5 - 156.0;
            let dy = y as f32 + 0.5 - 156.0;
            let r = dx.hypot(dy);
            let o = ((y * w + x) * 4) as usize;
            if !(19.5..155.5).contains(&r) {
                continue;
            }
            let metal = if scanned_disc {
                !(27.0..=153.0).contains(&r)
            } else {
                !(47.0..=148.0).contains(&r)
            };
            let c = if metal {
                let sheen = (dx.atan2(dy) * 3.0).sin() * 18.0;
                let v = (if r < 27.0 { 155.0 } else { 210.0 }) + sheen;
                [v as u8, (v + 4.0) as u8, (v + 8.0) as u8]
            } else if let Some(art) = &artwork {
                [art[o], art[o + 1], art[o + 2]]
            } else {
                let shade = 0.86 + 0.14 * (1.0 - y as f32 / 312.0);
                base.map(|c| (c as f32 * shade) as u8)
            };
            rgba[o..o + 4].copy_from_slice(&[c[0], c[1], c[2], 255]);
            if !metal && (46..266).contains(&x) && (53..107).contains(&y) {
                let a = (((y - 53) * 220 + x - 46) * 4) as usize;
                let alpha = print[a + 3] as u32;
                for k in 0..3 {
                    rgba[o + k] = ((print[a + k] as u32 * alpha
                        + rgba[o + k] as u32 * (255 - alpha))
                        / 255) as u8;
                }
            }
            let coverage = (155.5 - r).min(r - 19.5).clamp(0.0, 1.0);
            rgba[o + 3] = (coverage * 255.0) as u8;
        }
    }
    CartFace { rgba, w, h: w }
}

// PSP Universal Media Disc, with a white shell around the printed disc.
fn umd_face(cart: &Cart) -> CartFace {
    let (w, h) = (312u32, 224u32);
    let disc = disc_face(cart);
    let mut rgba = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let dx = (x as f32 + 0.5 - 156.0).abs();
            let lower = (y as f32 - 135.0).max(0.0);
            let edge = 153.0 - dx;
            if edge < 0.0 || y < 3 || dx * dx + lower * lower > 153.0 * 153.0 {
                continue;
            }
            let o = ((y * w + x) * 4) as usize;
            let shade = if edge < 4.0 || y < 7 || y > 218 {
                164
            } else {
                234
            };
            rgba[o..o + 4].copy_from_slice(&[shade, shade, shade, 255]);
            let rx = x as f32 + 0.5 - 156.0;
            let ry = y as f32 + 0.5 - 111.0;
            let r = rx.hypot(ry);
            if r < 99.0 {
                let sx = ((rx / 99.0 * 155.0 + 156.0) as u32).min(311);
                let sy = ((ry / 99.0 * 155.0 + 156.0) as u32).min(311);
                let d = ((sy * disc.w + sx) * 4) as usize;
                if disc.rgba[d + 3] > 0 {
                    rgba[o..o + 4].copy_from_slice(&disc.rgba[d..d + 4]);
                }
            }
        }
    }
    CartFace { rgba, w, h }
}

// A grey SNES cartridge: label, moulded side ribs and bottom connector.
fn snes_face(cart: &Cart) -> CartFace {
    let (w, h) = (360u32, 220u32);
    let mut rgba = vec![0u8; (w * h * 4) as usize];
    let label = cart.label.as_deref().and_then(|p| art::cover(p, 224, 139));
    let label = label.unwrap_or_else(|| {
        let colour = label_colour(&cart.stem);
        let mut image = vec![0u8; 224 * 139 * 4];
        for p in image.chunks_exact_mut(4) {
            p.copy_from_slice(&[colour[0], colour[1], colour[2], 255]);
        }
        if let Some(font) = text::label_font() {
            let layout = text::fit_box(font, &clean_label(&cart.stem), 208.0, 120.0, 3, 24.0, 10.0);
            text::draw_centred(&mut image, 224, 139, &layout, ink(colour));
        }
        image
    });
    for y in 0..h {
        for x in 0..w {
            let inside = if y < 202 {
                let dx = (x as i32 - 12).min(347 - x as i32).min(0);
                let dy = (y as i32 - 12).min(189 - y as i32).min(0);
                dx * dx + dy * dy <= 144
            } else {
                (66..294).contains(&x)
            };
            if !inside {
                continue;
            }
            let shade = if y >= 202 {
                85
            } else if !(4..=355).contains(&x) || !(4..=197).contains(&y) {
                123
            } else {
                183
            };
            let o = ((y * w + x) * 4) as usize;
            rgba[o..o + 4].copy_from_slice(&[shade, shade, shade + 3, 255]);
            if (68..292).contains(&x) && (24..163).contains(&y) {
                let l = (((y - 24) * 224 + x - 68) * 4) as usize;
                rgba[o..o + 4].copy_from_slice(&label[l..l + 4]);
            } else if !(54..=305).contains(&x) && (38..164).contains(&y) && y % 12 < 3 {
                rgba[o..o + 3].fill(143);
            } else if y > 205 && x % 12 < 7 {
                rgba[o..o + 3].copy_from_slice(&[179, 151, 79]);
            }
        }
    }
    CartFace { rgba, w, h }
}

fn n64_face(cart: &Cart) -> CartFace {
    let (w, h) = (360u32, 232u32);
    let title = label_text(cart);
    let label = cart.label.as_deref().and_then(|p| art::cover(p, 180, 190));
    let base = label_colour(&title);
    let mut text_rgba = vec![0; 180 * 190 * 4];
    if label.is_none() {
        for p in text_rgba.chunks_exact_mut(4) {
            p.copy_from_slice(&[base[0], base[1], base[2], 255]);
        }
        if let Some(font) = text::label_font() {
            let layout = text::fit_box(font, &title, 164.0, 160.0, 4, 24.0, 10.0);
            text::draw_centred(&mut text_rgba, 180, 190, &layout, ink(base));
        }
    }
    let print = label.as_deref().unwrap_or(&text_rgba);
    let mut rgba = vec![0; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let top = 8.0 + ((x as f32 - 180.0) / 180.0).powi(2) * 25.0;
            if y < 209 && y as f32 >= top && (3..357).contains(&x) {
                let v = if !(8..=351).contains(&x) || (y as f32) < top + 4.0 {
                    114
                } else {
                    173
                };
                let o = ((y * w + x) * 4) as usize;
                rgba[o..o + 4].copy_from_slice(&[v, v, v + 3, 255]);
                if (90..270).contains(&x) && (18..208).contains(&y) {
                    let q = (((y - 18) * 180 + x - 90) * 4) as usize;
                    rgba[o..o + 4].copy_from_slice(&print[q..q + 4]);
                    rgba[o + 3] = 255;
                }
            }
            if (74..286).contains(&x) && (209..232).contains(&y) {
                let o = ((y * w + x) * 4) as usize;
                let c = if x % 12 < 7 && y > 216 {
                    [180, 150, 71, 255]
                } else {
                    [72, 73, 75, 255]
                };
                rgba[o..o + 4].copy_from_slice(&c);
            }
        }
    }
    CartFace { rgba, w, h }
}

#[cfg(test)]
mod disc_tests {
    use super::*;
    #[test]
    fn ps1_is_a_disc_with_a_hole_and_no_cartridge_corners() {
        let cart = Cart {
            platform: Platform::Ps1,
            stem: "Disc game".into(),
            title: String::new(),
            rom: "game.chd".into(),
            label: None,
            code: String::new(),
            shell: None,
        };
        let face = cart_face(&cart);
        assert_eq!((face.w, face.h), (312, 312));
        assert_eq!(face.rgba[3], 0);
        assert_eq!(face.rgba[((156 * 312 + 156) * 4 + 3) as usize], 0);
        assert_eq!(face.rgba[((70 * 312 + 156) * 4 + 3) as usize], 255);
    }
}
