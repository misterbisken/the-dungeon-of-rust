use crate::game::Game;
use std::io::{self, Write};

pub fn forward(game: &mut Game) {
    //Funktionen som är om forward
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    println!("Du står framför en stor låst dörr!");
    if !game.nyckel {
        println!("Du har ingen nyckel!")
    } else {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();

        println!("Du vann!");
        game.running = false;
    }
}
