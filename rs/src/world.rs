use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use rand::seq::SliceRandom;

// Wanted to use Rc here and re-use the same heap value for the key and Colony value but couldnt
// figure it out
#[derive(Default, Debug)]
pub struct ColonyState {
    // [north, south, east, west]
    pub directions: [Option<String>; 4],
}

#[derive(Debug)]
pub struct World {
    state: HashMap<String, ColonyState>,
}

impl World {
    pub fn new(file_path: &str) -> io::Result<World> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut state: HashMap<String, ColonyState> = HashMap::new();

        for line in reader.lines() {
            match line {
                Ok(content) => {
                    let mut iter = content.split_whitespace();
                    // not very good error checking
                    let colony_name = iter.next().unwrap();
                    let mut colony_state = ColonyState::default();

                    for neighbours in iter {
                        let mut direction_iter = neighbours.split_terminator('=');
                        let direction = direction_iter.next();
                        let colony = direction_iter.next();

                        match (direction, colony) {
                            (None, None) | (None, _) | (_, None) => {
                                // handle error
                            }
                            (Some("north"), Some(colony)) => {
                                colony_state.directions[0] = Some(String::from(colony));
                            }
                            (Some("south"), Some(colony)) => {
                                colony_state.directions[1] = Some(String::from(colony));
                            }
                            (Some("east"), Some(colony)) => {
                                colony_state.directions[2] = Some(String::from(colony));
                            }
                            (Some("west"), Some(colony)) => {
                                colony_state.directions[3] = Some(String::from(colony));
                            }
                            (_, _) => {
                                // handle unknown
                            }
                        }
                    }
                    state.insert(String::from(colony_name), colony_state);
                }
                Err(err) => {
                    // handle error
                    eprintln!("Error reading line: {}", err);
                }
            }
        }
        Ok(World { state })
    }

    pub fn remove_colony(&mut self, colony: &String) {
        self.state.remove(colony);
    }

    pub fn print_remaining(&self) {
        todo!("Print remaining state of the world to a new file")
    }

    pub fn get_colonies(&self) -> Vec<String> {
        self.state.keys().cloned().collect()
    }

    pub fn get_random_direction(&self, colony: &str) -> Option<&String> {
        if let Some(colony_state) = self.state.get(colony) {
            let valid_directions: Vec<&String> = colony_state
                .directions
                .iter()
                .filter(|colony| {
                    colony.is_some() && self.state.contains_key(colony.as_ref().unwrap())
                })
                .map(|colony| colony.as_ref().unwrap())
                .collect();

            return valid_directions.choose(&mut rand::thread_rng()).cloned();
        }

        None
    }
}
