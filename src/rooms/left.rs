use crate::game::Game;
use std::io::{self, Write};

pub fn left(game: &mut Game) {
    //Funktionen som är om left
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    println!("You found a weapon!");
}
