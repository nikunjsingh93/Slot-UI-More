use std::path::PathBuf;
use std::time::Duration;

use slot_input::{Action, Gestures, Millis, RawEvent};
use slot_retro::Rumble;
use slot_store::Platform;
use slot_ui::FfState;

use crate::app::{App, Phase};
use crate::audio::{open_sink, AudioSink, Ring, Sfx, GBA_HZ};
use crate::core::open_core;
use crate::emu::{CoreState, EmuHandle, Speed};
use crate::frames::FrameRef;
use crate::input::Pad;
use crate::persist;

pub struct Session {
    root: PathBuf,
    app: App,
    emu: Option<EmuHandle>,
    sink: Box<dyn AudioSink>,
    gestures: Gestures,
    pad: Pad,
    rewinding: bool,
    fast: bool,
    motor: u16,
    reloading: bool,
    driven: bool,
    ps1_menu: bool,
    ps1_chord: bool,
    stickless: bool,
    dpad_analog: bool,
}

#[cfg(test)]
mod sp_controls_tests {
    use super::*;
    use slot_input::Btn;
    use slot_retro::ButtonMask;

    #[test]
    fn sp_analog_chord_is_consumed_and_menu_hold_still_exits_each_console() {
        for (platform, core) in [
            (Platform::N64, slot_store::Core::ParallelN64),
            (Platform::Dreamcast, slot_store::Core::Flycast),
            (Platform::Psp, slot_store::Core::Ppsspp),
        ] {
            let d = tempfile::tempdir().unwrap();
            crate::root::ensure(d.path());
            for stem in ["Probe", "Second"] {
                std::fs::write(d.path().join(format!("Games/{}/{stem}.{}", platform.dir_name(), platform.extensions()[0])), [0; 256]).unwrap();
            }
            slot_store::write_slot_state(d.path(), &slot_store::SlotState { clock_set: true, ..Default::default() }).unwrap();
            let mut s = Session::boot(d.path().to_path_buf());
            s.stickless = true;
            s.app.apply(Action::Insert);
            s.app.on_core_ready();
            for _ in 0..120 { s.app.update(1.0 / 60.0); }
            assert!(matches!(s.app.phase(), Phase::Playing { .. }));
            s.app.set_core(core);
            s.app.set_platform(platform);
            s.feed([RawEvent::Down(Btn::Menu), RawEvent::Down(Btn::Select)], 2100);
            assert!(s.dpad_analog);
            assert_eq!(s.app.toast(), Some(slot_ui::Toast::DpadAnalog));
            assert_eq!(s.pad.mask().0 & ButtonMask::SELECT, 0);
            s.feed([RawEvent::Up(Btn::Select), RawEvent::Up(Btn::Menu)], 2200);
            s.feed([RawEvent::Down(Btn::X), RawEvent::Down(Btn::Y)], 2250);
            assert_eq!(s.pad.mask().0 & (ButtonMask::X | ButtonMask::Y), ButtonMask::X | ButtonMask::Y);
            s.feed([RawEvent::Up(Btn::X), RawEvent::Up(Btn::Y)], 2260);
            s.feed([RawEvent::Down(Btn::Menu), RawEvent::Down(Btn::Select)], 2300);
            assert!(!s.dpad_analog);
            assert_eq!(s.app.toast(), Some(slot_ui::Toast::DpadDigital));
            s.feed([RawEvent::Up(Btn::Select), RawEvent::Up(Btn::Menu)], 2400);
            s.feed([RawEvent::Down(Btn::Menu)], 2500);
            s.feed([], 3501);
            assert!(!matches!(s.app.phase(), Phase::Playing { .. }), "{platform:?}: Menu hold did not eject");
        }
    }
}

impl Session {
    pub fn boot(root: PathBuf) -> Self {
        let mut sink: Box<dyn AudioSink> = open_sink();
        if let Err(e) = sink.open(GBA_HZ) {
            eprintln!("slot: audio: {e}");
        }
        Session {
            app: App::boot(&root),
            root,
            emu: None,
            sink,
            gestures: Gestures::new(),
            pad: Pad::default(),
            rewinding: false,
            fast: false,
            motor: 0,
            reloading: false,
            driven: false,
            ps1_menu: false,
            ps1_chord: false,
            stickless: matches!(std::env::var("SLOT_DEVICE").as_deref(), Ok("rg35xxsp" | "rgsp")),
            dpad_analog: false,
        }
    }

