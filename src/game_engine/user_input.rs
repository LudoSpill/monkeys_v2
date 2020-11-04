use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::sync::Mutex;
use std::sync::Arc;

use crate::model::island;
use island::Island;
use island::Direction;

pub fn get_user_input(island_mut: Arc<Mutex<Island>>) {

    let mut _stdout = stdout().into_raw_mode();
    for key in io::stdin().keys() {
        match key {
            Ok(key) => match key {
                // Move cases
                Key::Up => {
                    let mut island = island_mut.lock().expect("Erreur lock island");
                    let pirate = island.get_pirate();
                    pirate.move_pirate(Direction::UP);

                },
                Key::Down => {
                    let mut island = island_mut.lock().expect("Erreur lock island");
                    let pirate = island.get_pirate();
                    pirate.move_pirate(Direction::DOWN);

                },
                Key::Right => {
                    let mut island = island_mut.lock().expect("Erreur lock island");
                    let pirate = island.get_pirate();
                    pirate.move_pirate(Direction::RIGHT);

                },
                Key::Left => {
                    let mut island = island_mut.lock().expect("Erreur lock island");
                    let pirate = island.get_pirate();
                    pirate.move_pirate(Direction::LEFT);
                    
                },

                // Quit case
                Key::Ctrl('q') => {
                    let mut island = island_mut.lock().expect("Erreur lock island");
                    let pirate = island.get_pirate();
                    pirate.set_alive(false);
                    break},

                // Wrong key case
                _ => println!("Wrong key pressed {:?}\r", key),
            },
            Err(_) =>(), //TODO
        }

        let mut island = island_mut.lock().expect("island2 lock error");

        // Check if player is alive
        if island.get_pirate().get_alive() == false {
            println!("Perdu !\r");
            break;
        }

        island.refresh_display();
        island.handle_collisions();

        // Check if treasure was found
        if island.is_treasure_discovered() && island.get_pirate().get_alive(){
            println!("Gagné !\r");
            break;
        }

    }
}