/*!
TODO documentation
*/

pub mod flag_utils;
pub mod macros;

///struct that holds data for a flag and useful functions
pub struct Flag {
    value: u128,
}

impl Flag {
    /// set a flag value
    pub fn set_flag(&mut self, flag: u32, value: bool) {
        if value {
            self.value |= 1 << flag;
        } else {
            if self.get_flag(flag) {
                self.value -= 1 << flag;
            }
        }
    }

    /// get a value of flag
    pub fn get_flag(&self, flag: u32) -> bool {
        (self.value & (1 << flag)) != 0
    }

    /// returns a vector with all enabled flags
    pub fn get_all_flags(&self) -> Vec<bool> {
        let mut ret = vec![];
        for i in 0..128 {
            ret.push(&self.value & (1u128 << i) != 0);
        }
        for i in (0..ret.len()).rev() {
            if ret[i] {
                ret.truncate(i + 1);
                break;
            }
        }
        ret
    }

    /// creates Flag from int
    pub fn new_from_value(value: u128) -> Self {
        Self {value}
    }

    /// returns flags value
    pub fn get(&self) -> u128 {
        self.value
    }

    /// initializes a Flag with 0, use flag_new! macro to create with flags instead
    pub fn new() -> Self {
        Self { value: 0, }
    }
}
