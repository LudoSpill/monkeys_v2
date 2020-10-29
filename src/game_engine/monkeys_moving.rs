use std::sync::Mutex;
use std::sync::Arc;
use std::{thread, time};

use crate::model::island::Island;
use crate::model::monkeys::MonkeyMove;

pub fn move_monkeys(island_mut: Arc<Mutex<Island>>) {

    loop {
        let sleeping_time = time::Duration::from_secs(1);
        thread::sleep(sleeping_time);
        
        let mut island = island_mut.lock().unwrap();

        // let hunters = island.get_hunters();
        let erratics = island.get_erratics();

        // for hunter in hunters {
        //     hunter.monkey_move();
        // }

        for erratic in erratics {
            erratic.monkey_move();
        }

        island.handle_pirate_monkey_collision();
        island.refresh_display();     

        // Check if game is over
        if island.is_treasure_discovered() || !(island.get_pirate().get_alive()) {
            break;
        }
    }
    
}
