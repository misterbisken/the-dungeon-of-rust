use std::io::{self, Write};


struct Player {
    vapen: bool,
    player_hp: u32,
    player_dmg: u32,
}


struct Monster {
    monster_hp: u32,
    monster_dmg: u32,
    monster_alive: bool,
}

struct Game {
    nyckel: bool,
    running: bool,
}

fn main() {
    let mut game = Game { nyckel: false, running: true };


    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    println!("You awake in a dark dungeon!");
    while game.running == true {
        
     {
        println!("You can go left, forward, right");
        println!("What do you choose");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "forward" => forward(&mut game),

            "right" => right(&mut game),

            "left" => left(),

            _ => println!("Var snäll och välj ett bra alternativ"),
        }
    }    
    }
}

fn forward(game: &mut Game) {

    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
    

    println!("Du står framför en stor låst dörr!");
    if game.nyckel == false {
        println!("Du har ingen nyckel!")
    } else {

        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();

        println!("Du vann!");
        game.running = false;
    }
}

fn right(game: &mut Game) {
    //println!("Du möter ett stort monster!");
    //println!("Vill du slåss eller gå tillbaka?");

    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    println!("Du hittade en nyckel!");
    game.nyckel = true;

}

fn left() {

    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
    

    println!("Du hittar ett svärd");
}
