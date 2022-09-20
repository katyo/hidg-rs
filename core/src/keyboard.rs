use bitmask_enum::bitmask;
use core::mem::{size_of, transmute};
use static_assertions::const_assert_eq;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Class, StateChange};

/// Keyboard HID class
#[derive(Clone, Copy, Debug)]
pub struct Keyboard;

impl Class for Keyboard {
    type Input = KeyboardInput;
    type Output = KeyboardOutput;

    fn input(&self) -> Self::Input {
        Self::Input::default()
    }

    fn output(&self) -> Self::Output {
        Self::Output::default()
    }
}

impl AsRef<str> for Keyboard {
    fn as_ref(&self) -> &str {
        "keyboard"
    }
}

impl core::fmt::Display for Keyboard {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(self.as_ref())
    }
}

/// Modifier mask
#[bitmask(u8)]
pub enum Modifiers {
    /// Left Control
    LeftCtrl = 0x01,
    /// Left Shift
    LeftShift = 0x02,
    /// Left Alt
    LeftAlt = 0x04,
    /// Left Meta
    LeftMeta = 0x08,
    /// Right Control
    RightCtrl = 0x10,
    /// Right Shift
    RightShift = 0x20,
    /// Right Alt
    RightAlt = 0x40,
    /// Right Meta
    RightMeta = 0x80,
}

const_assert_eq!(size_of::<Modifiers>(), 1);

impl Default for Modifiers {
    fn default() -> Self {
        Self::none()
    }
}

impl Modifiers {
    /// Converts from raw value safely
    pub fn safe_from(raw: u8) -> Option<Self> {
        Some(Self::from(raw))
    }
}

