use std::fmt;
use CellState::{Dead, Live};
use ncurses::chtype;

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

impl From<CellState> for chtype {
    fn from(state: CellState) -> Self {
        match state {
            Dead => ' ' as chtype,
            Live => '#' as chtype
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