    pub fn play_sfx(&mut self, sfx: Sfx) {
        let ring = self.sink.ring();
        let rate = ring.sample_rate();
        if rate == 0 {
            return;
        }
        let mut samples = sfx.render(rate);
        crate::audio::volume::apply(&mut samples, self.app.output_volume());
        ring.mix(&samples);
    }

    pub fn audio_queued(&self) -> usize {
        self.sink.ring().queued_frames()
    }

    pub fn audio_ring(&self) -> std::sync::Arc<Ring> {
        self.sink.ring()
    }

    pub fn rumble(&mut self, strength: u16) {
        if strength == self.motor {
            return;
        }
        self.motor = strength;
        self.app.set_rumble(strength);
    }

    pub fn core_rumble(&self) -> Option<&Rumble> {
        self.emu.as_ref().map(EmuHandle::rumble)
    }

    pub fn app(&self) -> &App {
        &self.app
    }

    pub fn app_mut(&mut self) -> &mut App {
        &mut self.app
    }

    pub fn emu(&self) -> Option<&EmuHandle> {
        self.emu.as_ref()
    }

    pub fn set_driven(&mut self, driven: bool) {
        self.driven = driven;
        if let Some(emu) = &self.emu {
            emu.set_driven(driven);
        }
    }

    pub fn step_emulator(&self, present: Duration, timeout: Duration) -> bool {
        self.emu.as_ref().is_some_and(|emu| {
            emu.tick(present);
            emu.wait_frame(timeout)
        })
    }

    pub fn frame(&self) -> Option<FrameRef> {
        self.emu.as_ref().and_then(|e| e.latest_frame())
    }

    pub fn core_settling(&self) -> bool {
        self.emu
            .as_ref()
            .is_some_and(|e| e.state() == CoreState::Loading)
    }

    pub fn has_core(&self) -> bool {
        self.emu.is_some()
    }

    pub fn frame_ready(&self) -> bool {
        self.emu.as_ref().is_some_and(EmuHandle::frame_ready)
    }

    pub fn frames_published(&self) -> u64 {
        self.emu.as_ref().map_or(0, EmuHandle::published_count)
    }

    pub fn observed_speed(&self) -> Option<Speed> {
        self.emu.as_ref().map(EmuHandle::observed_speed)
    }

    pub fn frames_taken(&self) -> u64 {
        self.emu.as_ref().map_or(0, EmuHandle::frames_taken)
    }

    pub fn game_visible(&self) -> bool {
        self.app.game_visible()
    }

    pub fn feed(&mut self, events: impl IntoIterator<Item = RawEvent>, now: Millis) {
        let mut actions = Vec::new();
        for ev in events {
            // Menu can be released while an eject/insert animation is active.
            // Always clear its latch, even outside the console-playing branch.
            if ev == RawEvent::Up(slot_input::Btn::Menu) {
                self.ps1_menu = false;
                if std::mem::take(&mut self.ps1_chord) {
                    self.gestures = Gestures::new();
                    continue;
                }
            }
            if let RawEvent::Axis(axis, value) = ev {
                if let Some(a) = self.pad.axes.get_mut(axis as usize) {
                    *a = value;
                }
                continue;
            }
            if matches!(
                self.app.core(),
                slot_store::Core::PcsxRearmed
                    | slot_store::Core::Snes9x
                    | slot_store::Core::Snes9x2005Plus
                    | slot_store::Core::Mupen64plus
                    | slot_store::Core::ParallelN64
                    | slot_store::Core::Flycast
                    | slot_store::Core::Ppsspp
            ) && matches!(self.app.phase(), Phase::Playing { .. })
            {
                use slot_input::Btn;
                match ev {
                    RawEvent::Down(Btn::Menu) => {
                        self.ps1_menu = true;
                        self.ps1_chord = false;
                    }
                    RawEvent::Up(Btn::Menu) => {}
                    RawEvent::Down(Btn::Select)
                        if self.ps1_menu && self.stickless
                            && matches!(self.app.core(), slot_store::Core::ParallelN64
                                | slot_store::Core::Mupen64plus | slot_store::Core::Flycast
                                | slot_store::Core::Ppsspp) =>
                    {
                        self.ps1_chord = true;
                        self.gestures = Gestures::new();
                        self.dpad_analog = !self.dpad_analog;
                        self.app.show_input_mode(self.dpad_analog);
                        continue;
                    }
                    RawEvent::Down(Btn::R2)
                        if self.ps1_menu && self.app.core() == slot_store::Core::PcsxRearmed =>
                    {
                        self.ps1_chord = true;
                        self.gestures = Gestures::new();
                        if let Some(emu) = &self.emu {
                            emu.next_disc();
                        }
                        continue;
                    }
                    RawEvent::Down(Btn::X) if self.ps1_menu => {
                        self.ps1_chord = true;
                        self.gestures = Gestures::new();
                        actions.push(Action::SaveState);
                        continue;
                    }
                    RawEvent::Down(Btn::Y) if self.ps1_menu => {
                        self.ps1_chord = true;
                        self.gestures = Gestures::new();
                        actions.push(Action::LoadState);
                        continue;
                    }
                    RawEvent::Down(b) if is_ps1_button(b) => {
                        actions.push(Action::GbaDown(b));
                        continue;
                    }
                    RawEvent::Up(b) if is_ps1_button(b) => {
                        actions.push(Action::GbaUp(b));
                        continue;
                    }
                    _ => {}
                }
            }
            actions.extend(self.gestures.feed(ev, now));
        }
        actions.extend(self.gestures.tick(now));
        for action in actions {
            self.act(action);
        }
        self.sync_pad();
    }

