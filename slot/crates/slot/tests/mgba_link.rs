mod common;

use std::path::{Path, PathBuf};

use slot_retro::{ButtonMask, LibretroCore, RetroCore, GBA_H, GBA_W};

fn vendored() -> Option<PathBuf> {
    let dylib = common::vendored_core();
    if dylib.is_none() {
        eprintln!("no vendored mGBA core on this host, skipping");
    }
    dylib
}

fn rom(name: &str, bytes: Vec<u8>) -> PathBuf {
    let p = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    std::fs::write(&p, bytes).expect("write rom");
    p
}

fn keys_rom() -> Vec<u8> {
    let mut rom = common::gba_rom();
    let mut set = |index: usize, word: u32| {
        let o = 0xc0 + index * 4;
        rom[o..o + 4].copy_from_slice(&word.to_le_bytes());
    };
    set(5, 0xe2805c01);
    set(9, 0xe1d533b0);
    rom
}

fn sio_rom() -> Vec<u8> {
    let mut rom = common::gba_rom();
    let mut set = |index: usize, word: u32| {
        let o = 0xc0 + index * 4;
        rom[o..o + 4].copy_from_slice(&word.to_le_bytes());
    };
    set(5, 0xea000008);
    set(9, 0xea000009);
    set(15, 0xe2805c01);
    set(16, 0xe3a06000);
    set(17, 0xe1c563b4);
    set(18, 0xe3a06a02);
    set(19, 0xeafffff1);
    set(20, 0xe1c562b8);
    set(21, 0xe1d532b8);
    set(22, 0xeafffff2);
    rom
}

fn transfer_rom() -> Vec<u8> {
    let mut rom = common::gba_rom();
    let mut set = |index: usize, word: u32| {
        let o = 0xc0 + index * 4;
        rom[o..o + 4].copy_from_slice(&word.to_le_bytes());
    };
    set(5, 0xea000008);
    set(9, 0xea00000b);
    set(15, 0xe2805c01);
    set(16, 0xe3a06000);
    set(17, 0xe1c563b4);
    set(18, 0xe3a06a02);
    set(19, 0xe1c562b8);
    set(20, 0xe3866080);
    set(21, 0xeaffffef);
    set(22, 0xe1c562b8);
    set(23, 0xe1d533b0);
    set(24, 0xeafffff0);
    rom
}

fn dma_rom() -> Vec<u8> {
    let mut rom = common::gba_rom();
    let mut set = |index: usize, word: u32| {
        let o = 0xc0 + index * 4;
        rom[o..o + 4].copy_from_slice(&word.to_le_bytes());
    };
    set(5, 0xea000008);
    set(9, 0xe1d533b0);
    set(15, 0xe2805c01);
    set(16, 0xe3a06408);
    set(17, 0xe58060d4);
    set(18, 0xe3a06402);
    set(19, 0xe58060d8);
    set(20, 0xe3a06484);
    set(21, 0xe58060dc);
    set(22, 0xeaffffee);
    rom
}

fn multiplayer_rom() -> Vec<u8> {
    let mut rom = common::gba_rom();
    let mut set = |index: usize, word: u32| {
        let o = 0xc0 + index * 4;
        rom[o..o + 4].copy_from_slice(&word.to_le_bytes());
    };
    set(5, 0xea000008);
    set(9, 0xea00000a);
    set(15, 0xe2805c01);
    set(16, 0xe3a06000);
    set(17, 0xe1c563b4);
    set(18, 0xe3a06a02);
    set(19, 0xe1c562b8);
    set(20, 0xeafffff0);
    set(21, 0xe1d532b8);
    set(22, 0xe1c230b2);
    set(23, 0xe1d532b0);
    set(24, 0xe1c230b4);
    set(25, 0xe1d532b2);
    set(26, 0xe1c230b6);
    set(27, 0xe1c562b8);
    set(28, 0xe1d532b8);
    set(29, 0xe2033030);
    set(30, 0xe3833a01);
    set(31, 0xe1c532ba);
    set(32, 0xe3863080);
    set(33, 0xe1c532b8);
    set(34, 0xeaffffe7);
    rom
}

fn probe_rom() -> Vec<u8> {
    let mut rom = transfer_rom();
    let mut set = |index: usize, word: u32| {
        let o = 0xc0 + index * 4;
        rom[o..o + 4].copy_from_slice(&word.to_le_bytes());
    };
    set(7, 0xe354008c);
    set(12, 0xe354008c);
    set(22, 0xe1d543b0);
    set(23, 0xe3140001);
    set(24, 0x01c562ba);
    set(25, 0xe1d532b2);
    set(26, 0xe1c562b8);
    set(27, 0xeaffffed);
    rom
}

