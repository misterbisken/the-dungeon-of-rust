use std::io::{self, Write};
//importerar så att jag kan radera text samt läsa av inkommande text från användaren 


struct Player {
    vapen: bool,
    player_hp: u32,
    player_dmg: u32,
}
//Detta är vad spelaren har för värde. Sparar den i en structure man kan kalla på senare


struct Monster {
    monster_hp: u32,
    monster_dmg: u32,
    monster_alive: bool,
}
//Monster och dess värden, likt ovan...

struct Game {
    nyckel: bool,
    running: bool,
}
//Detta är alla variabler för spelet. Tillexempel två bools som checkar ifall man har nyckel eller ej
//Samt kan göra att spelet slutar ifall running blir false


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

fn forward(game: &mut Game) {       //Funktionen som är om forward
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    println!("Du står framför en stor låst dörr!");
    if !game.nyckel {
        println!("Du har ingen nyckel!")
    } else {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();

        println!("Du vann!");
        game.running = false;
    }
}

fn right(game: &mut Game) {         //Funkionen som är om right
    //println!("Du möter ett stort monster!");
    //println!("Vill du slåss eller gå tillbaka?");

    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    println!("Du hittade en nyckel!");
    game.nyckel = true;
}

fn left(game: &mut Game) {      //Funktionen som är om left
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    println!("Du hittar ett svärd");
}

fn quit(game: &mut Game) {      //Ifall spelaren skriver quit så kommer denna kallas och stoppar while loopen
    game.running = false;
}
