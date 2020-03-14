use game_of_life::{random_field, next_generation};

fn main() {
    let a = random_field((10, 10));
    let b = next_generation(&a);
    println!("{:8.4}", a);
    println!("");
    println!("{:8.4}", b);
}
