use crate::game::Game;
use std::io::{self, Write};

pub fn right(game: &mut Game) {
    //Funkionen som är om right

    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    println!("You have stumbled on a monster");
    println!("Will you fight or run?");
    println!("");
    println!("");
    println!("(fight)    (run)");

    let mut acceptbattle = String::new();
    io::stdin()
        .read_line(&mut acceptbattle)
        .expect("Failed to read line");

    // match acceptbattle.trim(){

    //   "fight" => fight(),

    // _ => println!("Please "),

    //  }

    //println!("Du hittade en nyckel!");
    //game.nyckel = true;
}