fn dma_a_rom() -> Vec<u8> {
    let mut rom = dma_rom();
    let mut set = |index: usize, word: u32| {
        let o = 0xc0 + index * 4;
        rom[o..o + 4].copy_from_slice(&word.to_le_bytes());
    };
    set(10, 0xea00000b);
    set(21, 0xeaffffef);
    set(23, 0xe3130001);
    set(24, 0x058060dc);
    set(25, 0xe1c230b0);
    set(26, 0xeaffffef);
    rom
}

fn normal_rom() -> Vec<u8> {
    let mut rom = common::gba_rom();
    let mut set = |index: usize, word: u32| {
        let o = 0xc0 + index * 4;
        rom[o..o + 4].copy_from_slice(&word.to_le_bytes());
    };
    set(5, 0xea000008);
    set(9, 0xea00000b);
    set(15, 0xe2805c01);
    set(16, 0xe3a06000);
    set(17, 0xe1c563b4);
    set(18, 0xe1c562b8);
    set(19, 0xe3a06001);
    set(20, 0xeafffff0);
    set(22, 0xe1c562ba);
    set(23, 0xe1d533b0);
    set(24, 0xeafffff0);
    rom
}

fn painted(picture: &[u8], x: usize) -> u16 {
    let pixel = &picture[x * 4..x * 4 + 4];
    u16::from(pixel[2] >> 3) | u16::from(pixel[1] >> 3) << 5 | u16::from(pixel[0] >> 3) << 10
}

#[test]
fn a_pair_restored_in_multiplayer_mode_talks_from_the_first_transfer() {
    let _g = common::core_lock();
    let Some(dylib) = vendored() else { return };
    let rom = rom("mgba-link-multiplayer.gba", multiplayer_rom());

    let alone = single_state(&dylib, &rom, 20);
    let mut pair = link_core(&dylib, 0);
    pair.load(&rom).expect("link mode refused the rom");
    for _ in 0..30 {
        pair.run_frame_linked(ButtonMask::default(), ButtonMask::default());
    }
    let transferring = pair.serialize().expect("no link state");
    drop(pair);

    let mut deaf = Vec::new();
    for (what, container) in [
        ("two one-GBA states", slk1([&alone, &alone])),
        ("a link state of a pair that was transferring", transferring),
    ] {
        let pictures = linked_pictures(&dylib, &rom, Some(&container), 10, |_| {
            ButtonMask::default()
        });
        for (player, frames) in pictures.iter().enumerate() {
            let heard = frames.iter().enumerate().skip(1).find_map(|(frame, picture)| {
                let (siocnt, from_0, from_1) =
                    (painted(picture, 1), painted(picture, 2), painted(picture, 3));
                (siocnt & 0x0008 == 0 || from_0 != 0x1000 || from_1 != 0x1010).then(|| {
                    format!(
                        "{what}: player {player}'s GBA, first on frame {frame}: SIOCNT {siocnt:04x}, \
                         SIOMULTI0 {from_0:04x}, SIOMULTI1 {from_1:04x}"
                    )
                })
            });
            deaf.extend(heard);
        }
    }
    assert!(
        deaf.is_empty(),
        "a pair restored in multiplayer mode was not ready or did not hear each other: {deaf:#?}"
    );
}

fn single_core(dylib: &Path) -> LibretroCore {
    LibretroCore::open(dylib).expect("vendored core is present but would not open")
}

fn link_core(dylib: &Path, player: u8) -> LibretroCore {
    let mut core = single_core(dylib);
    core.set_option("mgba_link", "on");
    core.set_option("mgba_link_player", &player.to_string());
    core
}

fn slk1(states: [&[u8]; 2]) -> Vec<u8> {
    let mut out = b"SLK1".to_vec();
    for state in states {
        out.extend_from_slice(&(state.len() as u32).to_le_bytes());
        out.extend_from_slice(state);
    }
    out
}

fn split_slk1(container: &[u8]) -> [Vec<u8>; 2] {
    assert_eq!(
        &container[..4],
        b"SLK1",
        "not a link state. A vendored core built before link mode ignores the option: run `task core`"
    );
    let mut rest = &container[4..];
    let mut take = || {
        let len = u32::from_le_bytes(rest[..4].try_into().unwrap()) as usize;
        let state = rest[4..4 + len].to_vec();
        rest = &rest[4 + len..];
        state
    };
    let states = [take(), take()];
    assert!(
        rest.is_empty(),
        "{} bytes after player 1's state",
        rest.len()
    );
    states
}

fn single_picture(dylib: &Path, rom: &Path, keys: u16, frames: usize) -> Vec<u8> {
    let mut core = single_core(dylib);
    core.load(rom).expect("load");
    for _ in 0..frames {
        core.run_frame(ButtonMask(keys));
    }
    core.video_xrgb8888().to_vec()
}