code_enum! {
    /// Key code
    Key: u8 {
        /// No key
        None = 0x00 => "none",

        /// Keyboard Error Roll Over - used for all slots if too many keys are pressed ("Phantom key")
        Overflow = 0x01 => "overflow",
        /// Keyboard POST Fail
        PostFail = 0x02 => "post-fail" | "postfail",
        /// Keyboard Error Undefined
        Undefined = 0x03 => "undefined",

        /// Keyboard a and A
        A = 0x04 => "a",
        /// Keyboard b and B
        B = 0x05 => "b",
        /// Keyboard c and C
        C = 0x06 => "c",
        /// Keyboard d and D
        D = 0x07 => "d",
        /// Keyboard e and E
        E = 0x08 => "e",
        /// Keyboard f and F
        F = 0x09 => "f",
        /// Keyboard g and G
        G = 0x0a => "g",
        /// Keyboard h and H
        H = 0x0b => "h",
        /// Keyboard i and I
        I = 0x0c => "i",
        /// Keyboard j and J
        J = 0x0d => "j",
        /// Keyboard k and K
        K = 0x0e => "k",
        /// Keyboard l and L
        L = 0x0f => "l",
        /// Keyboard m and M
        M = 0x10 => "m",
        /// Keyboard n and N
        N = 0x11 => "n",
        /// Keyboard o and O
        O = 0x12 => "o",
        /// Keyboard p and P
        P = 0x13 => "p",
        /// Keyboard q and Q
        Q = 0x14 => "q",
        /// Keyboard r and R
        R = 0x15 => "r",
        /// Keyboard s and S
        S = 0x16 => "s",
        /// Keyboard t and T
        T = 0x17 => "t",
        /// Keyboard u and U
        U = 0x18 => "u",
        /// Keyboard v and V
        V = 0x19 => "v",
        /// Keyboard w and W
        W = 0x1a => "w",
        /// Keyboard x and X
        X = 0x1b => "x",
        /// Keyboard y and Y
        Y = 0x1c => "y",
        /// Keyboard z and Z
        Z = 0x1d => "z",

        /// Keyboard 1 and !
        Num1 = 0x1e => "1",
        /// Keyboard 2 and @
        Num2 = 0x1f => "2",
        /// Keyboard 3 and #
        Num3 = 0x20 => "3",
        /// Keyboard 4 and $
        Num4 = 0x21 => "4",
        /// Keyboard 5 and %
        Num5 = 0x22 => "5",
        /// Keyboard 6 and ^
        Num6 = 0x23 => "6",
        /// Keyboard 7 and &
        Num7 = 0x24 => "7",
        /// Keyboard 8 and *
        Num8 = 0x25 => "8",
        /// Keyboard 9 and (
        Num9 = 0x26 => "9",
        /// Keyboard 0 and )
        Num0 = 0x27 => "0",

        /// Keyboard Return (ENTER)
        Enter = 0x28 => "enter",
        /// Keyboard ESCAPE
        Esc = 0x29 => "esc" | "escape",
        /// Keyboard DELETE (Backspace)
        BackSpace = 0x2a => "backspace" | "back-space",
        /// Keyboard Tab
        Tab = 0x2b => "tab",
        /// Keyboard Spacebar
        Space = 0x2c => "space",
        /// Keyboard - and _
        Minus = 0x2d => "minus" | "-",
        /// Keyboard = and +
        Equal = 0x2e => "equal" | "=",
        /// Keyboard [ and {
        LeftBrace = 0x2f => "left-brace" | "{" | "[",
        /// Keyboard ] and }
        RightBrace = 0x30 => "right-brace" | "}" | "]",
        /// Keyboard \ and |
        BackSlash = 0x31 => "back-slash" | "\\" | "|",
        /// Keyboard Non-US # and ~
        HashTilde = 0x32 => "hash-tilde" | "hash" | "tilde" | "#" | "~",
        /// Keyboard ; and :
        Semicolon = 0x33 => "semicolon" | ";",
        /// Keyboard ' and "
        Apostrophe = 0x34 => "apostrophe" | "'" | "\"",
        /// Keyboard ` and ~
        Grave = 0x35 => "grave" | "`",
        /// Keyboard , and <
        Comma = 0x36 => "comma" | ",",
        /// Keyboard . and >
        Dot = 0x37 => "dot" | ".",
        /// Keyboard / and ?
        Slash = 0x38 => "slash" | "/",
        /// Keyboard Caps Lock
        CapsLock = 0x39 => "caps-lock" | "capslock",

        /// Keyboard F1
        F1 = 0x3a => "f1",
        /// Keyboard F2
        F2 = 0x3b => "f2",
        /// Keyboard F3
        F3 = 0x3c => "f3",
        /// Keyboard F4
        F4 = 0x3d => "f4",
        /// Keyboard F5
        F5 = 0x3e => "f5",
        /// Keyboard F6
        F6 = 0x3f => "f6",
        /// Keyboard F7
        F7 = 0x40 => "f7",
        /// Keyboard F8
        F8 = 0x41 => "f8",
        /// Keyboard F9
        F9 = 0x42 => "f9",
        /// Keyboard F10
        F10 = 0x43 => "f10",
        /// Keyboard F11
        F11 = 0x44 => "f11",
        /// Keyboard F12
        F12 = 0x45 => "f12",

        /// Keyboard Print Screen
        SysRq = 0x46 => "sysrq" | "print-screen",
        /// Keyboard Scroll Lock
        ScrollLock = 0x47 => "scroll-lock" | "scrolllock",
        /// Keyboard Pause
        Pause = 0x48 => "pause",
        /// Keyboard Insert
        Insert = 0x49 => "insert",
        /// Keyboard Home
        Home = 0x4a => "home",
        /// Keyboard Page Up
        PageUp = 0x4b => "page-up" | "pageup",
        /// Keyboard Delete Forward
        Delete = 0x4c => "delete",
        /// Keyboard End
        End = 0x4d => "end",
        /// Keyboard Page Down
        PageDown = 0x4e => "page-down" | "pagedown",
        /// Keyboard Right Arrow
        Right = 0x4f => "right",
        /// Keyboard Left Arrow
        Left = 0x50 => "left",
        /// Keyboard Down Arrow
        Down = 0x51 => "down",
        /// Keyboard Up Arrow
        Up = 0x52 => "up",

        /// Keyboard Num Lock and Clear
        NumLock = 0x53 => "num-lock" | "numlock",
        /// Keypad /
        KeyPadSlash = 0x54 => "keypad-slash",
        /// Keypad *
        KeyPadAsterisk = 0x55 => "keypad-asterisk",
        /// Keypad -
        KeyPadMinus = 0x56 => "keypad-minus",
        /// Keypad +
        KeyPadPlus = 0x57 => "keypad-plus",
        /// Keypad ENTER
        KyePadEnter = 0x58 => "keypad-enter",
        /// Keypad 1 and End
        KeyPad1 = 0x59 => "keypad-1",
        /// Keypad 2 and Down Arrow
        KeyPad2 = 0x5a => "keypad-2",
        /// Keypad 3 and PageDn
        KeyPad3 = 0x5b => "keypad-3",
        /// Keypad 4 and Left Arrow
        KeyPad4 = 0x5c => "keypad-4",
        /// Keypad 5
        KeyPad5 = 0x5d => "keypad-5",
        /// Keypad 6 and Right Arrow
        KeyPad6 = 0x5e => "keypad-6",
        /// Keypad 7 and Home
        KeyPad7 = 0x5f => "keypad-7",
        /// Keypad 8 and Up Arrow
        KeyPad8 = 0x60 => "keypad-8",
        /// Keypad 9 and Page Up
        KeyPad9 = 0x61 => "keypad-9",
        /// Keypad 0 and Insert
        KeyPad0 = 0x62 => "keypad-0",
        /// Keypad . and Delete
        KeyPadDot = 0x63 => "keypad-dot",

        /// Keyboard Non-US \ and |
        NonUsBackSlash = 0x64 => "nonus-backslash",
        /// Keyboard Application
        Compose = 0x65 => "compose",
        /// Keyboard Power
        Power = 0x66 => "power",
        /// Keypad =
        KeyPadEqual = 0x67 => "keypad-equal",

        /// Keyboard F13
        F13 = 0x68 => "f13",
        /// Keyboard F14
        F14 = 0x69 => "f14",
        /// Keyboard F15
        F15 = 0x6a => "f15",
        /// Keyboard F16
        F16 = 0x6b => "f16",
        /// Keyboard F17
        F17 = 0x6c => "f17",
        /// Keyboard F18
        F18 = 0x6d => "f18",
        /// Keyboard F19
        F19 = 0x6e => "f19",
        /// Keyboard F20
        F20 = 0x6f => "f20",
        /// Keyboard F21
        F21 = 0x70 => "f21",
        /// Keyboard F22
        F22 = 0x71 => "f22",
        /// Keyboard F23
        F23 = 0x72 => "f23",
        /// Keyboard F24
        F24 = 0x73 => "f24",

        /// Keyboard Execute
        Open = 0x74 => "open",
        /// Keyboard Help
        Help = 0x75 => "help",
        /// Keyboard Menu
        Props = 0x76 => "props",
        /// Keyboard Select
        Front = 0x77 => "front",
        /// Keyboard Stop
        Stop = 0x78 => "stop",
        /// Keyboard Again
        Again = 0x79 => "again",
        /// Keyboard Undo
        Undo = 0x7a => "undo",
        /// Keyboard Cut
        Cut = 0x7b => "cut",
        /// Keyboard Copy
        Copy = 0x7c => "copy",
        /// Keyboard Paste
        Paste = 0x7d => "paste",
        /// Keyboard Find
        Find = 0x7e => "find",
        /// Keyboard Mute
        Mute = 0x7f => "mute",
        /// Keyboard Volume Up
        VolumeUp = 0x80 => "volume-up" | "volumeup",
        /// Keyboard Volume Down
        VolumeDown = 0x81 => "volume-down" | "volumedown",
        /// Keyboard Locking Caps Lock
        LockingCapsLock = 0x82 => "locking-caps-lock" | "locking-capslock",
        /// Keyboard Locking Num Lock
        LockingNumLock = 0x83 => "locking-num-lock" | "locking-numlock",
        /// Keyboard Locking Scroll Lock
        LockingScrollLock = 0x84 => "locking-scroll-lock" | "locking-scrolllock",
        /// Keypad Comma
        KeyPadComma = 0x85 => "keypad-comma",
        /// Keypad Equal Sign
        KeyPadEqualSign = 0x86 => "keypad-equal-sign",
        /// Keyboard International1
        Ro = 0x87 => "ro",
        /// Keyboard International2
        KatakanaHiragana = 0x88 => "katakana-hiragana",
        /// Keyboard International3
        Yen = 0x89 => "yen",
        /// Keyboard International4
        Henkan = 0x8a => "henkan",
        /// Keyboard International5
        Munenkan = 0x8b => "munenkan",
        /// Keyboard International6
        KeyPadJpComma = 0x8c => "keypad-jp-comma",
        // 0x8d  Keyboard International7
        // 0x8e  Keyboard International8
        // 0x8f  Keyboard International9
        /// Keyboard LANG1
        Hangeul = 0x90 => "hangeul",
        /// Keyboard LANG2
        Hanja = 0x91 => "hanja",
        /// Keyboard LANG3
        Katakana = 0x92 => "katakana",
        /// Keyboard LANG4
        Hiragana = 0x93 => "hiragana",
        /// Keyboard LANG5
        ZankakuHankaku = 0x94 => "zenkaku-hankaku",
        // 0x95  Keyboard LANG6
        // 0x96  Keyboard LANG7
        // 0x97  Keyboard LANG8
        // 0x98  Keyboard LANG9
        // 0x99  Keyboard Alternate Erase
        // 0x9a  Keyboard SysReq/Attention
        // 0x9b  Keyboard Cancel
        // 0x9c  Keyboard Clear
        // 0x9d  Keyboard Prior
        // 0x9e  Keyboard Return
        // 0x9f  Keyboard Separator
        // 0xa0  Keyboard Out
        // 0xa1  Keyboard Oper
        // 0xa2  Keyboard Clear/Again
        // 0xa3  Keyboard CrSel/Props
        // 0xa4  Keyboard ExSel

        // 0xb0  Keypad 00
        // 0xb1  Keypad 000
        // 0xb2  Thousands Separator
        // 0xb3  Decimal Separator
        // 0xb4  Currency Unit
        // 0xb5  Currency Sub-unit
        /// Keypad (
        KeyPadLeftParen = 0xb6 => "keypad-left-paren",
        /// Keypad )
        KeyPadRightParen = 0xb7 => "keypad-right-paren",
        // 0xb8  Keypad {
        // 0xb9  Keypad }
        // 0xba  Keypad Tab
        // 0xbb  Keypad Backspace
        // 0xbc  Keypad A
        // 0xbd  Keypad B
        // 0xbe  Keypad C
        // 0xbf  Keypad D
        // 0xc0  Keypad E
        // 0xc1  Keypad F
        // 0xc2  Keypad XOR
        // 0xc3  Keypad ^
        // 0xc4  Keypad %
        // 0xc5  Keypad <
        // 0xc6  Keypad >
        // 0xc7  Keypad &
        // 0xc8  Keypad &&
        // 0xc9  Keypad |
        // 0xca  Keypad ||
        // 0xcb  Keypad :
        // 0xcc  Keypad #
        // 0xcd  Keypad Space
        // 0xce  Keypad @
        // 0xcf  Keypad !
        // 0xd0  Keypad Memory Store
        // 0xd1  Keypad Memory Recall
        // 0xd2  Keypad Memory Clear
        // 0xd3  Keypad Memory Add
        // 0xd4  Keypad Memory Subtract
        // 0xd5  Keypad Memory Multiply
        // 0xd6  Keypad Memory Divide
        // 0xd7  Keypad +/-
        // 0xd8  Keypad Clear
        // 0xd9  Keypad Clear Entry
        // 0xda  Keypad Binary
        // 0xdb  Keypad Octal
        // 0xdc  Keypad Decimal
        // 0xdd  Keypad Hexadecimal
        /// Keyboard Left Control
        LeftCtrl = 0xe0 => "left-ctrl" | "ctrl",
        /// Keyboard Left Shift
        LeftShift = 0xe1 => "left-shift" | "shift",
        /// Keyboard Left Alt
        LeftAlt = 0xe2 => "left-alt" | "alt",
        /// Keyboard Left GUI
        LeftMeta = 0xe3 => "left-meta" | "meta",
        /// Keyboard Right Control
        RightCtrl = 0xe4 => "right-ctrl",
        /// Keyboard Right Shift
        RightShift = 0xe5 => "right-shift",
        /// Keyboard Right Alt
        RightAlt = 0xe6 => "right-alt",
        /// Keyboard Right GUI
        RightMeta = 0xe7 => "right-meta",
    }
}

