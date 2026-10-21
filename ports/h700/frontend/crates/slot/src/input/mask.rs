use slot_input::{Action, Btn};
use slot_retro::ButtonMask;

#[derive(Default)]
pub struct Pad {
    mask: u16,
    pub axes: [i16; 4],
}

impl Pad {
    pub fn apply(&mut self, action: Action) {
        match action {
            Action::GbaDown(b) => {
                if let Some(bit) = bit(b) {
                    self.mask |= bit;
                }
            }
            Action::GbaUp(b) => {
                if let Some(bit) = bit(b) {
                    self.mask &= !bit;
                }
            }
            _ => {}
        }
    }

    pub fn mask(&self) -> ButtonMask {
        ButtonMask(self.mask)
    }

    pub fn stickless(&self, analog: bool) -> (ButtonMask, [i16; 4]) {
        if !analog {
            return (self.mask(), self.axes);
        }
        let axis = |negative, positive| {
            match (self.mask & negative != 0, self.mask & positive != 0) {
                (true, false) => -32767,
                (false, true) => 32767,
                _ => 0,
            }
        };
        let axes = [axis(ButtonMask::LEFT, ButtonMask::RIGHT),
                    axis(ButtonMask::UP, ButtonMask::DOWN), 0, 0];
        let directions = ButtonMask::LEFT | ButtonMask::RIGHT | ButtonMask::UP | ButtonMask::DOWN;
        (ButtonMask(self.mask & !directions), axes)
    }

    pub fn clear(&mut self) {
        self.mask = 0;
        self.axes = [0; 4];
    }
}

#[cfg(test)]
mod stickless_tests {
    use super::*;
    #[test]
    fn analog_diagonals_release_and_opposite_directions_are_consistent() {
        let mut pad = Pad::default();
        pad.apply(Action::GbaDown(Btn::Left));
        pad.apply(Action::GbaDown(Btn::Up));
        pad.apply(Action::GbaDown(Btn::A));
        assert_eq!(pad.stickless(true), (ButtonMask(ButtonMask::A), [-32767, -32767, 0, 0]));
        pad.apply(Action::GbaDown(Btn::Right));
        assert_eq!(pad.stickless(true).1[0], 0);
        pad.apply(Action::GbaUp(Btn::Left));
        assert_eq!(pad.stickless(true).1[0], 32767);
        pad.clear();
        assert_eq!(pad.stickless(true), (ButtonMask(0), [0; 4]));
    }
    #[test]
    fn digital_mode_preserves_dpad_and_game_buttons() {
        let mut pad = Pad::default();
        pad.apply(Action::GbaDown(Btn::Down));
        pad.apply(Action::GbaDown(Btn::Y));
        assert_eq!(pad.stickless(false), (ButtonMask(ButtonMask::DOWN | ButtonMask::Y), [0; 4]));
    }
}

fn bit(btn: Btn) -> Option<u16> {
    Some(match btn {
        Btn::Up => ButtonMask::UP,
        Btn::Down => ButtonMask::DOWN,
        Btn::Left => ButtonMask::LEFT,
        Btn::Right => ButtonMask::RIGHT,
        Btn::A => ButtonMask::A,
        Btn::B => ButtonMask::B,
        Btn::X => ButtonMask::X,
        Btn::Y => ButtonMask::Y,
        Btn::L1 => ButtonMask::L,
        Btn::R1 => ButtonMask::R,
        Btn::L2 => ButtonMask::L2,
        Btn::R2 => ButtonMask::R2,
        Btn::Start => ButtonMask::START,
        Btn::Select => ButtonMask::SELECT,
        _ => return None,
    })
}
