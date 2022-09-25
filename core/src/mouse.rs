use bitmask_enum::bitmask;
use core::mem::{size_of, transmute};
use static_assertions::const_assert_eq;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Class, StateChange, ValueChange};

/// Mouse HID class
#[derive(Clone, Copy, Debug)]
pub struct Mouse;

impl Class for Mouse {
    type Input = MouseInput;
    type Output = MouseOutput;

    fn input(&self) -> Self::Input {
        Self::Input::default()
    }

    fn output(&self) -> Self::Output {
        Self::Output::default()
    }
}

impl AsRef<str> for Mouse {
    fn as_ref(&self) -> &str {
        "mouse"
    }
}

impl core::fmt::Display for Mouse {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(self.as_ref())
    }
}

/// Button mask
#[bitmask(u8)]
pub enum Buttons {
    /// Primary button
    ///
    /// Usually left for right hand.
    Primary = 0x01,

    /// Secondary button
    ///
    /// Usually right for right hand.
    Secondary = 0x02,

    /// Tertiary button
    ///
    /// Usually middle.
    Tertiary = 0x04,
}

const_assert_eq!(size_of::<Buttons>(), 1);

impl Default for Buttons {
    fn default() -> Self {
        Self::none()
    }
}

impl Buttons {
    /// Converts from raw value safely
    pub fn safe_from(raw: u8) -> Option<Self> {
        Some(Self::from(raw))
    }
}

code_enum! {
    /// Button code
    Button: u8 {
        /// No button
        None = 0x00 => "none" | "0",

        /// Primary button
        ///
        /// Usually left for right hand.
        Primary = 0x01 => "primary" | "first" | "1",

        /// Secondary button
        ///
        /// Usually right for right hand.
        Secondary = 0x02 => "secondary" | "second" | "2",

        /// Tertiary button
        ///
        /// Usually middle.
        Tertiary = 0x03 => "tertiary" | "third" | "3",
    }
}

impl From<Buttons> for Button {
    fn from(mods: Buttons) -> Self {
        let off = mods.bits.trailing_zeros() as u8;
        if off < 3 {
            Button::from(off + 1)
        } else {
            Button::None
        }
    }
}

impl From<Button> for Buttons {
    fn from(key: Button) -> Self {
        let code = key as u8;
        Buttons::from(if code != 0 { 1 << (code - 1) } else { 0 })
    }
}

impl Button {
    /// Converts from raw value safely
    pub fn safe_from(raw: u8) -> Option<Self> {
        if raw <= 3 {
            Some(From::from(raw))
        } else {
            None
        }
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::None
    }
}

serde_num! {
    Buttons: u8, "a button mask";
    Button: u8, "a numeric button code";
}

/// Mouse input report
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C, packed)]
pub struct MouseInput {
    /// Pressed buttons
    #[cfg_attr(feature = "serde", serde(rename = "b"))]
    button: Buttons,
    /// Pointer coordinates
    #[cfg_attr(feature = "serde", serde(rename = "p"))]
    pointer: (i16, i16),
    /// Mouse wheel coordinate
    #[cfg_attr(feature = "serde", serde(rename = "w"))]
    wheel: i8,
}

const_assert_eq!(size_of::<MouseInput>(), 6);

impl MouseInput {
    /// Get button mask
    pub fn mods(&self) -> Buttons {
        self.button
    }

    /// Get number of pressed buttons
    pub fn count_pressed(&self) -> usize {
        self.button.bits.count_ones() as _
    }

