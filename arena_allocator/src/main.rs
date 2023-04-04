// Hide unused
#![allow(dead_code)]
mod arena;
mod branded_vec;

use arena::Arena;
// use branded_vec::BrandedVec;

struct Monster {
    level: u32,
}

fn main() {
    Arena::new(|db_monsters| Arena::new(|dc_monsters| {
        let vegeta = db_monsters.alloc(Monster { level: 9001 });
        assert!(db_monsters.get(&vegeta).level > 9000);
        // db_monsters.free(vegeta);
        
        // In the following we want to test wheter we can get 
        // mutable references to multiple monsters in the same arena.
        let mut joker = dc_monsters.alloc(Monster { level: 100 });
        let mut riddler = dc_monsters.alloc(Monster { level: 101 });
        let mut penguin = dc_monsters.alloc(Monster { level: 102 });
        let joker_inst = dc_monsters.get_mut(&mut joker);
        let riddler_inst = dc_monsters.get_mut(&mut riddler);
        let penguin_inst = dc_monsters.get_mut(&mut penguin);

        joker_inst.level = 1000;
        riddler_inst.level = 1001;
        penguin_inst.level = 1002;

        println!("Joker: {}", joker_inst.level);
        println!("Riddler: {}", riddler_inst.level);
        println!("Penguin: {}", penguin_inst.level);
    }));
}