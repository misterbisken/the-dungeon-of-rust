use crate::game::Game;
use crate::monster::Monster;
use crate::player::Player;



pub fn fight(Player { vapen, player_hp, player_dmg }: Player){

    println!("Du har {}hp", {player_hp});
    println!("Du har {}dmg", {player_dmg});

    if vapen == true{

        println!("Du har ett vapen");

    }
    else {
        println!("Du har inget vapen");
    }
    

}


