use std::sync::Mutex;
use std::sync::Arc;
use std::thread;

mod model;
use model::island;
use island::Island;
use island::DEFAULT_ISLAND_SIZE;
use model::monkeys::Erratic_Monkey;

mod game_engine;
use game_engine::user_input;
use game_engine::monkeys_moving;

fn main() {
    let mut island = Island::new(10, 0);
    island.get_erratics().push(Erratic_Monkey::new(10, 3, 7));

    let island_mut = Arc::new(Mutex::new(island));
    let island2 = island_mut.clone();

    let monkeys_moving_thread = thread::spawn( || {
        monkeys_moving::move_monkeys(island2);
    });

    let mut island = island_mut.lock().unwrap();
    island.refresh_display();
    std::mem::drop(island);

    user_input::get_user_input(island_mut);

    monkeys_moving_thread.join().unwrap();
}