#[test]
fn link_mode_shows_the_local_players_gba_holding_its_own_port() {
    let _g = common::core_lock();
    let Some(dylib) = vendored() else { return };
    let rom = rom("mgba-link-keys.gba", keys_rom());

    let a = single_picture(&dylib, &rom, ButtonMask::A, 10);
    let b = single_picture(&dylib, &rom, ButtonMask::B, 10);
    assert_ne!(a, b, "the rom paints the same picture whatever is held");

    for (player, want) in [a, b].iter().enumerate() {
        let mut core = link_core(&dylib, player as u8);
        core.load(&rom).expect("link mode refused the rom");
        for _ in 0..10 {
            core.run_frame_linked(ButtonMask(ButtonMask::A), ButtonMask(ButtonMask::B));
        }
        assert!(
            core.video_xrgb8888() == want.as_slice(),
            "mgba_link_player={player} did not show player {player}'s GBA holding port {player}'s \
             buttons. A vendored core built before link mode ignores the option: run `task core`"
        );
    }
}

#[test]
fn link_mode_saves_both_gbas_in_one_link_state() {
    let _g = common::core_lock();
    let Some(dylib) = vendored() else { return };
    let rom = rom("mgba-link-state.gba", common::gba_rom());

    let mut core = link_core(&dylib, 0);
    core.load(&rom).expect("link mode refused the rom");
    for _ in 0..30 {
        core.run_frame_linked(ButtonMask::default(), ButtonMask::default());
    }
    let state = core.serialize().expect("no link state");
    for (player, gba) in split_slk1(&state).iter().enumerate() {
        assert!(
            gba.len() > 100_000,
            "player {player}'s GBA state is {} bytes",
            gba.len()
        );
    }
}

#[test]
fn link_mode_off_keeps_the_single_gba_state() {
    let _g = common::core_lock();
    let Some(dylib) = vendored() else { return };
    let rom = rom("mgba-link-off.gba", common::gba_rom());

    let mut core = single_core(&dylib);
    core.set_option("mgba_link", "off");
    core.load(&rom).expect("load");
    core.run_frame(ButtonMask::default());
    let state = core.serialize().expect("no state");
    assert_ne!(&state[..4], b"SLK1");
    assert!(state.len() > 100_000, "state is {} bytes", state.len());
}

#[test]
fn a_link_state_of_two_single_gba_states_restores_each_player_where_they_were() {
    let _g = common::core_lock();
    let Some(dylib) = vendored() else { return };
    let rom = rom("mgba-link-restore.gba", common::gba_rom());

    let mut single = single_core(&dylib);
    single.load(&rom).expect("load");
    let mut states = Vec::new();
    for frames in [10, 40] {
        for _ in 0..frames {
            single.run_frame(ButtonMask::default());
        }
        states.push(single.serialize().expect("no state"));
    }
    drop(single);

    let mut alone = Vec::new();
    for state in &states {
        let mut core = single_core(&dylib);
        core.load(&rom).expect("load");
        core.unserialize(state)
            .expect("the single core refused its own state");
        for _ in 0..5 {
            core.run_frame(ButtonMask::default());
        }
        alone.push(core.video_xrgb8888().to_vec());
    }
    assert_ne!(alone[0], alone[1], "the two moments paint the same picture");

    let container = slk1([&states[0], &states[1]]);
    for (player, want) in alone.iter().enumerate() {
        let mut core = link_core(&dylib, player as u8);
        core.load(&rom).expect("link mode refused the rom");
        core.unserialize(&container)
            .expect("link mode refused the link state");
        for _ in 0..5 {
            core.run_frame_linked(ButtonMask::default(), ButtonMask::default());
        }
        assert!(
            core.video_xrgb8888() == want.as_slice(),
            "player {player}'s GBA did not carry on from player {player}'s state"
        );
    }
}

#[test]
fn link_mode_refuses_anything_but_a_whole_link_state() {
    let _g = common::core_lock();
    let Some(dylib) = vendored() else { return };
    let rom = rom("mgba-link-malformed.gba", common::gba_rom());

    let mut core = link_core(&dylib, 0);
    core.load(&rom).expect("link mode refused the rom");
    core.run_frame_linked(ButtonMask::default(), ButtonMask::default());
    let good = core.serialize().expect("no link state");
    let [player_0, _] = split_slk1(&good);

    let mut wrong_magic = good.clone();
    wrong_magic[3] = b'2';
    let mut long_length = good.clone();
    long_length[4..8].copy_from_slice(&(good.len() as u32).to_le_bytes());
    let mut trailing = good.clone();
    trailing.push(0);
    let empty = slk1([&[], &[]]);
    let short = slk1([&player_0, &player_0[..100]]);

    for (what, bytes) in [
        ("a one-GBA state", player_0.as_slice()),
        ("a different magic", wrong_magic.as_slice()),
        ("a length past the end", long_length.as_slice()),
        ("bytes after player 1", trailing.as_slice()),
        ("the magic alone", b"SLK1".as_slice()),
        ("player 1 missing", &good[..8 + player_0.len()]),
        ("two empty states", empty.as_slice()),
        ("a player 1 state too short to be one", short.as_slice()),
    ] {
        assert!(core.unserialize(bytes).is_err(), "link mode took {what}");
    }
    core.unserialize(&good)
        .expect("link mode refused its own link state");
}

