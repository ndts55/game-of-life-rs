extern crate ncurses;

use super::field::{Field, next_generation};
use std::thread::sleep;
use std::time::Duration;
use ncurses::*;

const KEY_Q: i32 = 113;

pub fn play(field: Field, dur: Duration) {
    let mut field = field;
    initialize_ncurses(field.shape());
    loop {
        draw_field(&field);
        sleep(dur);
        field = next_generation(&field);
        // check if field is empty -> break
        // check if user entered q -> break
        if getch() == KEY_Q {
            break;
        }
    }
}

fn initialize_ncurses(dimensions: &[usize]) {
    let window =  initscr();
    keypad(stdscr(), true);
    nodelay(window, true);
}

fn draw_field(field: &Field) {
    // just print for now
    // TODO use ncurses
    println!("{:8.4}", field);
    print!("\n");
}
