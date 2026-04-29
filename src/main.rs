use std::io;

fn main() {
    println!("You awake in a dark dungeon!");
    println!("You can go left, forward, right");
    println!("What do you choose");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "forward" => println!("Du valde forward"),
        "right" => println!("Du valde right"),
        "left" => println!("Du valde left"),
        _ => println!("..."),
    }
}