#[test]
fn a_link_state_the_core_refuses_leaves_both_gbas_where_they_were() {
    let _g = common::core_lock();
    let Some(dylib) = vendored() else { return };
    let rom = rom("mgba-link-rollback.gba", common::gba_rom());

    let mut single = single_core(&dylib);
    single.load(&rom).expect("load");
    for _ in 0..10 {
        single.run_frame(ButtonMask::default());
    }
    let early = single.serialize().expect("no state");
    drop(single);
    let mut refused = early.clone();
    refused[..4].copy_from_slice(&u32::MAX.to_le_bytes());

    let mut control = link_core(&dylib, 0);
    control.load(&rom).expect("link mode refused the rom");
    for _ in 0..60 {
        control.run_frame_linked(ButtonMask::default(), ButtonMask::default());
    }
    let here = control.serialize().expect("no link state");
    drop(control);
    let mut control = link_core(&dylib, 0);
    control.load(&rom).expect("link mode refused the rom");
    control
        .unserialize(&here)
        .expect("link mode refused its own link state");
    for _ in 0..5 {
        control.run_frame_linked(ButtonMask::default(), ButtonMask::default());
    }
    let want = control.video_xrgb8888().to_vec();
    drop(control);

    let mut core = link_core(&dylib, 0);
    core.load(&rom).expect("link mode refused the rom");
    core.unserialize(&here)
        .expect("link mode refused its own link state");
    assert!(
        core.unserialize(&slk1([&early, &refused])).is_err(),
        "link mode took a state the core refuses"
    );
    for _ in 0..5 {
        core.run_frame_linked(ButtonMask::default(), ButtonMask::default());
    }
    assert!(
        core.video_xrgb8888() == want.as_slice(),
        "a refused restore left player 0's GBA restored instead of where it was"
    );
}

fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, &byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

fn script(frame: usize) -> (ButtonMask, ButtonMask) {
    let p0 = if frame.is_multiple_of(3) {
        ButtonMask::A
    } else {
        ButtonMask::RIGHT
    };
    let p1 = if frame % 5 < 2 {
        ButtonMask::B | ButtonMask::L
    } else {
        0
    };
    (ButtonMask(p0), ButtonMask(p1))
}

#[test]
fn both_players_devices_compute_the_same_machines() {
    let _g = common::core_lock();
    let Some(dylib) = vendored() else { return };
    let rom = rom("mgba-link-lockstep.gba", keys_rom());

    let mut single = single_core(&dylib);
    single.load(&rom).expect("load");
    let mut starts = Vec::new();
    for frames in [20, 45] {
        for _ in 0..frames {
            single.run_frame(ButtonMask::default());
        }
        starts.push(single.serialize().expect("no state"));
    }
    drop(single);
    let container = slk1([&starts[0], &starts[1]]);

    let mut hashes = Vec::new();
    for player in [0u8, 1, 0] {
        let mut core = link_core(&dylib, player);
        core.load(&rom).expect("link mode refused the rom");
        core.unserialize(&container)
            .expect("link mode refused the link state");
        for frame in 0..600 {
            let (p0, p1) = script(frame);
            core.run_frame_linked(p0, p1);
        }
        hashes.push(fnv1a(&core.serialize().expect("no link state")));
    }
    assert_eq!(
        hashes[0], hashes[2],
        "the same device computed two different machines"
    );
    assert_eq!(
        hashes[0], hashes[1],
        "player 0's and player 1's devices computed different machines"
    );
}

fn shared_script(frame: usize) -> ButtonMask {
    let mut keys = 0;
    if (10..40).contains(&frame) {
        keys |= ButtonMask::A;
    }
    if (50..53).contains(&frame) || frame == 70 {
        keys |= ButtonMask::B;
    }
    if (80..120).contains(&frame) && frame % 7 < 3 {
        keys |= ButtonMask::RIGHT;
    }
    ButtonMask(keys)
}

fn dma_script(frame: usize) -> ButtonMask {
    let mut keys = 0;
    if frame == 3 {
        keys |= ButtonMask::A;
    }
    if (60..63).contains(&frame) || frame == 70 {
        keys |= ButtonMask::RIGHT;
    }
    if (80..84).contains(&frame) {
        keys |= ButtonMask::B;
    }
    ButtonMask(keys)
}

fn linked_pictures(
    dylib: &Path,
    rom: &Path,
    container: Option<&[u8]>,
    frames: usize,
    buttons: fn(usize) -> ButtonMask,
) -> [Vec<Vec<u8>>; 2] {
    [0u8, 1].map(|player| {
        let mut core = link_core(dylib, player);
        core.load(rom).expect("link mode refused the rom");
        if let Some(container) = container {
            core.unserialize(container)
                .expect("link mode refused the link state");
        }
        (0..frames)
            .map(|frame| {
                let keys = buttons(frame);
                core.run_frame_linked(keys, keys);
                core.video_xrgb8888().to_vec()
            })
            .collect()
    })
}

