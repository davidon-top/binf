/*!
# binf, Binary flags
This crate aims to make working with binary/bit flags easier. It also provides a macro to add similar functionality to zig's packed structs with boolean fields.
*/

use std::ops::{Deref, DerefMut};

pub use binf_macros::*;

/// A trait for types that can be used as bit flags.
pub trait BitFlag {
    type T;
    /// Sets the flag at the given position to the given value.
    /// I don't know what happens if the position is larger than the number of bits in the type.
    fn set_flag(&mut self, position: u8, value: bool);
    /// Returns the value of the flag at the given position.
    /// I don't know what happens if the position is larger than the number of bits in the type.
    fn get_flag(&self, position: u8) -> bool;
    /// Returns an array of bools representing the flags.
    /// The first element in the array is the flag at position 0.
    fn flags(&self) -> Self::T;
    /// Sets the flags to the given values.
    /// flags can be any size, but if it is larger than the number of bits in the type only the first bits will be used.
    /// So if this is u8 flags should be 8 or less any more are ignored.
    fn set_flags(&mut self, flags: &[bool]);
}

/// implementation for u8. u8 means it can store 8 flags 1 flag per bit.
impl BitFlag for u8 {
    type T = [bool; 8];
    fn set_flag(&mut self, position: u8, value: bool) {
        if value {
            *self |= 1 << position;
        } else {
            *self &= !(1 << position);
        }
    }

    fn get_flag(&self, position: u8) -> bool {
        (*self & (1 << position)) != 0
    }

    fn flags(&self) -> Self::T {
        let mut flags = [false; 8];
        for (i, flag) in flags.iter_mut().enumerate() {
            *flag = self.get_flag(i as u8);
        }
        flags
    }

    fn set_flags(&mut self, flags: &[bool]) {
        for (i, v) in flags.iter().enumerate() {
            if i >= 8 {
                break;
            }
            self.set_flag(i as u8, *v);
        }
    }
}

impl BitFlag for u16 {
    type T = [bool; 16];
    fn set_flag(&mut self, position: u8, value: bool) {
        if value {
            *self |= 1 << position;
        } else {
            *self &= !(1 << position);
        }
    }

    fn get_flag(&self, position: u8) -> bool {
        (*self & (1 << position)) != 0
    }

    fn flags(&self) -> Self::T {
        let mut flags = [false; 16];
        for (i, flag) in flags.iter_mut().enumerate() {
            *flag = self.get_flag(i as u8);
        }
        flags
    }

    fn set_flags(&mut self, flags: &[bool]) {
        for (i, v) in flags.iter().enumerate() {
            if i >= 16 {
                break;
            }
            self.set_flag(i as u8, *v);
        }
    }
}

impl BitFlag for u32 {
    type T = [bool; 32];
    fn set_flag(&mut self, position: u8, value: bool) {
        if value {
            *self |= 1 << position;
        } else {
            *self &= !(1 << position);
        }
    }

    fn get_flag(&self, position: u8) -> bool {
        (*self & (1 << position)) != 0
    }

    fn flags(&self) -> Self::T {
        let mut flags = [false; 32];
        for (i, flag) in flags.iter_mut().enumerate() {
            *flag = self.get_flag(i as u8);
        }
        flags
    }

    fn set_flags(&mut self, flags: &[bool]) {
        for (i, v) in flags.iter().enumerate() {
            if i >= 32 {
                break;
            }
            self.set_flag(i as u8, *v);
        }
    }
}

impl BitFlag for u64 {
    type T = [bool; 64];
    fn set_flag(&mut self, position: u8, value: bool) {
        if value {
            *self |= 1 << position;
        } else {
            *self &= !(1 << position);
        }
    }

    fn get_flag(&self, position: u8) -> bool {
        (*self & (1 << position)) != 0
    }

    fn flags(&self) -> Self::T {
        let mut flags = [false; 64];
        for (i, flag) in flags.iter_mut().enumerate() {
            *flag = self.get_flag(i as u8);
        }
        flags
    }

    fn set_flags(&mut self, flags: &[bool]) {
        for (i, v) in flags.iter().enumerate() {
            if i >= 64 {
                break;
            }
            self.set_flag(i as u8, *v);
        }
    }
}

impl BitFlag for u128 {
    type T = [bool; 128];
    fn set_flag(&mut self, position: u8, value: bool) {
        if value {
            *self |= 1 << position;
        } else {
            *self &= !(1 << position);
        }
    }

    fn get_flag(&self, position: u8) -> bool {
        (*self & (1 << position)) != 0
    }

    fn flags(&self) -> Self::T {
        let mut flags = [false; 128];
        for (i, flag) in flags.iter_mut().enumerate() {
            *flag = self.get_flag(i as u8);
        }
        flags
    }

    fn set_flags(&mut self, flags: &[bool]) {
        for (i, v) in flags.iter().enumerate() {
            if i >= 128 {
                break;
            }
            self.set_flag(i as u8, *v);
        }
    }
}

/// A wrapper around a type that implements BitFlag. In case you don't want to import the trait and
/// see the trait methods on each unsized integer.
pub struct BitFlags<T>(T);

impl<T: BitFlag> BitFlags<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    /// Sets the flag at the given position to the given value.
    /// I don't know what happens if the position is larger than the number of bits in the type.
    pub fn set_flag(&mut self, position: u8, value: bool) {
        self.0.set_flag(position, value);
    }

    /// Returns the value of the flag at the given position.
    /// I don't know what happens if the position is larger than the number of bits in the type.
    pub fn get_flag(&self, position: u8) -> bool {
        self.0.get_flag(position)
    }

    /// Returns an array of bools representing the flags.
    /// The first element in the array is the flag at position 0.
    pub fn flags(&self) -> T::T {
        self.0.flags()
    }

    /// Sets the flags to the given values.
    /// flags can be any size, but if it is larger than the number of bits in the type only the first bits will be used.
    /// So if this is u8 flags should be 8 or less any more are ignored.
    pub fn set_flags(&mut self, flags: &[bool]) {
        self.0.set_flags(flags);
    }
}

impl<T: BitFlag> Deref for BitFlags<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: BitFlag> DerefMut for BitFlags<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
