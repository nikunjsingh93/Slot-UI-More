use slot::link_kind::{link_carried, link_kind, serial_option, LinkKind};

#[test]
fn the_wireless_adapter_games_are_wireless() {
    for code in ["BMGE", "BTME", "BR5E", "BRBE", "BDGE", "B4UE", "B85A"] {
        assert_eq!(link_kind(code, "", true), LinkKind::Wireless, "{code}");
    }
    for code in ["BPEE", "BPRE", "BPGE"] {
        assert_eq!(link_kind(code, "", true), LinkKind::Cable, "{code}");
    }
    for (code, title) in [
        ("BPRE", "POKEMON FIRE"),
        ("BPGE", "POKEMON LEAF"),
        ("BPEE", "POKEMON EMER"),
    ] {
        assert_eq!(link_kind(code, title, true), LinkKind::Wireless, "{code}");
    }
}

#[test]
fn ruby_sapphire_and_advance_wars_use_the_cable() {
    for code in ["AXVE", "AXPE", "AWRE", "AW2E"] {
        assert_eq!(
            link_kind(code, "POKEMON RUBY", true),
            LinkKind::Cable,
            "{code}"
        );
    }
}

#[test]
fn a_pokemon_hack_links_by_cable() {
    for (code, title, clean) in [
        ("BPEE", "POKEMON EMER", false),
        ("BPRE", "PKMN RADICAL", true),
        ("ZZZZ", "POKEMON EMER", true),
        ("", "POKEMON", true),
    ] {
        assert_eq!(
            link_kind(code, title, clean),
            LinkKind::Cable,
            "{code} {title} {clean}"
        );
    }
}

#[test]
fn everything_else_is_the_cable() {
    assert_eq!(link_kind("SLTE", "SLOT TEST", true), LinkKind::Cable);
    assert_eq!(link_kind("", "", true), LinkKind::Cable);
}

#[test]
fn the_mode_gpsp_would_pick_is_left_to_gpsp() {
    for (kind, code, title) in [
        (LinkKind::Cable, "SLTE", "SLOT TEST"),
        (LinkKind::Wireless, "BMGE", "MARIOGOLFADV"),
        (LinkKind::Wireless, "BPEE", "POKEMON EMER"),
        (LinkKind::Cable, "AXVE", "POKEMON RUBY"),
        (LinkKind::Cable, "AWRE", "ADVANCEWARS"),
        (LinkKind::Cable, "AW2E", "ADVANCEWARS2"),
    ] {
        assert_eq!(serial_option(kind, kind, code, title), "auto", "{code}");
    }
}

#[test]
fn switched_to_the_adapter_is_rfu() {
    for (code, title) in [
        ("SLTE", "SLOT TEST"),
        ("AXVE", "POKEMON RUBY"),
        ("AWRE", "ADVANCEWARS"),
        ("", ""),
    ] {
        assert_eq!(
            serial_option(LinkKind::Wireless, LinkKind::Cable, code, title),
            "rfu",
            "{code}"
        );
    }
}

#[test]
fn a_pokemon_cart_switched_to_the_cable_uses_the_pokemon_protocol() {
    for (code, title) in [
        ("BPEE", "POKEMON EMER"),
        ("ZZZZ", "POKEMON"),
        ("AXVE", "PKMN HACK"),
        ("AXPE", "PKMN HACK"),
        ("BPEE", "PKMN HACK"),
        ("BPRE", "PKMN HACK"),
        ("BPGE", "PKMN HACK"),
    ] {
        assert_eq!(
            serial_option(LinkKind::Cable, LinkKind::Wireless, code, title),
            "mul_poke",
            "{code} {title}"
        );
    }
}

#[test]
fn advance_wars_switched_to_the_cable_uses_its_own_protocol() {
    assert_eq!(
        serial_option(LinkKind::Cable, LinkKind::Wireless, "AWRE", "ADVANCEWARS"),
        "mul_aw1"
    );
    assert_eq!(
        serial_option(LinkKind::Cable, LinkKind::Wireless, "AW2E", "ADVANCEWARS2"),
        "mul_aw2"
    );
}

#[test]
fn any_other_cart_switched_to_the_cable_stays_on_auto() {
    for (code, title) in [("BMGE", "MARIOGOLFADV"), ("AWXE", "ADVANCEWARS"), ("", "")] {
        assert_eq!(
            serial_option(LinkKind::Cable, LinkKind::Wireless, code, title),
            "auto",
            "{code}"
        );
    }
}

#[test]
fn gpsp_carries_the_adapter_list_the_pokemon_family_and_advance_wars() {
    for (code, title) in [
        ("BMGE", "MARIOGOLFADV"),
        ("BTME", "MARIOKARTADV"),
        ("B2WE", "WARIOWARE"),
        ("BPEE", "POKEMON EMER"),
        ("AXVE", "POKEMON RUBY"),
        ("AXPE", "POKEMON SAPP"),
        ("ZZZZ", "POKEMON FIRE"),
        ("AWRE", "ADVANCEWARS"),
        ("AWRP", "ADVANCEWARS"),
        ("AW2E", "ADVANCEWARS2"),
        ("AW2P", "ADVANCEWARS2"),
    ] {
        assert!(link_carried(code, title), "{code} {title}");
    }
}

#[test]
fn gpsp_carries_nothing_else() {
    for (code, title) in [
        ("2ATE", "APOTRIS"),
        ("SLTE", "SLOT TEST"),
        ("AMTE", "METROIDFUSION"),
        ("A88E", "MARIO&LUIGIRPG"),
        ("AX4E", "SUPER MARIOD"),
        ("", ""),
    ] {
        assert!(!link_carried(code, title), "{code} {title}");
    }
}

#[test]
fn a_pokemon_hack_is_carried_the_same_as_the_retail_game() {
    for (code, title) in [
        ("BPEE", "PKMN RADICAL"),
        ("ZZZZ", "POKEMON"),
        ("BPPE", "POKEMON PINB"),
    ] {
        assert!(link_carried(code, title), "{code} {title}");
    }
}

#[test]
fn the_carried_carts_are_exactly_the_ones_with_a_mode_of_their_own() {
    for (code, title) in [
        ("BMGE", "MARIOGOLFADV"),
        ("BPEE", "POKEMON EMER"),
        ("AXVE", "POKEMON RUBY"),
        ("AWRE", "ADVANCEWARS"),
        ("AW2E", "ADVANCEWARS2"),
        ("2ATE", "APOTRIS"),
        ("SLTE", "SLOT TEST"),
        ("A88E", "MARIO&LUIGIRPG"),
        ("", ""),
    ] {
        let own = link_kind(code, title, true) == LinkKind::Wireless
            || serial_option(LinkKind::Cable, LinkKind::Wireless, code, title) != "auto";
        assert_eq!(link_carried(code, title), own, "{code} {title}");
    }
}
