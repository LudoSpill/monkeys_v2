use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::sync::Mutex;
use std::sync::Arc;

use crate::model::island;
use island::Island;
use island::Direction;

fn die(e: std::io::Error) {
    panic!(e);
}

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
                    island.refresh_display();
                    island.handle_collisions();
                },
                Key::Down => {
                    let mut island = island_mut.lock().expect("Erreur lock island");
                    let pirate = island.get_pirate();
                    pirate.move_pirate(Direction::DOWN);
                    island.refresh_display();
                    island.handle_collisions();
                },
                Key::Right => {
                    let mut island = island_mut.lock().expect("Erreur lock island");
                    let pirate = island.get_pirate();
                    pirate.move_pirate(Direction::RIGHT);
                    island.refresh_display();
                    island.handle_collisions();
                },
                Key::Left => {
                    let mut island = island_mut.lock().expect("Erreur lock island");
                    let pirate = island.get_pirate();
                    pirate.move_pirate(Direction::LEFT);
                    island.refresh_display();
                    island.handle_collisions();
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
            Err(err) => die(err),
        }

        let mut island = island_mut.lock().expect("island2 lock error");

        // Check if player is alive
        if island.get_pirate().get_alive() == false {
            println!("Perdu !\r");
            break;
        }

        // Check if treasure was found
        if island.is_treasure_discovered() {
            println!("Gagné !\r");
            break;
        }

    }
}