impl From<Modifiers> for Key {
    fn from(mods: Modifiers) -> Self {
        let off = mods.bits.trailing_zeros() as u8;
        if off < 8 {
            Key::from(0xe0u8 + off)
        } else {
            Key::None
        }
    }
}

impl From<Key> for Modifiers {
    fn from(key: Key) -> Self {
        let code = key as u8;
        Modifiers::from(if code >= 0xe0 && code <= 0xe7 {
            1 << (code - 0xe0)
        } else {
            0
        })
    }
}

impl Key {
    /// Converts from raw value safely
    pub fn safe_from(raw: u8) -> Option<Self> {
        if raw <= 0xe7 {
            Some(From::from(raw))
        } else {
            None
        }
    }
}

impl Default for Key {
    fn default() -> Self {
        Self::None
    }
}

code_enum! {
    /// LED code
    Led: u8 {
        /// No LED
        None = 0x00 => "none",
        /// Num lock
        NumLock = 0x01 => "num-lock" | "numlock",
        /// Caps lock
        CapsLock = 0x02 => "caps-lock" | "capslock",
        /// Scroll lock
        ScrollLock = 0x03 => "scroll-lock" | "scrollock",
        /// Compose LED
        Compose = 0x04 => "compose",
        /// Kana LED
        Kana = 0x05 => "kana",
    }
}