    /// Get iterator over pressed buttons
    pub fn pressed(&self) -> PressedButtons<'_> {
        PressedButtons {
            report: self,
            element: 0,
        }
    }

    /// Press or release buttons
    pub fn change_buttons(&mut self, mask: Buttons, state: bool) {
        if state {
            self.button |= mask;
        } else {
            self.button &= !mask;
        }
    }

    /// Press buttons
    pub fn press_buttons(&mut self, mask: Buttons) {
        self.change_buttons(mask, true);
    }

    /// Release buttons
    pub fn release_buttons(&mut self, mask: Buttons) {
        self.change_buttons(mask, false);
    }

    /// Press or release button
    pub fn change_button(&mut self, code: Button, state: bool) {
        self.change_buttons(code.into(), state);
    }

    /// Press button
    pub fn press_button(&mut self, code: Button) {
        self.change_button(code, true);
    }

    /// Release button
    pub fn release_button(&mut self, code: Button) {
        self.change_button(code, false);
    }

    /// Get XY coordinates
    pub fn pointer(&self) -> (i16, i16) {
        self.pointer
    }

    /// Set XY coordinates
    pub fn set_pointer(&mut self, pointer: (i16, i16)) {
        self.pointer = pointer;
    }

    /// Change XY coordinates
    pub fn change_pointer(&mut self, pointer: (i16, i16), relative: bool) {
        if relative {
            self.pointer = (self.pointer.0 + pointer.0, self.pointer.1 + pointer.1);
        } else {
            self.pointer = pointer;
        }
    }

    /// Get wheel value
    pub fn wheel(&self) -> i8 {
        self.wheel
    }

    /// Set wheel value
    pub fn set_wheel(&mut self, wheel: i8) {
        self.wheel = wheel;
    }

    /// Change wheel value
    pub fn change_wheel(&mut self, wheel: i8, relative: bool) {
        if relative {
            self.wheel += wheel;
        } else {
            self.wheel = wheel;
        }
    }

    /// Get changes between two reports
    ///
    /// Difference of two reports
    pub fn diff<'i>(
        &'i self,
        other: &'i Self,
        relative_pointer: bool,
        relative_wheel: bool,
    ) -> MouseInputChanges<'i> {
        MouseInputChanges {
            new: self,
            old: other,
            element: 0,
            relative_pointer,
            relative_wheel,
        }
    }
}

impl<'i> core::ops::Sub<&'i MouseInput> for &'i MouseInput {
    type Output = MouseInputChanges<'i>;

    fn sub(self, other: Self) -> Self::Output {
        self.diff(other, false, false)
    }
}

/// Change between mouse input reports
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseInputChange {
    /// Button state change
    Button(StateChange<Button>),
    /// Pointer coordinates change
    Pointer((i16, i16)),
    /// Wheel value change
    Wheel(i8),
}

/// Changes between mouse input reports
pub struct MouseInputChanges<'i> {
    new: &'i MouseInput,
    old: &'i MouseInput,
    element: u8,
    relative_pointer: bool,
    relative_wheel: bool,
}

impl<'i> Iterator for MouseInputChanges<'i> {
    type Item = MouseInputChange;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.element < 8 {
                // find changed buttons
                let buttons = Buttons::from(1 << self.element);
                self.element += 1;
                if Buttons::none() != ((self.new.button ^ self.old.button) & buttons) {
                    let button = Button::from(buttons);
                    let old_button = Button::from(self.old.button & buttons);
                    return Some(MouseInputChange::Button(StateChange::new(
                        button,
                        matches!(old_button, Button::None),
                    )));
                }
            } else if self.element < 9 {
                // changed pointer coords
                self.element += 1;
                if self.new.pointer.0 != self.old.pointer.0
                    || self.new.pointer.1 != self.old.pointer.1
                {
                    return Some(MouseInputChange::Pointer(if self.relative_pointer {
                        (
                            self.new.pointer.0 - self.old.pointer.0,
                            self.new.pointer.1 - self.old.pointer.1,
                        )
                    } else {
                        (self.new.pointer.0, self.new.pointer.1)
                    }));
                }
            } else if self.element < 10 {
                self.element += 1;
                if self.new.wheel != self.old.wheel {
                    return Some(MouseInputChange::Wheel(if self.relative_wheel {
                        self.new.wheel - self.old.wheel
                    } else {
                        self.new.wheel
                    }));
                }
            } else {
                return None;
            }
        }
    }
}

