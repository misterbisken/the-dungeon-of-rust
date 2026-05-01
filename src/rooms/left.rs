use crate::{Player, game::Game, player};
use std::io::{self, Write};

pub fn left(game: &mut Game, player: &mut Player) {
    //Funktionen som är om left
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    if player.vapen == true {

        println!("Du har redan hittat vapnet!")

    } else {


        println!("You found a weapon!");
        player.player_dmg = 7;
        player.vapen = true;

    }
    



}
