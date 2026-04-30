use std::io::{self, Write};
//importerar så att jag kan radera text samt läsa av inkommande text från användaren 
mod game;
mod rooms;
use crate::rooms::forward;
use crate::game::Game;



fn main() {
    let mut game = Game {
        nyckel: false,
        running: true,
    };

    print!("\x1B[2J\x1B[1;1H");         //Detta stycket är det som gör att text rensas. 
    io::stdout().flush().unwrap();      // <==



    println!("You awake in a dark dungeon!");
    while game.running == true {                    //Här börjar spelmotorn vilket är en enkel while sats
        {
            println!("You can go left, forward, right");
            println!("What do you choose");                 //Första valen som spelaren får 
            println!("quit to exit the game");
            println!("");
            println!("");
            println!("(left)   (forward)   (right)");

            let mut choice = String::new();                 //Skapar input variablen    
            io::stdin()                                     //Använder importen från toppen
                .read_line(&mut choice)                     //Läser av spelarens val och lägger in det i en 
                                                            //muterande string
                .expect("Failed to read line");             //Ifall det inte blir något skrivs detta

            match choice.trim() {                           //Början på match satsen
                "forward" => forward(&mut game),

                "right" => right(&mut game),

                "left" => left(&mut game),

                "quit" => quit(&mut game),

                _ => println!("Var snäll och välj ett bra alternativ"),     //Ifall man inte har valt något skrivs detta
            }
        }
    }
}



fn right(game: &mut Game) {         //Funkionen som är om right


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

fn left(game: &mut Game) {      //Funktionen som är om left
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    println!("You found a weapon!");
}

fn quit(game: &mut Game) {      //Ifall spelaren skriver quit så kommer denna kallas och stoppar while loopen
    game.running = false;
}

/* 

fn fight(game: &mut Game, Player.player_hp()){

    

    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    println!("You started the fight!");
    while Player.player_hp > 0 && Monster.monster_hp > 0 {

    //   println!("You start by hitting the monster for {}", Player.player_dmg); 


    }

    //println!("You have {} hp" , Player.player_hp = 20, "and {} damage", Player.player_dmg = 3);
}

*/
