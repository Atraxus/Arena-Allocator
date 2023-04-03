// use arena::Arena;

struct Monster {
    level: u32,
}

fn main() {
    // Arena::new(|db_monsters| Arena::new(|dc_monsters| {                      // We create two arenas; db_monsters and dc_monsters
    //     let vegeta = db_monsters.alloc(Monster { level: 9001 });             // Vegeta is a shared ghost cell
    //     assert!(db_monsters.get(&vegeta).level > 9000);                      // With get, we get a shared reference to the inner value
    //     db_monsters.free(vegeta);                                            // Vegeta is freed, but the memory is not yet reused
        
    //     let mut joker = dc_monsters.alloc(Monster { level: 100 });           // Joker is an exclusive ghost cell
    //     dc_monsters.get_mut(&mut joker).level = 101;                         // With get_mut, we get an exclusive reference to the inner value
    //     // db_monsters.get(&joker).level = 102;                              // Doesn't compile! Joker is exclusive to dc_monsters
    // });
    
    let rancor = Monster { level: 100 };
    println!("{:?}", rancor.level);
}