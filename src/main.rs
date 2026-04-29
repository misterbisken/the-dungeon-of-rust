use std::io;

fn main() {
    println!("You awake in a dark dungeon!");
    println!("You can go left, forward, right");
    println!("What do you choose");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Pressed wrong button");

    println!("You pressed to go forward");
}
