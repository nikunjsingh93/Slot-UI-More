use slot::bootlogo::{install, valid, Outcome};

fn bmp(fill: u8) -> Vec<u8> {
    let mut b = vec![0u8; 54];
    b[0..2].copy_from_slice(b"BM");
    b[2..6].copy_from_slice(&1_036_854u32.to_le_bytes());
    b[10..14].copy_from_slice(&54u32.to_le_bytes());
    b[14..18].copy_from_slice(&40u32.to_le_bytes());
    b[18..22].copy_from_slice(&720i32.to_le_bytes());
    b[22..26].copy_from_slice(&480i32.to_le_bytes());
    b[26..28].copy_from_slice(&1u16.to_le_bytes());
    b[28..30].copy_from_slice(&24u16.to_le_bytes());
    b.extend(std::iter::repeat_n(fill, 720 * 480 * 3));
    b
}

#[test]
fn only_a_720x480_24_bit_bmp_is_valid() {
    assert!(valid(&bmp(0)));
    assert!(!valid(&bmp(0)[..1000]));
    let mut wide = bmp(0);
    wide[18..22].copy_from_slice(&640i32.to_le_bytes());
    assert!(!valid(&wide));
    let mut deep = bmp(0);
    deep[28..30].copy_from_slice(&32u16.to_le_bytes());
    assert!(!valid(&deep));
    let mut packed = bmp(0);
    packed[30..34].copy_from_slice(&1u32.to_le_bytes());
    assert!(!valid(&packed));
    let mut png = bmp(0);
    png[0..2].copy_from_slice(b"PN");
    assert!(!valid(&png));
}

#[test]
fn a_new_logo_replaces_the_old_and_keeps_the_original_once() {
    let d = tempfile::tempdir().unwrap();
    std::fs::write(d.path().join("bootlogo.bmp"), bmp(1)).unwrap();

    assert_eq!(install(&bmp(2), d.path()).unwrap(), Outcome::Installed);
    assert_eq!(
        std::fs::read(d.path().join("bootlogo.bmp")).unwrap(),
        bmp(2)
    );
    assert_eq!(
        std::fs::read(d.path().join("bootlogo.baseos.bmp")).unwrap(),
        bmp(1)
    );

    assert_eq!(install(&bmp(3), d.path()).unwrap(), Outcome::Installed);
    assert_eq!(
        std::fs::read(d.path().join("bootlogo.baseos.bmp")).unwrap(),
        bmp(1),
        "the original was overwritten by a logo slot had installed"
    );
    assert!(!d.path().join("bootlogo.tmp").exists());
}

#[test]
fn the_same_logo_is_left_alone() {
    let d = tempfile::tempdir().unwrap();
    std::fs::write(d.path().join("bootlogo.bmp"), bmp(2)).unwrap();
    assert_eq!(install(&bmp(2), d.path()).unwrap(), Outcome::Same);
    assert!(!d.path().join("bootlogo.baseos.bmp").exists());
}

#[test]
fn a_bad_logo_never_touches_the_partition() {
    let d = tempfile::tempdir().unwrap();
    std::fs::write(d.path().join("bootlogo.bmp"), bmp(1)).unwrap();
    assert_eq!(install(&bmp(2)[..500], d.path()).unwrap(), Outcome::Invalid);
    assert_eq!(
        std::fs::read(d.path().join("bootlogo.bmp")).unwrap(),
        bmp(1)
    );
    assert!(!d.path().join("bootlogo.baseos.bmp").exists());
}
