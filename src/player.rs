pub struct Player {
    pub vapen: bool,
    pub player_hp: u32,
    pub player_dmg: u32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            vapen: false,
            player_hp: 10,
            player_dmg: 3,
        }
    }
}
//Detta är vad spelaren har för värde. Sparar den i en structure man kan kalla på senare
