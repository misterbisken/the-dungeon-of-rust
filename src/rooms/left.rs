use crate::{Player, game::Game, player};
use std::io::{self, Write};

pub fn left(game: &mut Game, player: &mut Player) {
    //Funktionen som är om left
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    if player.vapen == true {
        println!("Du har redan hittat vapnet!");

        let mut _continue = String::new();
        io::stdin()
            .read_line(&mut _continue)
            .expect("Failed to read line");
    } else {
        println!("You found a weapon!");
        println!("");
        println!("Press Enter to continue");

        player.player_dmg = 8;
        player.vapen = true;

        let mut _continue = String::new();
        io::stdin()
            .read_line(&mut _continue)
            .expect("Failed to read line");
    }
}
