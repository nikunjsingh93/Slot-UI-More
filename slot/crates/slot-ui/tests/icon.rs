use slot_ui::{badge_face, icon_box, icon_face, Badge, Icon, HUD_ICON_PX, LINK_HOST_INK};

#[test]
fn icons_are_indexed_in_declaration_order() {
    for (i, icon) in Icon::ALL.iter().enumerate() {
        assert_eq!(icon.index(), i, "{icon:?} is out of order in ALL");
    }
}

#[test]
fn the_shared_box_is_the_one_faces_come_back_in() {
    let f = icon_face(Icon::Rewind, 18.0, [255, 255, 255]);
    assert_eq!(icon_box(18.0), (f.w, f.h));
}

#[test]
fn every_icon_rasterises_to_something_visible() {
    for icon in Icon::ALL {
        let f = icon_face(icon, 18.0, [255, 255, 255]);
        let lit = f.rgba.chunks(4).filter(|p| p[3] > 0).count();
        assert!(
            lit > 20,
            "{icon:?} rasterised to {lit} opaque pixels, glyph is missing"
        );
    }
}

#[test]
fn icons_are_distinguishable_from_each_other() {
    let faces: Vec<_> = Icon::ALL
        .iter()
        .map(|i| (i, icon_face(*i, 18.0, [255, 255, 255]).rgba))
        .collect();
    for (n, (a, fa)) in faces.iter().enumerate() {
        for (b, fb) in &faces[n + 1..] {
            assert_ne!(fa, fb, "{a:?} and {b:?} rasterised identically");
        }
    }
}

#[test]
fn all_icons_share_one_box() {
    let first = icon_face(Icon::Volume, 18.0, [255, 255, 255]);
    for icon in Icon::ALL {
        let f = icon_face(icon, 18.0, [255, 255, 255]);
        assert_eq!(
            (f.w, f.h),
            (first.w, first.h),
            "{icon:?} is {}x{}, the row would reflow",
            f.w,
            f.h
        );
    }
}

#[test]
fn the_icon_is_tinted_with_the_colour_asked_for() {
    icon_face(Icon::Volume, 18.0, [255, 255, 255]);
    let f = icon_face(Icon::Volume, 18.0, [200, 30, 10]);
    let opaque: Vec<_> = f.rgba.chunks(4).filter(|p| p[3] == 255).collect();
    assert!(
        opaque.iter().any(|p| p[..3] == [200, 30, 10]),
        "the tint did not reach the glyph"
    );
    assert!(
        !opaque.iter().any(|p| p[..3] == [255, 255, 255]),
        "the cache served the previous colour"
    );
}

#[test]
fn a_bigger_size_gives_a_bigger_face() {
    let small = icon_face(Icon::Brightness, 18.0, [255, 255, 255]);
    let big = icon_face(Icon::Brightness, 36.0, [255, 255, 255]);
    assert!(
        big.h > small.h && big.w > small.w,
        "36 px rasterised to {}x{} against 18 px at {}x{}, the size was ignored",
        big.w,
        big.h,
        small.w,
        small.h
    );
    assert_eq!(
        icon_face(Icon::Brightness, 18.0, [255, 255, 255]).rgba,
        small.rgba,
        "the cache handed back the wrong size"
    );
}

#[test]
fn icons_carry_a_halo_so_they_read_without_a_plate() {
    let f = icon_face(Icon::FastForward, 18.0, [255, 255, 255]);
    let opaque = |p: &[u8]| p[3] > 128;
    let dark = f.rgba.chunks(4).filter(|p| opaque(p) && p[0] < 60).count();
    let light = f.rgba.chunks(4).filter(|p| opaque(p) && p[0] > 200).count();
    assert!(dark > 8, "no halo, only {dark} dark pixels");
    assert!(
        light > 8,
        "the glyph itself is gone, only {light} light pixels"
    );
}

#[test]
fn a_new_icon_does_not_resize_the_box_the_others_share() {
    assert_eq!(
        icon_box(HUD_ICON_PX),
        (26, 27),
        "if this moved, the new glyph is wider or taller than every icon before it"
    );
}

#[test]
fn icon_box_accounts_for_the_halo() {
    let (bw, bh) = icon_box(18.0);
    let f = icon_face(Icon::Volume, 18.0, [255, 255, 255]);
    assert_eq!((bw, bh), (f.w, f.h));
}

#[test]
fn link_badges_share_the_icon_box_and_differ() {
    let link = badge_face(Badge::Link, HUD_ICON_PX, [255, 255, 255]);
    let broken = badge_face(Badge::LinkBroken, HUD_ICON_PX, [255, 255, 255]);
    assert_eq!(
        (link.w, link.h),
        icon_box(HUD_ICON_PX),
        "the link badge would move the corner"
    );
    assert_eq!((broken.w, broken.h), icon_box(HUD_ICON_PX));
    assert_ne!(
        link.rgba, broken.rgba,
        "a broken link looks like a live one"
    );
    assert!(
        link.rgba.chunks(4).any(|p| p[3] == 255),
        "the badge drew nothing"
    );
}

#[test]
fn a_link_badge_wears_its_colour() {
    let face = badge_face(Badge::Link, HUD_ICON_PX, LINK_HOST_INK);
    assert!(face
        .rgba
        .chunks(4)
        .any(|p| p[3] == 255 && p[..3] == LINK_HOST_INK));
}