    fn sync_pad(&mut self) {
        for btn in self.app.taken_buttons() {
            self.pad.apply(Action::GbaUp(*btn));
        }
        if let Some(emu) = &self.emu {
            let analog = self.stickless && self.dpad_analog
                && matches!(self.app.core(), slot_store::Core::ParallelN64
                    | slot_store::Core::Mupen64plus | slot_store::Core::Flycast
                    | slot_store::Core::Ppsspp);
            let (buttons, axes) = self.pad.stickless(analog);
            emu.set_input(buttons);
            emu.set_axes(axes);
        }
    }

    fn bridge_link(&mut self, f: impl FnOnce(&mut App)) {
        let had_link = self.app.link_active();
        f(&mut self.app);
        if had_link && !self.app.link_active() {
            if let Some(emu) = &self.emu {
                emu.end_link();
            }
        }
    }

    fn act(&mut self, action: Action) {
        if trace() {
            eprintln!("slot: {action:?} in {:?}", self.app.phase());
        }
        match action {
            Action::RewindStart => self.rewinding = true,
            Action::RewindStop => self.rewinding = false,
            Action::FfStart => self.fast = true,
            Action::FfStop => self.fast = false,
            _ => {}
        }
        let menu = self.overlaid();
        self.bridge_link(|app| app.apply(action));
        if matches!(
            action,
            Action::VolumeUp | Action::VolumeDown | Action::MuteToggle
        ) {
            if let Some(emu) = &self.emu {
                emu.set_volume(self.app.output_volume());
            }
        }
        if menu || self.overlaid() {
            self.pad.clear();
        } else if self.app.takes_from_the_game(action) {
            if let Action::GbaDown(btn) | Action::GbaUp(btn) = action {
                self.pad.apply(Action::GbaUp(btn));
            }
        } else {
            self.pad.apply(action);
        }
        self.sync_rumble();
    }

