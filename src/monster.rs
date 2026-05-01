pub struct Monster {
    pub monster_hp: u32,
    pub monster_dmg: u32,
    pub monster_alive: bool,
}
//Monster och dess värden, likt ovan...

impl Monster {
    pub fn new() -> Monster {
        Monster {
            monster_alive: true,
            monster_dmg: 4,
            monster_hp: 15,
        }
    }
}