fn differing_frames(pictures: &[Vec<Vec<u8>>; 2]) -> Vec<usize> {
    (0..pictures[0].len())
        .filter(|&frame| pictures[0][frame] != pictures[1][frame])
        .collect()
}

fn single_state(dylib: &Path, rom: &Path, frames: usize) -> Vec<u8> {
    let mut single = single_core(dylib);
    single.load(rom).expect("load");
    for _ in 0..frames {
        single.run_frame(ButtonMask::default());
    }
    single.serialize().expect("no state")
}

#[test]
fn two_identical_gbas_read_the_same_buttons_on_the_same_frame() {
    let _g = common::core_lock();
    let Some(dylib) = vendored() else { return };
    let rom = rom("mgba-link-same-buttons.gba", keys_rom());

    let state = single_state(&dylib, &rom, 20);
    let pictures = linked_pictures(
        &dylib,
        &rom,
        Some(&slk1([&state, &state])),
        120,
        shared_script,
    );
    let differing = differing_frames(&pictures);
    assert!(
        differing.is_empty(),
        "two identical GBAs given the same buttons painted different buttons, first on frame {:?} \
         (all: {differing:?})",
        differing.first()
    );
}

type Start = (
    &'static str,
    PathBuf,
    Option<Vec<u8>>,
    fn(usize) -> ButtonMask,
);

fn every_start(dylib: &Path) -> Vec<Start> {
    let transfers = rom("mgba-link-starts.gba", transfer_rom());
    let blocked = rom("mgba-link-starts-dma.gba", dma_rom());
    let blocked_late = rom("mgba-link-starts-dma-a.gba", dma_a_rom());
    let normal = rom("mgba-link-starts-normal.gba", normal_rom());
    let end = single_state(dylib, &transfers, 20);
    let reset = single_state(dylib, &transfers, 0);
    let normal_end = single_state(dylib, &normal, 20);
    vec![
        ("a fresh load", transfers.clone(), None, shared_script),
        (
            "[end, end]",
            transfers.clone(),
            Some(slk1([&end, &end])),
            shared_script,
        ),
        (
            "[end, reset]",
            transfers.clone(),
            Some(slk1([&end, &reset])),
            shared_script,
        ),
        (
            "[reset, end]",
            transfers.clone(),
            Some(slk1([&reset, &end])),
            shared_script,
        ),
        (
            "[reset, reset]",
            transfers,
            Some(slk1([&reset, &reset])),
            shared_script,
        ),
        (
            "a fresh load blocked by a DMA",
            blocked,
            None,
            shared_script,
        ),
        (
            "a DMA started mid-session, on frame 3",
            blocked_late,
            None,
            dma_script,
        ),
        (
            "[end, end] in normal serial mode",
            normal,
            Some(slk1([&normal_end, &normal_end])),
            shared_script,
        ),
    ]
}

fn next_frame_end(gba: &[u8]) -> u32 {
    let u32_at = |at: usize| u32::from_le_bytes(gba[at..at + 4].try_into().unwrap());
    let u16_at = |at: usize| u16::from_le_bytes(gba[at..at + 2].try_into().unwrap());
    let mut header = 0x61000;
    let driver = loop {
        let tag = u32_at(header);
        assert_ne!(tag, 0, "a GBA in a link state has no lockstep driver state");
        if tag == 0x41 {
            break u32_at(header + 8) as usize + 4;
        }
        header += 16;
    };
    let clock = u32_at(0x0c)
        .wrapping_add(u32_at(0x68))
        .wrapping_sub(u32_at(driver + 0x34));
    let lines = (159 + 228 - u32::from(u16_at(0x406))) % 228;
    let hblank = if u16_at(0x404) & 2 == 0 { 224 } else { 0 };
    clock
        .wrapping_add(u32_at(0x1f4))
        .wrapping_add(hblank + lines * 1232)
}

#[test]
fn every_start_gives_both_gbas_the_same_buttons_on_the_same_frame() {
    let _g = common::core_lock();
    let Some(dylib) = vendored() else { return };

    let mut late = Vec::new();
    for (what, rom, container, buttons) in every_start(&dylib) {
        let pictures = linked_pictures(&dylib, &rom, container.as_deref(), 120, buttons);
        let differing = differing_frames(&pictures);
        if !differing.is_empty() {
            late.push(format!("{what}: frames {differing:?}"));
        }
    }
    assert!(
        late.is_empty(),
        "two GBAs given the same buttons painted different buttons: {late:#?}"
    );
}

