use std::sync::Mutex;
use std::sync::Arc;
use std::{thread, time};

use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod model;
use model::island;
use island::Island;

mod game_engine;
use game_engine::user_input;
use game_engine::automated;

fn main() {
    print_menu(true);
    let mut _stdout = stdout().into_raw_mode();
    for key in io::stdin().keys() {
        print_menu(false);
        match key {
            Ok(key) => match key {
                
                // Start case
                Key::Char('s') => main_loop(),
                Key::Char('S') => main_loop(),

                // Quit case
                Key::Char('q') => break,
                Key::Char('Q') => break,

                // Wrong key case
                _ => (),
            },
            Err(_) =>(),
        }

    }
}

fn main_loop() {
    let island_size = 10;
    let mut current_nb_erratic = 1;
    let mut current_nb_hunters = 1;
    let mut current_nb_bottles = 8;
    let mut turn_counter = 0;

    loop {

        print_next_level(turn_counter);

        current_nb_erratic = nb_erratic(turn_counter);
        current_nb_hunters = nb_hunters(turn_counter, current_nb_hunters);
        current_nb_bottles = nb_bottles(turn_counter);
        println!("{} {} {} {}\r", turn_counter, current_nb_erratic,current_nb_hunters,current_nb_bottles);

        let island = Island::new(island_size, current_nb_hunters, current_nb_erratic, current_nb_bottles);
        let island_mut = Arc::new(Mutex::new(island));
        let island_mut_2 = island_mut.clone();
        let island_move_monkeys = island_mut.clone();

        let monkeys_moving_thread = thread::spawn( || {
            automated::move_monkeys(island_move_monkeys);
        });

        let mut island = island_mut.lock().unwrap();
        island.refresh_display();
        std::mem::drop(island);

        user_input::get_user_input(island_mut);

        monkeys_moving_thread.join().unwrap();
        
        let mut island = island_mut_2.lock().unwrap();
        if !island.get_pirate().get_alive() {
            break;
        }
        std::mem::drop(island);
        turn_counter +=1;
    }
    println!("> Press any key to continue...\r");
}

fn nb_erratic(turn_nb: usize) -> usize {
    turn_nb + 1
}

fn nb_hunters(turn_nb: usize, current_nb_hunters: usize) -> usize {
    let nb_hunt: usize;
    if turn_nb > 0 {
        if turn_nb % 2 == 0 {
            nb_hunt = current_nb_hunters+1;
        }
        else {
            nb_hunt = current_nb_hunters;
        }
    }
    else {
        nb_hunt = 1;
    }

    nb_hunt
}

fn nb_bottles(turn_nb: usize) -> usize {
    let nb_bot;
    if turn_nb < 4 {
        nb_bot = 8 - turn_nb;
    }
    else {
        nb_bot = 5;
    }
    nb_bot
}

fn print_menu(at_start: bool) {
    for _ in 0..20{
        println!("\r");
    }
    println!("\r
              ███╗   ███╗ ██████╗ ███╗   ██╗██╗  ██╗███████╗██╗   ██╗███████╗\r
              ████╗ ████║██╔═══██╗████╗  ██║██║ ██╔╝██╔════╝╚██╗ ██╔╝██╔════╝\r
              ██╔████╔██║██║   ██║██╔██╗ ██║█████╔╝ █████╗   ╚████╔╝ ███████╗\r
              ██║╚██╔╝██║██║   ██║██║╚██╗██║██╔═██╗ ██╔══╝    ╚██╔╝  ╚════██║\r
              ██║ ╚═╝ ██║╚██████╔╝██║ ╚████║██║  ██╗███████╗   ██║   ███████║\r
              ╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚═╝  ╚═╝╚══════╝   ╚═╝   ╚══════╝\r
    ");

    if at_start{
        // let sleeping_time = time::Duration::from_millis(2000);
        // thread::sleep(sleeping_time);
        
        println!("> Press any key to continue...\r");

    }
    else {
        println!("\r> Press 'S' to start the game\r");
        println!("> Press 'Q' to quit the game\r");
    }
    
}

fn print_next_level(turn_nb: usize) {
    for _ in 0..20{
        println!("\r");
    }
    println!("#######################################\r");
    println!("######### Next level: level {} #########\r",turn_nb);
    println!("#######################################\r");
    for _ in 0..5{
        println!("\r");
    }
    
    let sleeping_time = time::Duration::from_millis(5000);
    thread::sleep(sleeping_time);
}
