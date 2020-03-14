use game_of_life::field::random_field;
use game_of_life::game::play;
use std::time::Duration;

fn main() {
    play(random_field((10, 10)), Duration::from_millis(250));
}