#[test]
fn every_start_joins_the_two_gbas_with_their_frames_ending_together() {
    let _g = common::core_lock();
    let Some(dylib) = vendored() else { return };

    let mut apart = Vec::new();
    for (what, rom, container, buttons) in every_start(&dylib) {
        let mut core = link_core(&dylib, 0);
        core.load(&rom).expect("link mode refused the rom");
        if let Some(container) = &container {
            core.unserialize(container)
                .expect("link mode refused the link state");
        }
        for frame in 0..120 {
            let keys = buttons(frame);
            core.run_frame_linked(keys, keys);
        }
        let [player_0, player_1] = split_slk1(&core.serialize().expect("no link state"));
        let gap = next_frame_end(&player_1).wrapping_sub(next_frame_end(&player_0)) as i32;
        if gap.unsigned_abs() >= 256 {
            apart.push(format!(
                "{what}: player 1's next frame ends {gap} cycles after player 0's"
            ));
        }
    }
    assert!(
        apart.is_empty(),
        "the two GBAs' frames do not end together: {apart:#?}"
    );
}

#[test]
fn a_restore_does_not_depend_on_what_the_cores_ran_before_it() {
    let _g = common::core_lock();
    let Some(dylib) = vendored() else { return };
    let rom = rom("mgba-link-probe.gba", probe_rom());

    let alone = single_state(&dylib, &rom, 20);
    let container = slk1([&alone, &alone]);

    let mut seen = Vec::new();
    for player in [0u8, 1] {
        for ran_first in [false, true] {
            let mut core = link_core(&dylib, player);
            core.load(&rom).expect("link mode refused the rom");
            if ran_first {
                let a = ButtonMask(ButtonMask::A);
                for _ in 0..30 {
                    core.run_frame_linked(a, a);
                }
            }
            core.unserialize(&container)
                .expect("link mode refused the link state");
            let at_restore = fnv1a(&core.serialize().expect("no link state"));
            for _ in 0..10 {
                core.run_frame_linked(ButtonMask::default(), ButtonMask::default());
            }
            let ten_frames_on = fnv1a(&core.serialize().expect("no link state"));
            let cores = if ran_first {
                "cores that ran 30 linked frames first"
            } else {
                "freshly loaded cores"
            };
            seen.push((
                format!("player {player}'s device, {cores}"),
                at_restore,
                ten_frames_on,
            ));
        }
    }

    let (_, want_at_restore, want_ten_on) = &seen[0];
    let differing: Vec<String> = seen
        .iter()
        .filter(|(_, at_restore, ten_on)| at_restore != want_at_restore || ten_on != want_ten_on)
        .map(|(what, at_restore, ten_on)| {
            format!("{what}: {at_restore:016x} at the restore, {ten_on:016x} ten frames on")
        })
        .collect();
    assert!(
        differing.is_empty(),
        "the same link state computed different machines depending on what the cores ran before \
         it. Wanted {want_at_restore:016x} then {want_ten_on:016x}, as {} gave: {differing:#?}",
        seen[0].0
    );
}

fn race_script(frame: usize) -> ButtonMask {
    let mut keys = 0;
    if (1500..=1506).contains(&frame) {
        keys |= ButtonMask::DOWN;
    }
    if (1560..=1566).contains(&frame) || (frame >= 4300 && (frame - 4300) % 30 <= 3) {
        keys |= ButtonMask::A;
    }
    ButtonMask(keys)
}

fn write_ppm(path: &Path, xrgb: &[u8]) {
    let mut out = format!("P6\n{GBA_W} {GBA_H}\n255\n").into_bytes();
    for pixel in xrgb.chunks(4) {
        out.extend_from_slice(&[pixel[2], pixel[1], pixel[0]]);
    }
    std::fs::write(path, out).expect("write picture");
}

#[test]
#[ignore]
fn mario_kart_super_circuit_is_the_same_race_on_both_devices() {
    let _g = common::core_lock();
    let dylib = common::vendored_core().expect("no vendored mGBA core: run `task core`");
    let rom = PathBuf::from(
        std::env::var_os("SLOT_MKSC_ROM")
            .expect("set SLOT_MKSC_ROM to a Mario Kart: Super Circuit ROM"),
    );

    let mut hashes = Vec::new();
    for player in [0u8, 1] {
        let mut core = link_core(&dylib, player);
        core.load(&rom).expect("link mode refused Mario Kart");
        let started = std::time::Instant::now();
        for frame in 1..=25_000 {
            let keys = race_script(frame);
            core.run_frame_linked(keys, keys);
        }
        let secs = started.elapsed().as_secs_f64();
        eprintln!(
            "mgba_link_player={player}: 25000 frame pairs in {secs:.1} s, {:.0} pairs/s",
            25_000.0 / secs
        );
        let picture =
            Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("mksc-player{player}.ppm"));
        write_ppm(&picture, core.video_xrgb8888());
        eprintln!("last picture: {}", picture.display());
        hashes.push(fnv1a(&core.serialize().expect("no link state")));
    }
    assert_eq!(
        hashes[0], hashes[1],
        "player 0's and player 1's devices computed different races"
    );
}

