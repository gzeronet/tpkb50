//! Included custom mouse and special char keys in L1, L2.

use crate::{
    action::Action,
    keycodes::{KeyCode::*, MouseCode::*},
    keymatrix::{COLUMNS, ROWS},
};

pub type Layout = [Action; COLUMNS * ROWS];

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum LayerNumber {
    LN1 = 1,
    LN2 = 2,
}
pub const LAYERS: [Layout; 3] = [L0, L1, L2];

// activate by indexing into LAYERS
const LTKT: Action = Action::LayerTapKey(LayerNumber::LN1, Tab);
const LTKS: Action = Action::LayerTapKey(LayerNumber::LN2, Space);

const TRNS: Action = Action::Transparent;

// mouse key
const MSB1: Action = Action::Mouse(BTN1);
const MSB2: Action = Action::Mouse(BTN2);
const MSB3: Action = Action::Mouse(BTN3);
const WHUP: Action = Action::Mouse(BTN4);
const WHDN: Action = Action::Mouse(BTN5);
const WHLT: Action = Action::Mouse(BTN6);
const WHRT: Action = Action::Mouse(BTN7);

// special chars
const SKN0: Action = Action::ShiftKey(N0);
const SKN1: Action = Action::ShiftKey(N1);
const SKN2: Action = Action::ShiftKey(N2);
const SKN3: Action = Action::ShiftKey(N3);
const SKN4: Action = Action::ShiftKey(N4);
const SKN5: Action = Action::ShiftKey(N5);
const SKN6: Action = Action::ShiftKey(N6);
const SKN7: Action = Action::ShiftKey(N7);
const SKN8: Action = Action::ShiftKey(N8);
const SKN9: Action = Action::ShiftKey(N9);

pub const L0: Layout = layout![
    Escape   Q        W        E        R        T        Y        U        I        O        P        LBracket RBracket
    LCtrl    A        S        D        F        G        No       H        J        K        L        SColon   Enter
    Minus    Quote    Z        X        C        V        B        N        M        Comma    Dot      Slash    Equal
    LShift   Grave    LMeta    RMeta    LTKT     Quote    No       BSpace   LTKS     LAlt     RAlt     BSlash   RShift
];

pub const L1: Layout = layout![
    TRNS     SKN2     SKN3     SKN4     SKN5     SKN6     SKN7     SKN8     SKN9     SKN0     SKN1     TRNS     TRNS
    TRNS     N2       N3       N4       N5       N6       No       N7       N8       N9       N0       N1       TRNS
    F1       F2       F3       F4       F5       F6       No       F7       F8       F9       F10      F11      F12
    TRNS     VolDown  TRNS     TRNS     TRNS     TRNS     No       TRNS     Space    TRNS     TRNS     VolUp    TRNS
];

pub const L2: Layout = layout![
    TRNS     No       No       No       PgUp     WHRT     PScreen  WHUP     Up       MSB3     MSB2     MSB1     Delete
    TRNS     No       Home     PgDown   End      WHLT     No       WHDN     Left     Down     Right    No       TRNS
    F1       F2       F3       F4       F5       F6       No       F7       F8       F9       F10      F11      F12
    TRNS     VolDown  TRNS     TRNS     Tab      TRNS     No       TRNS     TRNS     TRNS     TRNS     VolUp    TRNS
];
