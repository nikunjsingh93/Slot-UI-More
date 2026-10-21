use slot_gfx::BACKDROP;
use slot_ui::{
    edge, gb_table_shells, gba_shell_for, housing, opening, shell_presets, table_keys,
    DEFAULT_SHELL, DMG_SHELL, DUAL_MODE_SHELL, GB_CLEAR_SHELL,
};

fn distance(a: [u8; 3], b: [f32; 4]) -> u32 {
    (0..3)
        .map(|i| (a[i] as i32 - (b[i] * 255.0).round() as i32).unsigned_abs())
        .sum()
}

#[test]
fn every_shell_is_visible_against_the_backdrop() {
    let codes = ["", "AMTE", "MSKE"].into_iter().chain(table_keys());
    for code in codes {
        let s = gba_shell_for(code);
        let d = distance(s.colour, BACKDROP);
        assert!(
            d > 60,
            "{code} shell {:?} is only {d}/765 from the backdrop, it will not be seen",
            s.colour
        );
    }
    assert!(distance(DEFAULT_SHELL.colour, BACKDROP) > 60);
    let paks = [
        ("the grey pak", DMG_SHELL),
        ("the black pak", DUAL_MODE_SHELL),
        ("the clear pak", GB_CLEAR_SHELL),
    ];
    let rows = gb_table_shells()
        .into_iter()
        .map(|s| ("a Game Boy table row", s));
    for (what, s) in paks.into_iter().chain(rows) {
        let d = distance(s.colour, BACKDROP);
        assert!(
            d > 60,
            "{what} {:?} is only {d}/765 from the backdrop, it will not be seen",
            s.colour
        );
    }
    for (name, s) in shell_presets() {
        assert!(
            distance(s.colour, BACKDROP) > 60,
            "the {name} preset will not be seen"
        );
    }
}

#[test]
fn every_chrome_band_is_visible_against_its_neighbour() {
    let d = |a: [f32; 4], b: [f32; 4]| -> f32 {
        (0..3).map(|i| (a[i] - b[i]).abs()).sum::<f32>() * 255.0
    };
    assert!(
        d(housing(), BACKDROP) > 60.0,
        "the housing vanishes into the backdrop"
    );
    assert!(
        d(opening(), housing()) > 40.0,
        "the opening vanishes into the housing"
    );
    assert!(
        d(edge(), opening()) > 60.0,
        "the lip vanishes into the opening"
    );
}