    pub fn update(&mut self, dt: f32) {
        if let Some(success) = self.emu.as_ref().and_then(EmuHandle::take_disc_notice) {
            self.app.disc_notice(success);
        }
        self.bridge_link(|app| app.update(dt));
        if let Some(emu) = &self.emu {
            emu.set_volume(self.app.output_volume());
        }
        if let Some((client_id, transport)) = self.app.take_link_transport() {
            match &self.emu {
                Some(emu) => match self.app.link_player() {
                    Some(player) => emu.begin_cable(player, transport),
                    None => emu.begin_link(client_id, transport),
                },
                None => eprintln!("slot: link: a transport arrived with no core to run it"),
            }
        }
        if let Some(on) = self.app.take_colour_correction() {
            if let Some((key, value)) = crate::core::colour_option(self.app.core(), on) {
                if let Some(emu) = &self.emu {
                    emu.set_option(key, value);
                }
            }
        }
        if let Some((stem, serial)) = self.app.take_link_reload() {
            self.reload_for_link(&stem, serial);
        }
        if self.app.link_active() {
            if self.emu.as_ref().is_some_and(EmuHandle::bios_mismatch) {
                self.bridge_link(|app| app.bios_mismatch());
            } else if self.emu.as_ref().is_some_and(EmuHandle::peer_ended) {
                self.bridge_link(|app| app.peer_ended());
            } else if self.emu.as_ref().is_some_and(EmuHandle::link_lost) {
                self.app.peer_lost();
            }
        }
        if let Some(sfx) = self.app.take_sfx() {
            self.play_sfx(sfx);
        }
        self.sync_core();
        self.sync_reload();
        if !self.has_core() {
            for action in self.gestures.drop_ff_latch() {
                self.act(action);
            }
        }
        self.app
            .set_game_ready(self.emu.as_ref().is_some_and(EmuHandle::has_published));
        self.sync_speed();
        self.sync_rewind_hud();
        self.sync_ff_hud();
        self.sync_rumble();
        self.sync_pad();
    }

    fn sync_rumble(&mut self) {
        let want = match &self.emu {
            Some(emu) if self.playing() && self.app.rumble_enabled() => emu.rumble().strength(),
            _ => 0,
        };
        self.rumble(want);
    }

    fn sync_ff_hud(&mut self) {
        let ff = match (self.actually_fast_forwarding(), self.gestures.ff_latched()) {
            (false, _) => FfState::Off,
            (true, false) => FfState::Held,
            (true, true) => FfState::Latched,
        };
        self.app.set_ff(ff);
    }

    fn sync_rewind_hud(&mut self) {
        let fill = self
            .actually_rewinding()
            .then(|| self.emu.as_ref().map(EmuHandle::rewind_fill))
            .flatten();
        match fill {
            Some(fill) => self.app.show_rewind(fill),
            None => self.app.hide_rewind(),
        }
    }

    fn actually_rewinding(&self) -> bool {
        self.rewinding && self.playing() && self.app.may_rewind()
    }

    fn actually_fast_forwarding(&self) -> bool {
        self.fast && self.playing() && self.app.may_fast_forward()
    }

    fn inserting(&self) -> bool {
        matches!(self.app.phase(), Phase::Inserting { .. })
    }

    fn showing_polaroids(&self) -> bool {
        matches!(self.app.phase(), Phase::Polaroids { .. })
    }

    fn overlaid(&self) -> bool {
        self.showing_polaroids() || self.held()
    }

    fn playing(&self) -> bool {
        matches!(self.app.phase(), Phase::Playing { .. }) && !self.held()
    }

    fn held(&self) -> bool {
        self.app.power_menu().is_some() || self.app.game_menu_open() || self.app.shutting_down()
    }

    fn dozing(&self) -> bool {
        matches!(self.app.phase(), Phase::Doze { .. })
    }

    fn ejecting(&self) -> bool {
        matches!(self.app.phase(), Phase::Ejecting { .. })
    }

    fn sync_speed(&self) {
        if let Some(emu) = &self.emu {
            emu.set_fast_steps(u32::from(self.app.ff_speed()));
            emu.set_ff_sound(self.app.ff_sound());
            emu.set_speed(
                if self.inserting()
                    || self.ejecting()
                    || self.showing_polaroids()
                    || self.dozing()
                    || (self.held() && !self.app.link_active())
                {
                    Speed::Paused
                } else if self.actually_fast_forwarding() {
                    Speed::Fast
                } else {
                    Speed::Normal
                },
            );
            emu.set_rewinding(self.actually_rewinding());
        }
    }

    fn no_core() -> bool {
        std::env::var_os("SLOT_NO_CORE").is_some_and(|v| v != "0")
    }

    fn sync_core(&mut self) {
        if Self::no_core() {
            return;
        }
        let stem = match self.app.phase() {
            Phase::Shelf => {
                self.emu = None;
                self.ps1_menu = false;
                self.ps1_chord = false;
                return;
            }
            Phase::Inserting { cart, .. } => cart.clone(),
            _ => return,
        };
        if self.emu.is_none() {
            let (_, serial) = self.app.link_mode(&stem);
            self.spawn_core(&stem, serial);
        }
        match self.emu.as_ref().map(EmuHandle::state) {
            Some(CoreState::Loading) => {}
            Some(CoreState::Ready) => self.app.on_core_ready(),
            Some(CoreState::Failed) | None => {
                self.emu = None;
                self.app.on_core_failed();
            }
        }
    }

