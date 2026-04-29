use std::io;

fn main() {
    println!("You awake in a dark dungeon!");
    println!("You can go left, forward, right");
    println!("What do you choose");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    
    
    match choice.trim(){
        "forward" => forward(),

        "right" => right(),
        
        "left" => left(),
        
        _ => println!("Var snäll och välj ett bra alternativ"),
    }    
}


fn forward(){
    println!("Du har kallat på forward funktionen!");
}

fn right(){
    println!("Du har kallat på right funktionen!");
}

fn left(){
    println!("Du har kallat på left funktionen!");
}