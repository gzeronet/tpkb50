//! Custom 4 rows, just let key matrix as simple as possible.

use bit_field::BitArray;
use hal::gpio::{EPin, Input, Output};
use stm32f4xx_hal as hal;

pub const ROWS: usize = 4;
pub const COLUMNS: usize = 13;
type RowPins = [EPin<Input>; ROWS];
type ColumnPins = [EPin<Output>; COLUMNS];

// State of the scan matrix
pub const KEYBYTES: usize = 7; // ROWS * COLUMNS / 8 round up
pub type KeyState = [u8; KEYBYTES];

pub struct KeyMatrix {
    // Stores the currently pressed down keys from last sample.
    pub state: KeyState,
    row_pins: RowPins,
    column_pins: ColumnPins,
}

impl KeyMatrix {
    pub fn new(row_pins: RowPins, column_pins: ColumnPins) -> Self {
        Self {
            state: [0; KEYBYTES],
            row_pins,
            column_pins,
        }
    }

    pub fn current_state(&mut self) -> KeyState {
        for column in 0..COLUMNS {
            self.column_pins[column].set_high();
            cortex_m::asm::delay(1000); // empirical time
            for i in 0..ROWS {
                self.state
                    .set_bit(i * COLUMNS + column, self.row_pins[i].is_high());
            }
            self.column_pins[column].set_low();
        }
        self.state
    }
}