impl Led {
    /// Converts from raw value safely
    pub fn safe_from(raw: u8) -> Option<Self> {
        if raw >= 0x01 && raw <= 0x05 {
            Some(From::from(raw))
        } else {
            None
        }
    }
}

/// LED mask
#[bitmask(u8)]
pub enum Leds {
    /// Num lock
    NumLock = 0x01,
    /// Caps lock
    CapsLock = 0x02,
    /// Scroll lock
    ScrollLock = 0x04,
    /// Compose LED
    Compose = 0x08,
    /// Kana LED
    Kana = 0x10,
}

const_assert_eq!(size_of::<Leds>(), 1);

impl Default for Leds {
    fn default() -> Self {
        Self::none()
    }
}

impl Leds {
    /// Convert from raw value safely
    pub fn safe_from(raw: u8) -> Option<Self> {
        if raw & 0xe0 == 0 {
            Some(Self::from(raw))
        } else {
            None
        }
    }
}

impl From<Led> for Leds {
    fn from(code: Led) -> Self {
        Self::from(if matches!(code, Led::None) {
            0u8
        } else {
            1u8 << (code as u8 - Led::NumLock as u8)
        })
    }
}

impl From<Leds> for Led {
    fn from(leds: Leds) -> Self {
        let off = leds.bits.trailing_zeros() as u8;
        if off < 8 {
            Led::from(Led::NumLock as u8 + off)
        } else {
            Led::None
        }
    }
}

