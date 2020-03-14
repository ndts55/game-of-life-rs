use super::{Field, next_generation};
use std::thread::sleep;
use std::time::Duration;

pub fn play(field: Field, dur: Duration) {
    let mut field = field;
    loop {
        // draw field
        draw_field(&field);
        // wait for pause amount of time
        sleep(dur);
        // calculate next generation
        field = next_generation(&field);
    }
}

fn draw_field(field: &Field) {
    // just print for now
    // TODO use ncurses
    println!("{:8.4}", field);
    print!("\n");
}
