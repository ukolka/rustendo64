use std::fmt;

const RAM_SIZE: usize = 4 * 1024 * 1024;

pub struct Interconnect {
    ram: Vec<u16>
}

impl fmt::Debug for Interconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl Default for Interconnect {
    fn default() -> Self {
        Interconnect {
            ram: vec![0; RAM_SIZE]
        }
    }
}
