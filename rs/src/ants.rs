use rand::seq::SliceRandom;
use std::collections::HashMap;

use crate::World;

#[derive(Debug, Eq, PartialEq)]
pub struct Ants {
    ant_map: HashMap<i32, String>,
}

impl Ants {
    pub fn new(num_ants: i32, world: &World) -> Ants {
        let mut ant_map = HashMap::new();

        let keys = world.get_colonies();

        for i in 0..num_ants {
            if let Some(rand_colony) = keys.choose(&mut rand::thread_rng()) {
                ant_map.insert(i, rand_colony.to_string());
            }
        }

        // more than 1 ant can start at the same colony they will get killed
        Ants { ant_map }
    }

    pub fn get_num_ants(&self) -> usize {
        self.ant_map.len()
    }

    pub fn move_ants(&mut self, world: &World) {
        let ant_positions = self.get_new_ant_positions(world);
        for (ant, new_colony) in ant_positions {
            let old_colony = self.ant_map.insert(ant, new_colony.to_string());
            // println!("Moved Ant: {} from: {:?} to {:?}", ant, old_colony.unwrap(), new_colony);
        }
    }

    pub fn fight_wars(&mut self, world: &mut World) {
        let wars = self.get_wars();
        for (colony, war) in wars.iter() {
            if war.len() > 1 {
                // there is a war
                println!(
                    "{} has been destroyed by {}",
                    colony,
                    self::Ants::format_ant_list(war)
                );
                for ant in war {
                    self.ant_map.remove(ant);
                }
                world.remove_colony(colony)
            }
        }
    }

    fn get_new_ant_positions(&self, world: &World) -> Vec<(i32, String)> {
        self.ant_map
            .iter()
            .map(|(key, value)| match world.get_random_direction(value) {
                Some(random_colony) => (*key, random_colony.to_owned()),
                _ => (*key, value.to_owned()),
            })
            .collect()
    }

    fn get_wars(&self) -> HashMap<String, Vec<i32>> {
        let mut check: HashMap<String, Vec<i32>> = HashMap::new();
        for (ant, colony) in self.ant_map.iter() {
            match check.get_mut(colony) {
                Some(war) => {
                    war.push(*ant);
                }
                _ => {
                    check.insert(colony.to_string(), vec![*ant]);
                }
            }
        }
        check
    }

    fn format_ant_list(ant_list: &Vec<i32>) -> String {
        match ant_list.len() {
            0 => String::new(),
            _ => {
                let ant_list: Vec<String> =
                    ant_list.iter().map(|ant| format!("Ant: {}", ant)).collect();
                ant_list.join(" and ")
            }
        }
    }
}