serde_num! {
    Modifiers: u8, "a modifier mask";
    Leds: u8, "a LED mask";
    Key: u8, "a numeric key code";
    Led: u8, "a numeric LED code";
}

/// Keyboard input report
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C, packed)]
pub struct KeyboardInput {
    /// Pressed modifier keys
    #[cfg_attr(feature = "serde", serde(rename = "mod"))]
    modifier: Modifiers,
    /// Reserved unused
    #[cfg_attr(feature = "serde", serde(skip))]
    reserved: u8,
    /// Pressed key codes
    #[cfg_attr(feature = "serde", serde(rename = "key"))]
    keycodes: [Key; 6],
}

const_assert_eq!(size_of::<KeyboardInput>(), 8);

impl KeyboardInput {
    /// Get iterator over pressed keys
    ///
    /// Modifiers also returned as key codes before ordinary keys
    pub fn pressed<'i>(&'i self) -> AllPressedKeys<'i> {
        AllPressedKeys {
            report: self,
            modifier: true,
            element: 0,
        }
    }

    /// Get modifier mask
    pub fn mods(&self) -> Modifiers {
        self.modifier
    }

    /// Get number of all pressed keys
    pub fn count_pressed(&self) -> usize {
        self.count_pressed_mods() + self.count_pressed_keys()
    }

    /// Get number of pressed modifiers
    pub fn count_pressed_mods(&self) -> usize {
        self.modifier.bits.count_ones() as _
    }

    /// Get number of pressed keys excepting modifiers
    pub fn count_pressed_keys(&self) -> usize {
        for i in 0..6 {
            if matches!(self.keycodes[i], Key::None) {
                return i;
            }
        }
        return 6;
    }

    /// Get slice of pressed keys excepting modifiers
    pub fn pressed_keys(&self) -> &[Key] {
        &self.keycodes[0..self.count_pressed_keys()]
    }

    /// Press or release modifiers only
    pub fn change_mods(&mut self, mask: Modifiers, state: bool) {
        if state {
            self.modifier |= mask;
        } else {
            self.modifier &= !mask;
        }
    }

    /// Press modifiers only
    pub fn press_mods(&mut self, mask: Modifiers) {
        self.change_mods(mask, true);
    }

    /// Release modifiers only
    pub fn release_mods(&mut self, mask: Modifiers) {
        self.change_mods(mask, false);
    }

    /// Press or release key
    pub fn change_key(&mut self, key: Key, state: bool) {
        if matches!(key, Key::None) {
            return;
        }
        let modifier = Modifiers::from(key);
        if modifier.bits == 0 {
            // ordinary key
            let mut len = self.count_pressed_keys();
            if state {
                // press key
                if len < 6 {
                    for i in 0..len {
                        if self.keycodes[i] == key {
                            // key already pressed
                            return;
                        }
                    }
                    self.keycodes[len] = key;
                }
            } else {
                // release key
                let mut i = 0;
                while i < len {
                    // find key in pressed
                    if self.keycodes[i] == key {
                        len -= 1;
                        for j in i..len {
                            // remove key
                            self.keycodes[j] = self.keycodes[j + 1];
                        }
                        self.keycodes[len] = Key::None;
                    } else {
                        i += 1;
                    }
                }
            }
        } else {
            // modifier key
            if state {
                // press key
                self.modifier |= modifier;
            } else {
                // release key
                self.modifier &= !modifier;
            }
        }
    }

    /// Press key
    pub fn press_key(&mut self, key: Key) {
        self.change_key(key, true);
    }

    /// Release key
    pub fn release_key(&mut self, key: Key) {
        self.change_key(key, false);
    }
}

