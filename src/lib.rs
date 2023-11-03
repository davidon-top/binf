/*!
# binf, Binary flags
This crate aims to make working with binary/bit flags easier. It also provides a macro to add similar functionality to zig's packed structs with boolean fields.
*/

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
        for i in 0..8 {
            flags[i] = self.get_flag(i as u8);
        }
        flags
    }

    fn set_flags(&mut self, flags: &[bool]) {
        for (i, v) in flags.into_iter().enumerate() {
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
        for i in 0..16 {
            flags[i] = self.get_flag(i as u8);
        }
        flags
    }

    fn set_flags(&mut self, flags: &[bool]) {
        for (i, v) in flags.into_iter().enumerate() {
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
        for i in 0..32 {
            flags[i] = self.get_flag(i as u8);
        }
        flags
    }

    fn set_flags(&mut self, flags: &[bool]) {
        for (i, v) in flags.into_iter().enumerate() {
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
        for i in 0..64 {
            flags[i] = self.get_flag(i as u8);
        }
        flags
    }

    fn set_flags(&mut self, flags: &[bool]) {
        for (i, v) in flags.into_iter().enumerate() {
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
        for i in 0..128 {
            flags[i] = self.get_flag(i as u8);
        }
        flags
    }

    fn set_flags(&mut self, flags: &[bool]) {
        for (i, v) in flags.into_iter().enumerate() {
            if i >= 128 {
                break;
            }
            self.set_flag(i as u8, *v);
        }
    }
}
