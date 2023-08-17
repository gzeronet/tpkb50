//! Get a lot help from https://github.com/ah-/anne-key.
//! That keyboard framework looks great, just like tmk.
//! Removed bt, led, add LayerTapKey, ShiftKey, Mouse 7 btns.

#![deny(warnings)]
#![deny(unsafe_code)]

use crate::{
    action::Action,
    keycodes::KeyCode,
    keymatrix::{KeyState, COLUMNS, KEYBYTES, ROWS},
    layout::LAYERS,
};
use bit_field::{BitArray, BitField};
use usbd_hid::descriptor::KeyboardReport;

pub struct Keyboard {
    layers: Layers,
    previous_state: KeyState,
    // layer_tap_key holding counter
    holding_counter: u8,
}

impl Keyboard {
    pub const fn new() -> Keyboard {
        Keyboard {
            layers: Layers::new(),
            previous_state: [0; KEYBYTES],
            holding_counter: 0,
        }
    }

    /// Get the action for `key`.

    /// The top non-Transparent action at index `key` amongst the
    /// currently active layers is returned.
    fn get_action(&self, key: usize) -> Action {
        let mut action = Action::Transparent;
        for i in (0..LAYERS.len()).rev() {
            if self.layers.current.get_bit(i) {
                action = LAYERS[i][key];
            }
            if action != Action::Transparent {
                break;
            }
        }
        action
    }

    pub fn gen_report(&mut self, state: &KeyState) -> Option<KeyboardReport> {
        if &self.previous_state != state {
            let mut hid = HidProcessor::default();

            for key in 0..COLUMNS * ROWS {
                let pressed = state.get_bit(key);
                let changed = self.previous_state.get_bit(key) != pressed;

                // Only handle currently pressed and changed keys to
                // cut down on processing time.
                if pressed || changed {
                    let action = self.get_action(key);
                    match action {
                        Action::LayerTapKey(layer, kc) => {
                            self.layers
                                .process(&Action::LayerMomentary(layer), pressed, changed);
                            match (pressed, changed) {
                                (true, false) => {
                                    self.holding_counter = self.holding_counter.saturating_add(1);
                                }
                                (false, true) => {
                                    self.holding_counter = self.holding_counter.saturating_add(1);
                                    if self.holding_counter == 1 {
                                        hid.process(&kc.to_action(), true, changed);
                                        hid.report.reserved = kc as u8;
                                    }
                                    self.holding_counter = 0;
                                }
                                _ => {}
                            }
                        }
                        _ => {
                            hid.process(&action, pressed, changed);
                            self.layers.process(&action, pressed, changed);
                        }
                    }
                }
            }

            self.layers.finish();
            self.previous_state = *state;
            return Some(hid.report);
        }
        None
    }
}

trait EventProcessor {
    fn process(&mut self, action: &Action, pressed: bool, changed: bool);
    fn finish(&mut self) {}
}

/// Bit-field of the currently active layers, indexed by position in
/// [`layout::LAYERS`].
struct Layers {
    current: u8,
    /// Active layers after action processing is finished
    next: u8,
}

impl Layers {
    const fn new() -> Layers {
        Layers {
            current: 0b1,
            next: 0b1,
        }
    }
}

impl EventProcessor for Layers {
    fn process(&mut self, action: &Action, pressed: bool, changed: bool) {
        if changed {
            match (*action, pressed) {
                (Action::LayerMomentary(layer), _) => self.next.set_bit(layer as usize, pressed),
                (Action::LayerToggle(layer), true) => {
                    let current = self.next.get_bit(layer as usize);
                    self.next.set_bit(layer as usize, !current)
                }
                _ => &mut self.next,
            };
        }
    }

    fn finish(&mut self) {
        self.current = self.next;
    }
}

struct HidProcessor {
    pub report: KeyboardReport,
    /// Number of normal keys to be sent in `report`
    i: usize,
}

impl HidProcessor {
    pub const fn default() -> Self {
        Self {
            report: KeyboardReport {
                modifier: 0,
                reserved: 0,
                leds: 0,
                keycodes: [0u8; 6],
            },
            i: 0,
        }
    }
}

impl EventProcessor for HidProcessor {
    fn process(&mut self, action: &Action, pressed: bool, _changed: bool) {
        if pressed {
            match *action {
                Action::Key(code) => {
                    if code.is_modifier() {
                        self.report
                            .modifier
                            .set_bit(code as usize - KeyCode::LCtrl as usize, true);
                    } else if code.is_normal_key() && self.i < self.report.keycodes.len() {
                        self.report.keycodes[self.i] = code as u8;
                        self.i += 1;
                    }
                }
                // implement shift & key
                Action::ShiftKey(code) => {
                    if code.is_normal_key() && self.i < self.report.keycodes.len() {
                        self.report
                            .modifier
                            .set_bit(KeyCode::LShift as usize - KeyCode::LCtrl as usize, true);
                        self.report.keycodes[self.i] = code as u8;
                        self.i += 1;
                    }
                }
                // implement wheel
                Action::Mouse(code) => self.report.reserved = code as u8,
                _ => {}
            }
        }
    }
}
