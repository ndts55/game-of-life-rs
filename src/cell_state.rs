use std::fmt;
use CellState::{Dead, Live};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CellState {
    Dead = 0,
    Live = 1,
}

impl From<&u8> for CellState {
    fn from(num: &u8) -> Self {
        match num {
            0 => Dead,
            1 => Live,
            _ => panic!("panic"),
        }
    }
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dead => " ",
                Live => "#",
            }
        )
    }
}