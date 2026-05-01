use crate::game::Game;
use crate::monster::Monster;
use crate::player::Player;



pub fn fight(game: &mut Game, player: &mut Player, monster: &mut Monster) {


    println!("Du har {}hp", {player.player_hp});
    println!("Du har {}dmg", {player.player_dmg});

    if player.vapen == true{

        println!("Du har ett vapen");

    }
    else {
        println!("Du har inget vapen");
    }

    while player.player_hp > 0 {

        monster.monster_hp = monster.monster_hp - player.player_dmg;

        println!("Monstret har nu {}hp", monster.monster_hp);

        player.player_hp = player.player_hp - monster.monster_dmg;
    
        println!("Du har nu {}", player.player_hp);
    }
    

}


