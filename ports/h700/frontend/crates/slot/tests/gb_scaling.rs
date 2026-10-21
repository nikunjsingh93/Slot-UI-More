mod common;
use slot::app::Phase;
use slot::video_mode::{fills_height, source_rect, video_mode_for, width_fraction, write_video_mode, VideoMode};
use slot_gfx::WHOLE_TEXTURE;
use slot_input::{Action, Btn};
use slot_store::Platform;

#[test]
fn l1_cycles_full_screen_and_aspect_fit_r1_resets_and_each_cart_keeps_its_choice() {
    for platform in [Platform::Gb, Platform::Gbc] {
        let d = common::tmp_root_with_gb_carts(&["First", "Second"]);
        let mut a = common::app_playing_in(d.path(), "First");
        a.set_platform(platform);
        assert!(matches!(a.phase(), Phase::Playing { .. }));
        for mode in [VideoMode::Stretch, VideoMode::HeightFit, VideoMode::Stretch, VideoMode::HeightFit] {
            a.apply(Action::GbaDown(Btn::L1));
            a.apply(Action::GbaUp(Btn::L1));
            assert_eq!(video_mode_for(d.path(), "First"), mode);
            assert_eq!(video_mode_for(d.path(), "Second"), VideoMode::Actual);
            assert!(a.picture_fills_height());
            assert_eq!(a.picture_width(), width_fraction(platform, mode));
            assert_eq!(a.source_rect(), source_rect(platform, mode));
        }
        a.apply(Action::GbaDown(Btn::R1));
        a.apply(Action::GbaUp(Btn::R1));
        assert_eq!(video_mode_for(d.path(), "First"), VideoMode::Actual);
        assert!(!a.picture_fills_height());
        assert_eq!(a.picture_width(), 1.0);
        assert_eq!(a.source_rect(), WHOLE_TEXTURE);
        // GBA shoulders remain game buttons, so they cannot change picture scaling.
        a.set_platform(Platform::Gba);
        a.apply(Action::GbaDown(Btn::L1));
        assert_eq!(a.source_rect(), WHOLE_TEXTURE);
        assert!(!a.picture_fills_height());
        assert!(!a.taken_buttons().contains(&Btn::L1));
    }
}

#[test]
fn aspect_fit_uses_the_physical_panel_and_old_preferences_remain_compatible() {
    let d = tempfile::tempdir().unwrap();
    for mode in [VideoMode::Actual, VideoMode::Stretch, VideoMode::HeightFit] {
        write_video_mode(d.path(), "Game", mode).unwrap();
        assert_eq!(video_mode_for(d.path(), "Game"), mode);
    }
    assert_eq!(VideoMode::parse("actual"), Some(VideoMode::Actual));
    assert_eq!(VideoMode::parse("stretch"), Some(VideoMode::Stretch));
    for p in [Platform::Gb, Platform::Gbc] {
        let width = width_fraction(p, VideoMode::HeightFit) * 640.0;
        assert!((width / 480.0 - 160.0 / 144.0).abs() < 0.00001);
        assert!((width - 533.3333).abs() < 0.001);
    }
    for p in [Platform::Gba, Platform::Ps1, Platform::Snes, Platform::N64, Platform::Dreamcast, Platform::Psp] {
        assert!(!fills_height(p, VideoMode::HeightFit));
        assert_eq!(width_fraction(p, VideoMode::HeightFit), 1.0);
    }
}