impl Extend<StateChange<Modifiers>> for KeyboardInput {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = StateChange<Modifiers>>,
    {
        for StateChange { data, state } in iter {
            self.change_mods(data, state);
        }
    }
}

impl Extend<StateChange<Key>> for KeyboardInput {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = StateChange<Key>>,
    {
        for StateChange { data, state } in iter {
            self.change_key(data, state);
        }
    }
}

impl Extend<KeyboardInput> for KeyboardInput {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = KeyboardInput>,
    {
        for item in iter {
            *self = item;
        }
    }
}

/// An iterator over all pressed keys including modifiers
pub struct AllPressedKeys<'i> {
    report: &'i KeyboardInput,
    modifier: bool,
    element: u8,
}

impl<'i> Iterator for AllPressedKeys<'i> {
    type Item = Key;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.modifier {
                let modifier = Modifiers::from(1 << self.element);
                self.element += 1;
                if self.element >= 8 {
                    self.modifier = false;
                    self.element = 0;
                }
                let key = Key::from(self.report.modifier & modifier);
                if !matches!(key, Key::None) {
                    return Some(key);
                }
            } else if self.element < 6 {
                let key = self.report.keycodes[self.element as usize];
                self.element += 1;
                if !matches!(key, Key::None) {
                    return Some(key);
                }
            } else {
                return None;
            }
        }
    }
}

