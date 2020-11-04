use std::sync::Mutex;
use std::sync::Arc;
use std::{thread, time};

use crate::model::island::Island;

pub fn move_monkeys(island_mut: Arc<Mutex<Island>>) {

    loop {
        let sleeping_time = time::Duration::from_secs(3);
        thread::sleep(sleeping_time);
        
        let mut island = island_mut.lock().unwrap();
        
        island.move_monkeys();

        island.handle_pirate_monkey_collision();
        island.refresh_display();     

        // Check if game is over
        if island.is_treasure_discovered() || !(island.get_pirate().get_alive()) {
            break;
        }
    }
    
}