impl Extend<StateChange<Buttons>> for MouseInput {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = StateChange<Buttons>>,
    {
        for StateChange { data, state } in iter {
            self.change_buttons(data, state);
        }
    }
}

impl Extend<StateChange<Button>> for MouseInput {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = StateChange<Button>>,
    {
        for StateChange { data, state } in iter {
            self.change_button(data, state);
        }
    }
}

impl Extend<ValueChange<(i16, i16)>> for MouseInput {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = ValueChange<(i16, i16)>>,
    {
        for ValueChange { data, relative } in iter {
            self.change_pointer(data, relative);
        }
    }
}

impl Extend<(i16, i16)> for MouseInput {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (i16, i16)>,
    {
        for data in iter {
            self.set_pointer(data);
        }
    }
}

impl Extend<ValueChange<i8>> for MouseInput {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = ValueChange<i8>>,
    {
        for ValueChange { data, relative } in iter {
            self.change_wheel(data, relative)
        }
    }
}

impl Extend<MouseInput> for MouseInput {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = MouseInput>,
    {
        for item in iter {
            *self = item;
        }
    }
}

/// An iterator over pressed buttons
pub struct PressedButtons<'i> {
    report: &'i MouseInput,
    element: u8,
}

impl<'i> Iterator for PressedButtons<'i> {
    type Item = Button;

    fn next(&mut self) -> Option<Self::Item> {
        while self.element < 8 {
            let mask = Buttons::from(1u8 << self.element);
            self.element += 1;
            if self.report.button.contains(mask) {
                return Some(mask.into());
            }
        }
        None
    }
}

/// Keyboard output report
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C, packed)]
pub struct MouseOutput {}

const_assert_eq!(size_of::<MouseOutput>(), 0);

raw_ref! {
    Buttons;
    Button;
    MouseInput;
    MouseOutput;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn button_mask_to_code() {
        assert_eq!(Button::from(Buttons::default()), Button::None);
        assert_eq!(Button::from(Buttons::from(0)), Button::None);
        assert_eq!(Button::from(Buttons::Primary), Button::Primary);
        assert_eq!(Button::from(Buttons::Secondary), Button::Secondary);
        assert_eq!(Button::from(Buttons::Tertiary), Button::Tertiary);
    }

    #[test]
    fn button_code_to_mask() {
        assert_eq!(Buttons::from(Button::None), Buttons::default());
        assert_eq!(Buttons::from(Button::Primary), Buttons::Primary);
        assert_eq!(Buttons::from(Button::Secondary), Buttons::Secondary);
        assert_eq!(Buttons::from(Button::Tertiary), Buttons::Tertiary);
    }

    #[test]
    fn mouse_input_diff() {
        let mut old = MouseInput::default();

        old.press_button(Button::Primary);
        old.press_button(Button::Tertiary);
        old.set_pointer((120, 450));
        old.set_wheel(7);

        let mut new = MouseInput::default();

        new.press_button(Button::Tertiary);
        new.press_button(Button::Secondary);
        new.set_pointer((120, 150));
        new.set_wheel(-1);

        let mut changes = &new - &old;

        assert_eq!(
            changes.next(),
            Some(MouseInputChange::Button(StateChange::release(
                Button::Primary
            )))
        );
        assert_eq!(
            changes.next(),
            Some(MouseInputChange::Button(StateChange::press(
                Button::Secondary
            )))
        );
        assert_eq!(changes.next(), Some(MouseInputChange::Pointer((120, 150))));
        assert_eq!(changes.next(), Some(MouseInputChange::Wheel(-1)));
        assert_eq!(changes.next(), None);

        new.release_button(Button::Secondary);
        new.set_wheel(7);

        let mut changes = new.diff(&old, true, true);

        assert_eq!(
            changes.next(),
            Some(MouseInputChange::Button(StateChange::release(
                Button::Primary
            )))
        );
        assert_eq!(changes.next(), Some(MouseInputChange::Pointer((0, -300))));
        assert_eq!(changes.next(), None);
    }
}
