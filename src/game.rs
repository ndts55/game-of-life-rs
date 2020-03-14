extern crate ncurses;

use super::field::{next_generation, random_field, Field};
use ncurses::*;
use std::thread::sleep;
use std::time::Duration;

const KEY_Q: i32 = 113;

pub fn play() {
    initialize_ncurses();
    let mut width = 0;
    let mut height = 0;
    getmaxyx(stdscr(), &mut height, &mut width);
    start(
        random_field((height as usize, width as usize)),
        Duration::from_millis(39),
    );
}

fn initialize_ncurses() {
    let window = initscr();
    keypad(stdscr(), true);
    nodelay(window, true);
    noecho();
}

fn start(field: Field, dur: Duration) {
    let mut field = field;
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

fn draw_field(field: &Field) {
    clear();
    for ((y, x), &value) in field.indexed_iter() {
        mv(y as i32, x as i32);
        addch(chtype::from(value));
    }

    refresh();
}