#[test]
#[ignore]
fn mario_kart_super_circuit_races_linked_after_a_restore() {
    let _g = common::core_lock();
    let dylib = common::vendored_core().expect("no vendored mGBA core: run `task core`");
    let rom = PathBuf::from(
        std::env::var_os("SLOT_MKSC_ROM")
            .expect("set SLOT_MKSC_ROM to a Mario Kart: Super Circuit ROM"),
    );

    let raced: Vec<Vec<u8>> = [0u8, 1]
        .into_iter()
        .map(|player| {
            let mut boot = link_core(&dylib, player);
            boot.load(&rom).expect("link mode refused Mario Kart");
            for frame in 1..=25_000 {
                let keys = race_script(frame);
                boot.run_frame_linked(keys, keys);
            }
            boot.video_xrgb8888().to_vec()
        })
        .collect();

    let mut single = single_core(&dylib);
    single.load(&rom).expect("mGBA refused Mario Kart");
    for frame in 1..=1400 {
        single.run_frame(race_script(frame));
    }
    let title = single.serialize().expect("no state");
    drop(single);
    let container = slk1([&title, &title]);

    let mut hashes = Vec::new();
    let mut pictures = Vec::new();
    for player in [0u8, 1] {
        let mut core = link_core(&dylib, player);
        core.load(&rom).expect("link mode refused Mario Kart");
        core.unserialize(&container)
            .expect("link mode refused the title-screen link state");
        let started = std::time::Instant::now();
        for frame in 1401..=25_000 {
            let keys = race_script(frame);
            core.run_frame_linked(keys, keys);
        }
        let secs = started.elapsed().as_secs_f64();
        eprintln!(
            "restored, mgba_link_player={player}: 23600 frame pairs in {secs:.1} s, {:.0} pairs/s",
            23_600.0 / secs
        );
        let picture = Path::new(env!("CARGO_TARGET_TMPDIR"))
            .join(format!("mksc-restored-player{player}.ppm"));
        write_ppm(&picture, core.video_xrgb8888());
        eprintln!("last picture: {}", picture.display());
        pictures.push(core.video_xrgb8888().to_vec());
        hashes.push(fnv1a(&core.serialize().expect("no link state")));
    }
    assert_eq!(
        hashes[0], hashes[1],
        "player 0's and player 1's devices computed different machines after a restore"
    );
    for (player, (restored, booted)) in pictures.iter().zip(&raced).enumerate() {
        assert!(
            restored == booted,
            "restored at the title screen, player {player}'s device is not showing the race a \
             reset boot shows at frame 25,000"
        );
    }
}

#[test]
fn the_cable_is_plugged_in_after_a_load_and_after_a_restore() {
    let _g = common::core_lock();
    let Some(dylib) = vendored() else { return };
    let rom = rom("mgba-link-sio.gba", sio_rom());

    let alone = single_picture(&dylib, &rom, 0, 30);

    let pictures = |container: Option<&[u8]>| -> Vec<Vec<u8>> {
        (0..2u8)
            .map(|player| {
                let mut core = link_core(&dylib, player);
                core.load(&rom).expect("link mode refused the rom");
                if let Some(container) = container {
                    core.unserialize(container)
                        .expect("link mode refused the link state");
                }
                for _ in 0..30 {
                    core.run_frame_linked(ButtonMask::default(), ButtonMask::default());
                }
                core.video_xrgb8888().to_vec()
            })
            .collect()
    };

    let loaded = pictures(None);
    assert!(
        loaded[1] != loaded[0],
        "after a load, player 1's GBA read the same SIOCNT as player 0's: no cable"
    );
    assert!(
        loaded[1] != alone,
        "after a load, player 1's GBA read what a lone GBA reads: no cable"
    );

    let mut single = single_core(&dylib);
    single.load(&rom).expect("load");
    for _ in 0..30 {
        single.run_frame(ButtonMask::default());
    }
    let state = single.serialize().expect("no state");
    drop(single);

    let restored = pictures(Some(&slk1([&state, &state])));
    assert!(
        restored[1] != restored[0],
        "after a restore, player 1's GBA read the same SIOCNT as player 0's: no cable"
    );
    assert!(
        restored[1] != alone,
        "after a restore, player 1's GBA read what a lone GBA reads: no cable"
    );
}

#[test]
fn a_link_states_travels_between_two_link_mode_cores() {
    let _g = common::core_lock();
    let Some(dylib) = vendored() else { return };
    let rom = rom("mgba-link-multiplayer.gba", multiplayer_rom());

    let mut host = link_core(&dylib, 0);
    host.load(&rom).expect("link mode refused the rom");
    let state = host.serialize().expect("the host would not serialize");
    assert!(
        state.starts_with(b"SLK1"),
        "the host produced something that is not a link state"
    );
    drop(host);

    let mut joiner = link_core(&dylib, 1);
    joiner.load(&rom).expect("link mode refused the rom");
    joiner
        .unserialize(&state)
        .expect("a link-mode core refused a link state from its own build");
}

