use crate::keycodes::{KeyCode, MouseCode};
use crate::layout::LayerNumber;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Action {
    Nop,
    Transparent, // Fall-through to the next layer underneath
    Key(KeyCode),
    ShiftKey(KeyCode),
    LayerTapKey(LayerNumber, KeyCode),
    LayerMomentary(LayerNumber),
    LayerToggle(LayerNumber),
    Mouse(MouseCode),
}

// Allow auto-conversion of KeyCodes to Action for nicer layout formatting
// and drop commas
macro_rules! layout {
    ( $( $e:expr )* ) => {
        [
            $(
                $e.to_action(),
            )*
        ]
    };
}

impl KeyCode {
    pub const fn to_action(self) -> Action {
        Action::Key(self)
    }
}

impl Action {
    pub const fn to_action(self) -> Action {
        self
    }
}
