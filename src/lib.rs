/*!
TODO documentation
*/

pub mod flag_utils;
pub mod macros;
#[cfg(test)]
mod test;

///struct that holds data for a flag and useful functions
pub struct Flag {
    value: u128,
}

impl Flag {
    /// set a flag value
    pub fn set_flag(&mut self, flag: u128, value: bool) {
        if value {
            self.value |= 1 << flag;
        } else {
            self.value &= !(1 << flag);
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
            ret.push(&self.value & (1 << i) != 0);
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

    /// sets internal value to this
    pub fn set_value(&mut self, value: u128) {
        self.value = value;
    }

    /// initializes a Flag with 0, use flag_new! macro to create with flags instead
    pub fn new() -> Self {
        Self { value: 0, }
    }
}