/// Keyboard output report
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C, packed)]
pub struct KeyboardOutput {
    /// Active LEDs
    #[cfg_attr(feature = "serde", serde(rename = "led"))]
    leds: Leds,
}

const_assert_eq!(size_of::<KeyboardOutput>(), 1);

impl KeyboardOutput {
    /// Get reference to LEDs mask
    pub fn leds(&self) -> &Leds {
        &self.leds
    }

    /// Get number of lit LEDs
    pub fn count_lit(&self) -> usize {
        self.leds.bits.count_ones() as _
    }

    /// Get iterator over lit LEDs
    pub fn lit(&self) -> LitLeds {
        LitLeds {
            report: self,
            element: 0,
        }
    }

    /// Change LEDs state
    pub fn change_leds(&mut self, leds: Leds, state: bool) {
        if state {
            self.leds |= leds;
        } else {
            self.leds &= !leds;
        }
    }

    /// Turn LEDs on
    pub fn on_leds(&mut self, leds: Leds) {
        self.change_leds(leds, true);
    }

    /// Turn LEDs off
    pub fn off_leds(&mut self, leds: Leds) {
        self.change_leds(leds, false);
    }

    /// Change LED state
    pub fn change_led(&mut self, led: Led, state: bool) {
        self.change_leds(led.into(), state);
    }

    /// Turn LED on
    pub fn on_led(&mut self, led: Led) {
        self.change_led(led, true);
    }

    /// Turn LED off
    pub fn off_led(&mut self, led: Led) {
        self.change_led(led, false);
    }
}

impl Extend<StateChange<Leds>> for KeyboardOutput {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = StateChange<Leds>>,
    {
        for StateChange { data, state } in iter {
            self.change_leds(data, state);
        }
    }
}

impl Extend<StateChange<Led>> for KeyboardOutput {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = StateChange<Led>>,
    {
        for StateChange { data, state } in iter {
            self.change_led(data, state);
        }
    }
}

impl Extend<KeyboardOutput> for KeyboardOutput {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = KeyboardOutput>,
    {
        for item in iter {
            *self = item;
        }
    }
}

/// An iterator over lit LEDs
pub struct LitLeds<'i> {
    report: &'i KeyboardOutput,
    element: u8,
}

impl<'i> Iterator for LitLeds<'i> {
    type Item = Led;

    fn next(&mut self) -> Option<Self::Item> {
        while self.element < 8 {
            let led = Leds::from(1u8 << self.element);
            self.element += 1;
            if self.report.leds.contains(led) {
                return Some(led.into());
            }
        }
        None
    }
}

