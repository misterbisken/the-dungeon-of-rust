use super::fight::fight;
use crate::{Monster, Player, game::Game};
use std::io::{self, Write};

pub fn right(game: &mut Game, player: &mut Player, monster: &mut Monster) {
    //Funkionen som är om right

    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    if monster.monster_alive == true {
        println!("You have stumbled on a monster");
        println!("Will you fight or run?");
        println!("");
        println!("");
        println!("(fight)    (run)");
        println!("");
        println!("");
        println!("Your stats:");
        println!("Player hp {}", player.player_hp);
        println!("Player dmg {}", player.player_dmg);
        println!("Player weapon {}", player.vapen);

        let mut acceptbattle = String::new();
        io::stdin()
            .read_line(&mut acceptbattle)
            .expect("Failed to read line");

        match acceptbattle.trim() {
            "fight" => fight(game, player, monster),

            "run" => print!(""),

            _ => println!("Please enter a valid answer"),
        }
    } else {
        println!("Du har redan dödat monstret och fått nyckeln!");
    }
}