#[test]
fn a_single_core_refuses_a_link_state() {
    let _g = common::core_lock();
    let Some(dylib) = vendored() else { return };
    let rom = rom("mgba-link-multiplayer.gba", multiplayer_rom());

    let mut host = link_core(&dylib, 0);
    host.load(&rom).expect("link mode refused the rom");
    let state = host.serialize().expect("the host would not serialize");
    drop(host);

    let mut single = single_core(&dylib);
    single.load(&rom).expect("load");
    assert!(
        single.unserialize(&state).is_err(),
        "a single GBA took a state holding two"
    );
}

#[test]
fn the_card_cart_link_state_travels_between_two_link_mode_cores() {
    let _g = common::core_lock();
    let Some(dylib) = vendored() else { return };
    let card = common::repo_root().join("sdcard/Games/GBA/Advance Wars.gba");
    let sav = common::repo_root().join("sdcard/Saves/GBA/Advance Wars.sav");
    if !card.exists() {
        eprintln!("no card cart on this machine, skipping");
        return;
    }
    let save = std::fs::read(&sav).ok();

    let mut host = link_core(&dylib, 0);
    host.load(&card).expect("link mode refused the cart");
    if let Some(s) = &save {
        host.load_save_ram(s)
            .expect("the host refused its own save");
    }
    let state = host.serialize().expect("the host would not serialize");
    eprintln!("host state {} bytes", state.len());
    drop(host);

    let mut joiner = link_core(&dylib, 1);
    joiner.load(&card).expect("link mode refused the cart");
    if let Some(s) = &save {
        joiner
            .load_save_ram(s)
            .expect("the joiner refused its own save");
    }
    joiner
        .unserialize(&state)
        .expect("the joiner refused the host's link state");
}

#[test]
fn skipping_the_peers_picture_and_sound_does_not_change_the_machine() {
    let _g = common::core_lock();
    let Some(dylib) = vendored() else { return };
    let rom = rom("mgba-link-multiplayer.gba", multiplayer_rom());
    let script = [
        ButtonMask(ButtonMask::A),
        ButtonMask(ButtonMask::B),
        ButtonMask::default(),
        ButtonMask(ButtonMask::A | ButtonMask::B),
    ];

    let run = |player: u8| {
        let mut core = link_core(&dylib, player);
        core.load(&rom).expect("link mode refused the rom");
        for i in 0..240 {
            core.run_frame_linked(script[i % script.len()], script[(i + 1) % script.len()]);
        }
        core.serialize().expect("no link state")
    };

    let as_host = run(0);
    let as_joiner = run(1);
    assert_eq!(
        as_host.len(),
        as_joiner.len(),
        "the two ends produced link states of different sizes"
    );
    assert_eq!(
        as_host, as_joiner,
        "the two ends ran the same inputs to different machines: skipping the peer's picture is \
         not state-safe, and two devices would drift apart in a race"
    );
}

fn ready(emu: &slot::emu::EmuHandle) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while emu.state() == slot::emu::CoreState::Loading {
        assert!(
            std::time::Instant::now() < deadline,
            "the core never settled"
        );
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    assert_eq!(emu.state(), slot::emu::CoreState::Ready);
}

#[test]
fn a_link_mode_worker_resumes_a_one_gba_state() {
    use slot::audio::AudioSink;
    use slot::persist::Snapshot;
    let _g = common::core_lock();
    let Some(dylib) = vendored() else { return };
    let rom = rom("mgba-link-resume.gba", common::gba_rom());
    let one = single_state(&dylib, &rom, 30);

    let emu = slot::emu::EmuHandle::spawn_linked(
        Box::new(link_core(&dylib, 0)),
        rom,
        slot::audio::StubSink::new().ring(),
        None,
        Some(one),
        0,
    );
    ready(&emu);
    assert!(
        emu.snapshot().resume_trusted(),
        "link mode refused the one-GBA resume"
    );
}

#[test]
fn a_link_mode_workers_state_resumes_on_a_single_core() {
    use slot::audio::AudioSink;
    use slot::persist::Snapshot;
    let _g = common::core_lock();
    let Some(dylib) = vendored() else { return };
    let rom = rom("mgba-link-leave.gba", common::gba_rom());

    let emu = slot::emu::EmuHandle::spawn_linked(
        Box::new(link_core(&dylib, 1)),
        rom.clone(),
        slot::audio::StubSink::new().ring(),
        None,
        None,
        1,
    );
    ready(&emu);
    let state = emu.snapshot().state().expect("no state");
    drop(emu);
    assert_ne!(&state[..4], b"SLK1");

    let mut single = single_core(&dylib);
    single.load(&rom).expect("load");
    single
        .unserialize(&state)
        .expect("a single GBA refused the link-mode worker's state");
}
