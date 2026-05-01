use std::io::{self, Write};
//importerar så att jag kan radera text samt läsa av inkommande text från användaren
mod game;
mod monster;
mod player;
mod rooms;

use crate::game::Game;
use crate::monster::Monster;
use crate::player::Player;
use crate::rooms::forward;
use crate::rooms::left;
use crate::rooms::right;

fn main() {
    let mut game = Game {
        nyckel: false,
        running: true,
    };

    let mut player = Player::new();
    let mut monster = Monster::new();

    print!("\x1B[2J\x1B[1;1H"); //Detta stycket är det som gör att text rensas. 
    io::stdout().flush().unwrap(); // <==

    println!("You awake in a dark dungeon!");
    println!("");

    let mut _continue = String::new();
    io::stdin()
        .read_line(&mut _continue)
        .expect("Failed to read line");

    while game.running == true {
        //Här börjar spelmotorn vilket är en enkel while sats

        print!("\x1B[2J\x1B[1;1H"); //Detta stycket är det som gör att text rensas. 
        io::stdout().flush().unwrap(); // <==

        {
            println!("You can go left, forward, right");
            println!("");
            println!("What do you choose"); //Första valen som spelaren får 
            println!("");
            println!("quit to exit the game");
            println!("");
            println!("");
            println!("(left)   (forward)   (right)");
            println!("");

            let mut choice = String::new(); //Skapar input variablen    
            io::stdin() //Använder importen från toppen
                .read_line(&mut choice) //Läser av spelarens val och lägger in det i en
                //muterande string
                .expect("Failed to read line"); //Ifall det inte blir något skrivs detta

            match choice.trim() {
                //Början på match satsen
                "forward" => forward(&mut game),

                "right" => right(&mut game, &mut player, &mut monster),

                "left" => left(&mut game, &mut player),

                "quit" => quit(&mut game),

                _ => println!("Var snäll och välj ett bra alternativ"), //Ifall man inte har valt något skrivs detta
            }
        }
    }
}

fn quit(game: &mut Game) {
    //Ifall spelaren skriver quit så kommer denna kallas och stoppar while loopen
    game.running = false;
}
