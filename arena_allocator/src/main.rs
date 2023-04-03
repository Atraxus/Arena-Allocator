// use arena::Arena;

struct Monster {
    level: u32,
}

fn main() {
    // Arena::new(|db_monsters| Arena::new(|dc_monsters| {
    //     let vegeta = db_monsters.alloc(Monster { level: 9001 });
    //     assert!(db_monsters.get(&vegeta).level > 9000);
    //     db_monsters.free(vegeta);
        
    //     let mut joker = dc_monsters.alloc(Monster { level: 100 });
    //     dc_monsters.get_mut(&mut joker).level = 101;
    //     // db_monsters.get(&joker).level = 102; // doesn't compile!
    // });
    
    let rancor = Monster { level: 100 };
    println!("{:?}", rancor.level);
}