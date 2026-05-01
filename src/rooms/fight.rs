use crate::game::Game;
use crate::monster::Monster;
use crate::player::Player;
use std::io::{self, Write};

pub fn fight(game: &mut Game, player: &mut Player, monster: &mut Monster) {
    if player.vapen {
        println!("Du har ett vapen");
    } else {
        println!("Du har inget vapen");
    }

    while player.player_hp > 0 && monster.monster_hp > 0 {
        if monster.monster_dmg >= player.player_hp {
            println!("Du dog!");
            game.running = false;

            break;
        } else if player.player_dmg >= monster.monster_hp {
            println!("Du besegrade monstret!");
            monster.monster_alive = false;
            game.nyckel = true;

            break;
        } else {
            println!("Du attakerar monstret!");

            monster.monster_hp = monster.monster_hp - player.player_dmg;

            println!("Monstret har nu {}hp", monster.monster_hp);

            println!("Monstret attakerar dig!");

            player.player_hp = player.player_hp - monster.monster_dmg;

            println!("Du har nu {}", player.player_hp);

            let mut _continue = String::new();
            io::stdin()
                .read_line(&mut _continue)
                .expect("Failed to read line");
        }
    }
}