raw_ref! {
    Modifiers;
    Leds;
    Key;
    Led;
    KeyboardInput;
    KeyboardOutput;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mod_mask_to_key_code() {
        assert_eq!(Key::from(Modifiers::from(0)), Key::None);
        assert_eq!(Key::from(Modifiers::LeftCtrl), Key::LeftCtrl);
        assert_eq!(Key::from(Modifiers::RightAlt), Key::RightAlt);
        assert_eq!(Key::from(Modifiers::RightMeta), Key::RightMeta);
    }

    #[test]
    fn key_code_to_mod_mask() {
        assert_eq!(Modifiers::from(Key::A), Modifiers::default());
        assert_eq!(Modifiers::from(Key::LeftCtrl), Modifiers::LeftCtrl);
        assert_eq!(Modifiers::from(Key::RightShift), Modifiers::RightShift);
        assert_eq!(Modifiers::from(Key::RightMeta), Modifiers::RightMeta);
    }

    #[test]
    fn keyboard_input() {
        let mut report = KeyboardInput::default();
        assert_eq!(report.count_pressed_keys(), 0);
        assert_eq!(report.count_pressed(), 0);

        report.press_mods(Modifiers::LeftAlt | Modifiers::RightShift | Modifiers::RightMeta);
        assert_eq!(report.count_pressed_keys(), 0);
        assert_eq!(report.count_pressed_mods(), 3);
        assert_eq!(report.count_pressed(), 3);

        report.press_key(Key::A);
        assert_eq!(report.count_pressed_keys(), 1);
        assert_eq!(report.count_pressed(), 4);

        report.press_key(Key::Esc);
        assert_eq!(report.count_pressed_keys(), 2);
        assert_eq!(report.count_pressed_mods(), 3);
        assert_eq!(report.count_pressed(), 5);

        report.release_mods(Modifiers::RightMeta);
        assert_eq!(report.count_pressed_keys(), 2);
        assert_eq!(report.count_pressed_mods(), 2);
        assert_eq!(report.count_pressed(), 4);

        report.press_key(Key::Enter);
        assert_eq!(report.count_pressed_keys(), 3);

        report.press_key(Key::A);
        assert_eq!(report.count_pressed_keys(), 3);

        report.press_key(Key::B);
        assert_eq!(report.count_pressed_keys(), 4);

        report.press_key(Key::LeftMeta);
        assert_eq!(report.count_pressed_keys(), 4);
        assert_eq!(report.count_pressed(), 7);

        report.release_key(Key::Esc);
        assert_eq!(report.count_pressed_keys(), 3);

        report.press_key(Key::Tab);
        assert_eq!(report.count_pressed_keys(), 4);

        report.release_key(Key::B);
        assert_eq!(report.count_pressed_keys(), 3);
        assert_eq!(report.count_pressed(), 6);

        let mut iter = report.pressed();
        assert_eq!(iter.next(), Some(Key::LeftAlt));
        assert_eq!(iter.next(), Some(Key::LeftMeta));
        assert_eq!(iter.next(), Some(Key::RightShift));
        assert_eq!(iter.next(), Some(Key::A));
        assert_eq!(iter.next(), Some(Key::Enter));
        assert_eq!(iter.next(), Some(Key::Tab));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn keyboard_output() {
        let mut report = KeyboardOutput::default();
        assert_eq!(report.count_lit(), 0);

        report.on_leds(Leds::NumLock | Leds::CapsLock);
        assert_eq!(report.count_lit(), 2);

        report.off_led(Led::CapsLock);
        eprintln!("{:?}", report.leds.bits);
        assert_eq!(report.count_lit(), 1);

        report.on_leds(Leds::ScrollLock);
        assert_eq!(report.count_lit(), 2);

        report.on_led(Led::NumLock);
        assert_eq!(report.count_lit(), 2);

        report.on_led(Led::Compose);
        assert_eq!(report.count_lit(), 3);

        let mut iter = report.lit();
        assert_eq!(iter.next(), Some(Led::NumLock));
        assert_eq!(iter.next(), Some(Led::ScrollLock));
        assert_eq!(iter.next(), Some(Led::Compose));
        assert_eq!(iter.next(), None);
    }
}
