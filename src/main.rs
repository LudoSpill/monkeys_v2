use std::sync::Mutex;
use std::sync::Arc;
use std::thread;

mod model;
use model::island;
use island::Island;

mod game_engine;
use game_engine::user_input;
use game_engine::monkeys_moving;

fn main() {
    let island = Island::new(10, 2, 2, 0, 2);

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
