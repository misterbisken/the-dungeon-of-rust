use crate::game::Game;
use std::io::{self, Write};

pub fn forward(game: &mut Game) {
    //Funktionen som är om forward
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    println!("Du står framför en stor låst dörr!");

    let mut _continue = String::new();
    io::stdin()
        .read_line(&mut _continue)
        .expect("Failed to read line");

    if !game.nyckel {
        println!("Du har ingen nyckel!");

        let mut _continue = String::new();
        io::stdin()
            .read_line(&mut _continue)
            .expect("Failed to read line");
    } else {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();

        println!("Du vann!");

        let mut _continue = String::new();
        io::stdin()
            .read_line(&mut _continue)
            .expect("Failed to read line");

        game.running = false;
    }
}
