mod ants;
mod world;

use std::env;

use ants::Ants;
use world::World;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 2 {
        println!("Usage: <NUM_ANTS> <WORLD_FILE_PATH>");
        return;
    }

    let num_starting_ants: i32 = args[0].parse::<i32>().unwrap();
    let world_path: &str = args[1].as_str();

    let mut world = World::new(world_path).unwrap();
    // println!("World: {:?}", world);

    let mut ants = Ants::new(num_starting_ants, &world);
    // println!("Ants: {:?}", ants);

    let mut i = 0;
    while i < 10000 && ants.get_num_ants() > 0 {
        // move ants
        ants.move_ants(&world);

        // fight wars
        ants.fight_wars(&mut world);

        i += 1;
    }

    let num_ants = ants.get_num_ants();
    if num_ants > 0 {
        println!("Finished sim with {} remaining", num_ants);
    }

    // re-print world
    // world.print_remaining();
}
