extern crate ncurses;

use super::field::{next_generation, random_field, Field};
use ncurses::*;
use std::thread::sleep;
use std::time::Duration;

const KEY_Q: i32 = 113;

pub fn play() {
    // TODO get window size and initialize field with that size
    start(random_field((10, 20)), Duration::from_millis(250));
}

pub fn start(field: Field, dur: Duration) {
    let mut field = field;
    initialize_ncurses();
    loop {
        draw_field(&field);
        sleep(dur);
        field = next_generation(&field);
        // check if field is same as before -> break
        // check if user entered q -> break
        if getch() == KEY_Q {
            break;
        }
    }

    endwin();
}

fn initialize_ncurses() {
    let window = initscr();
    keypad(stdscr(), true);
    nodelay(window, true);
    noecho();
}

fn draw_field(field: &Field) {
    clear();
    for ((y, x), &value) in field.indexed_iter() {
        mv(y as i32, x as i32);
        addch(chtype::from(value));
    }

    refresh();
}
