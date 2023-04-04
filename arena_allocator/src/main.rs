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
    Arena::new(|mut db_monsters| Arena::new(|mut dc_monsters| {
        let vegeta = db_monsters.alloc(Monster { level: 9001 });
        assert!(db_monsters.get(vegeta).level > 9000);
        // db_monsters.free(vegeta);
        
        let joker = dc_monsters.alloc(Monster { level: 100 });
        dc_monsters.get_mut(joker).level = 101;
        // db_monsters.get(&joker).level = 102; // doesn't compile!
    }));
}