    fn spawn_core(&mut self, stem: &str, serial: &'static str) {
        let Some((rom, platform)) = self
            .app
            .seated_cart()
            .filter(|c| c.stem == stem)
            .map(|c| (c.rom.clone(), c.platform))
        else {
            return;
        };
        if platform == Platform::N64 && crate::ports::n64_native(&self.root, stem) {
            if let Err(error)=crate::ports::request_n64(&self.root,&rom) {
                eprintln!("slot: N64 launch: {error}");self.app.on_core_failed();return;
            }
            std::process::exit(85);
        }
        if platform == Platform::Ports {
            if let Err(error) = crate::ports::request(&self.root, &rom) {
                eprintln!("slot: port launch: {error}");
                self.app.on_core_failed();
                return;
            }
            // The parent launcher runs the port after this process exits,
            // releasing DRM master, EGL and ALSA before the game starts.
            std::process::exit(85);
        }
        let core = slot_store::core_for_platform(&self.root, stem, platform);
        self.ps1_menu = false;
        self.ps1_chord = false;
        self.app.set_core(core);
        self.app.set_platform(platform);
        self.dpad_analog = self.stickless && platform == Platform::N64;
        self.app
            .set_video_mode(crate::video_mode::video_mode_for(&self.root, stem));
        self.app.set_link_loaded(serial);
        let resume = (!self.app.starting_clean())
            .then(|| persist::read_resume(&self.root, platform, core, stem))
            .flatten();
        let player = self.app.link_player();
        let opened = open_core(
            &self.root,
            core,
            serial,
            self.app.colour_correction(),
            player,
        );
        self.app.set_named_core(opened.named);
        let sav = persist::read_sav(&self.root, platform, stem);
        let ring = self.sink.ring();
        let emu = match player.filter(|_| platform == Platform::Gba) {
            Some(p) => EmuHandle::spawn_linked(opened.core, rom, ring, sav, resume, p),
            None => EmuHandle::spawn(opened.core, rom, ring, sav, resume),
        };
        emu.set_volume(self.app.output_volume());
        emu.set_driven(self.driven);
        self.app.set_snapshot(Box::new(emu.snapshot()));
        self.emu = Some(emu);
    }

    fn reload_for_link(&mut self, stem: &str, serial: &'static str) {
        eprintln!("slot: link: loading {stem} again with gpsp_serial={serial}");
        if self.emu.is_some() {
            self.app.flush_resume();
        }
        self.emu = None;
        self.spawn_core(stem, serial);
        self.reloading = true;
    }

    fn sync_reload(&mut self) {
        if !self.reloading {
            return;
        }
        match self.emu.as_ref().map(EmuHandle::state) {
            Some(CoreState::Loading) => {}
            Some(CoreState::Ready) => {
                self.reloading = false;
                self.app.link_reload_done();
            }
            Some(CoreState::Failed) | None => {
                self.reloading = false;
                self.emu = None;
                self.app.link_reload_failed();
                if let Some((stem, serial)) = self.app.take_link_reload() {
                    self.reload_for_link(&stem, serial);
                }
            }
        }
    }
}

pub(crate) fn trace() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("SLOT_TRACE").is_some())
}

fn is_ps1_button(b: slot_input::Btn) -> bool {
    use slot_input::Btn::*;
    matches!(
        b,
        Up | Down | Left | Right | A | B | X | Y | L1 | R1 | L2 | R2 | Start | Select
    )
}

