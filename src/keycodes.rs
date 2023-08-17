//! Include extra function keys.
//! Add Mouse btn codes.

#![allow(dead_code)]

// USB HID KeyCodes
#[derive(PartialOrd, PartialEq, Copy, Clone, Debug, Default)]
pub enum KeyCode {
    #[default]
    No = 0x00,
    RollOver,
    PostFail,
    Undefined,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M, // 0x10
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    N1,
    N2,
    N3, // 0x20
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N0,
    Enter,
    Escape,
    BSpace,
    Tab,
    Space,
    Minus,
    Equal,
    LBracket,
    RBracket,  // 0x30
    BSlash,    // \ (and |)
    NonUSHash, // Non-US # and ~ (Typically near the Enter key)
    SColon,    // ; (and :)
    Quote,     // ' and "
    Grave,     // Grave accent and tilde
    Comma,     // , and <
    Dot,       // . and >
    Slash,     // / and ?
    Capslock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7, // 0x40
    F8,
    F9,
    F10,
    F11,
    F12,
    PScreen,
    Scrolllock,
    Pause,
    Insert,
    Home,
    PgUp,
    Delete,
    End,
    PgDown,
    Right,
    Left, // 0x50
    Down,
    Up,
    Numlock,
    KpSlash,
    KpAsterisk,
    KpMinus,
    KpPlus,
    KpEnter,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8, // 0x60
    Kp9,
    Kp0,
    KpDot,
    NonUSBackslash, // Non-US \ and | (Typically near the Left-Shift key)
    Application,    // 0x65 - Max keycode the Bluetooth HID descriptor supports
    // extra function keys
    Power,
    KpEqual,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21, // 0x70
    F22,
    F23,
    F24,
    Execute,
    Help,
    Menu,
    Select,
    Stop,
    Again,
    Undo,
    Cut,
    Copy,
    Paste,
    Find,
    Mute,
    VolUp, // 0x80
    VolDown,
    LockingCapsLock,
    LockingNumLock,
    LockingScrollLock,
    KpComma,
    KpEqualSign,
    Intl1,
    Intl2,
    Intl3,
    Intl4,
    Intl5,
    Intl6,
    Intl7,
    Intl8,
    Intl9,
    Lang1, // 0x90
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    Lang6,
    Lang7,
    Lang8,
    Lang9,
    AltErase,
    SysReq,
    Cancel,
    Clear,
    Prior,
    Return,
    Separator,
    Out, // 0xA0
    Oper,
    ClearAgain,
    CrSel,
    ExSel,

    // Modifiers
    LCtrl = 0xE0,
    LShift,
    LAlt,
    LMeta,
    RCtrl,
    RShift,
    RAlt,
    RMeta, // 0xE7
}

impl KeyCode {
    pub fn is_modifier(self) -> bool {
        self >= KeyCode::LCtrl && self <= KeyCode::RMeta
    }

    pub fn is_normal_key(self) -> bool {
        self >= KeyCode::A && self <= KeyCode::ExSel
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum MouseCode {
    BTN1 = 0b001,
    BTN2 = 0b010,
    BTN3 = 0b100,
    BTN4,
    BTN5,
    BTN6,
    BTN7,
}