#[cfg(test)]
mod ps1_controls {
    use super::*;
    use slot_input::Btn;
    #[test]
    fn menu_released_during_ejection_does_not_steal_face_buttons_in_next_game() {
        for (platform, core, rom) in [
            (Platform::Ps1, slot_store::Core::PcsxRearmed, "Disc.chd"),
            (Platform::Snes, slot_store::Core::Snes9x2005Plus, "Cart.sfc"),
            (Platform::N64, slot_store::Core::ParallelN64, "Cart.n64"),
            (Platform::N64, slot_store::Core::Mupen64plus, "Cart.n64"),
            (Platform::Dreamcast, slot_store::Core::Flycast, "Disc.chd"),
            (Platform::Psp, slot_store::Core::Ppsspp, "Disc.iso"),
        ] {
            let root = tempfile::tempdir().unwrap();
            let mut session = Session::boot(root.path().to_path_buf());
            let playing = || {
                let mut app = App::new(
                    (0..2)
                        .map(|i| slot_store::Cart {
                            platform,
                            stem: format!("Game{i}"),
                            rom: rom.into(),
                            label: None,
                            title: String::new(),
                            code: String::new(),
                            shell: None,
                        })
                        .collect(),
                );
                app.set_core(core);
                app.set_platform(platform);
                app.apply(Action::Insert);
                app.on_core_ready();
                for _ in 0..120 {
                    app.update(1.0 / 60.0);
                }
                app
            };
            session.app = playing();
            session.feed([RawEvent::Down(Btn::Menu)], 2000);
            session.app.apply(Action::Eject);
            assert!(matches!(session.app.phase(), Phase::Ejecting { .. }));
            session.feed([RawEvent::Up(Btn::Menu)], 2100);
            assert!(!session.ps1_menu);
            session.app = playing();
            for (button, mask) in [
                (Btn::X, slot_retro::ButtonMask::X),
                (Btn::Y, slot_retro::ButtonMask::Y),
            ] {
                session.feed([RawEvent::Down(button)], 2300);
                assert_eq!(session.pad.mask().0, mask);
                session.feed([RawEvent::Up(button)], 2310);
                assert_eq!(session.pad.mask().0, 0);
            }
            // Save-state chords consume the face button only while Menu is held.
            session.feed([RawEvent::Down(Btn::Menu), RawEvent::Down(Btn::X)], 2400);
            assert_eq!(session.pad.mask().0, 0);
            session.feed([RawEvent::Up(Btn::X), RawEvent::Up(Btn::Menu)], 2410);
            session.feed([RawEvent::Down(Btn::Y)], 2420);
            assert_eq!(session.pad.mask().0, slot_retro::ButtonMask::Y);
        }
    }
    #[test]
    fn playstation_buttons_reach_the_pad_without_turbo_or_gesture_shortcuts() {
        let root = tempfile::tempdir().unwrap();
        let mut session = Session::boot(root.path().to_path_buf());
        session.app = App::new(vec![slot_store::Cart {
            platform: Platform::Ps1,
            stem: "Disc".into(),
            rom: "Disc.chd".into(),
            label: None,
            title: String::new(),
            code: String::new(),
            shell: None,
        }]);
        session.app.set_core(slot_store::Core::PcsxRearmed);
        session.app.apply(Action::Insert);
        session.app.on_core_ready();
        for _ in 0..120 {
            session.app.update(1.0 / 60.0);
        }
        assert!(matches!(session.app.phase(), Phase::Playing { .. }));
        let buttons = [
            Btn::A,
            Btn::B,
            Btn::X,
            Btn::Y,
            Btn::L1,
            Btn::R1,
            Btn::L2,
            Btn::R2,
            Btn::Select,
            Btn::Start,
        ];
        session.feed(buttons.map(RawEvent::Down), 2000);
        let want = slot_retro::ButtonMask::A
            | slot_retro::ButtonMask::B
            | slot_retro::ButtonMask::X
            | slot_retro::ButtonMask::Y
            | slot_retro::ButtonMask::L
            | slot_retro::ButtonMask::R
            | slot_retro::ButtonMask::L2
            | slot_retro::ButtonMask::R2
            | slot_retro::ButtonMask::SELECT
            | slot_retro::ButtonMask::START;
        assert_eq!(session.pad.mask().0, want);
        assert!(!session.fast && !session.rewinding);
        session.feed(buttons.map(RawEvent::Up), 2010);
        assert_eq!(session.pad.mask().0, 0);
        session.feed([RawEvent::Down(Btn::Menu), RawEvent::Down(Btn::R2)], 2100);
        session.feed([RawEvent::Up(Btn::R2), RawEvent::Up(Btn::Menu)], 2120);
        session.feed([], 4000);
        assert!(matches!(session.app.phase(), Phase::Playing { .. }));
        assert_eq!(session.pad.mask().0, 0);
    }
}
