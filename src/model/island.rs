use rand::Rng;

use super::bottle::Bottle;
use super::pirate::Pirate;
use super::monkeys;
use monkeys::HunterMonkey;
use monkeys::ErraticMonkey;
use monkeys::MonkeyMove;
use super::treasure::Treasure;

#[derive(PartialEq,Clone,Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE
}

#[derive(Clone)]
pub struct Island {
    size : usize,
    grid: Vec<Vec<char>>,
    pirate: Pirate,
    nb_hunters: usize,
    hunters: Vec<HunterMonkey>,
    nb_erratics: usize,
    erratics: Vec<ErraticMonkey>,
    nb_max_bottles: usize,
    bottles: Vec<Bottle>,
    treasure: Treasure
}

impl Island {

    pub fn new(new_size: usize, new_nb_hunters: usize, new_nb_erratics: usize, new_nb_max_bottles: usize) -> Self {
        
        // Initializing the game
        let (
            new_x_treasure, new_y_treasure,
            new_x_pirate, new_y_pirate, 
            bottles_coords, 
            erratics_coords, 
            hunters_coords
        ) = Island::init_island(new_size, new_nb_hunters, new_nb_erratics, new_nb_max_bottles);

        let mut new_bottles: Vec<Bottle> = Vec::new();
        for i in 0..new_nb_max_bottles {
            new_bottles.push(Bottle::new(new_size, bottles_coords[i][0], bottles_coords[i][1]));
        }

        let mut new_erratics: Vec<ErraticMonkey> = Vec::new();
        for i in 0..new_nb_hunters {
            new_erratics.push(ErraticMonkey::new(new_size, erratics_coords[i][0] as i8, erratics_coords[i][1] as i8));
        }

        let mut new_hunters: Vec<HunterMonkey> = Vec::new();
        for i in 0..new_nb_hunters {
            new_hunters.push(HunterMonkey::new(new_size, hunters_coords[i][0] as i8, hunters_coords[i][1] as i8));
        }

        Self {
            size : new_size,
            grid : Island::create_grid(new_size),
            pirate : Pirate::new(new_size, new_x_pirate, new_y_pirate),
            nb_hunters: new_nb_hunters,
            hunters : new_hunters,
            nb_erratics: new_nb_erratics,
            erratics: new_erratics,
            nb_max_bottles: new_nb_max_bottles,
            bottles : new_bottles,
            treasure : Treasure::new(new_x_treasure, new_y_treasure)
        }
    }

    fn init_island(new_size: usize, new_nb_hunters: usize, new_nb_erratics: usize, new_nb_max_bottles: usize) 
        -> (usize, usize, usize, usize, Vec<[usize; 2]>, Vec<[usize; 2]>, Vec<[usize; 2]>)
    {
        let (x_treasure, y_treasure) = Island::init_treasure(new_size);
        let (x_pirate, y_pirate) = Island::init_pirate(new_size,x_treasure,y_treasure);
        let bottles: Vec<[usize; 2]> = Island::init_bottles(new_size, new_nb_max_bottles);
        let erratics = Island::init_erratics(new_size, new_nb_erratics, x_pirate, y_pirate);
        let hunters = Island::init_hunters(new_size, new_nb_hunters, x_pirate, y_pirate, erratics.clone());
        (x_treasure, y_treasure, x_pirate, y_pirate, bottles, erratics, hunters)
    }

    fn init_treasure(new_size: usize) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0,new_size-1);
        let y = rng.gen_range(0,new_size-1);
        (x, y)
    }

    fn init_pirate(new_size: usize, x_treasure: usize, y_treasure: usize) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        let mut x = rng.gen_range(0,new_size-1);
        let mut y = rng.gen_range(0,new_size-1);

        while x == x_treasure && y == y_treasure {
            x = rng.gen_range(0,new_size-1);
            y = rng.gen_range(0,new_size-1);
        }
        (x, y)
    }

    fn init_bottles(new_size: usize, nb_max_bottles: usize) -> Vec<[usize; 2]> {
        let mut bottles: Vec<[usize; 2]> = Vec::new();
        let mut rng = rand::thread_rng();
        
        for _ in 0..nb_max_bottles {
            bottles.push([rng.gen_range(0,new_size-1),rng.gen_range(0,new_size-1)]);
            for i in 0..bottles.len()-1 {
                while bottles[bottles.len()-1] == bottles[i] {
                    bottles.pop();
                    bottles.push([rng.gen_range(0,new_size-1),rng.gen_range(0,new_size-1)])
                }
            }
        }
        bottles
    }

    fn init_erratics(new_size: usize, nb_erratics: usize, x_pirate: usize, y_pirate: usize) -> Vec<[usize; 2]> {
        let mut erratics: Vec<[usize; 2]> = Vec::new();
        let mut rng = rand::thread_rng();
        
        for _ in 0..nb_erratics {
            erratics.push([rng.gen_range(0,new_size-1) ,rng.gen_range(0,new_size-1)]);
            for i in 0..erratics.len()-1 {
                while (erratics[erratics.len()-1] == erratics[i]) || 
                       Island::distance_between(x_pirate, y_pirate, erratics[erratics.len()-1][0], erratics[erratics.len()-1][1]) < 3.0 
                {
                    erratics.pop();
                    erratics.push([rng.gen_range(0,new_size-1) ,rng.gen_range(0,new_size-1)]);
                }
            }
        }

        erratics
    }

    fn init_hunters(new_size: usize, nb_hunters: usize, x_pirate: usize, y_pirate: usize, erratics: Vec<[usize; 2]>) -> Vec<[usize; 2]> {
        let mut hunters: Vec<[usize; 2]> = Vec::new();
        let mut rng = rand::thread_rng();
        
        for _ in 0..nb_hunters {
            hunters.push([rng.gen_range(0,new_size-1) ,rng.gen_range(0,new_size-1)]);
            for i in 0..hunters.len()-1 {
                for erratic in erratics.iter() {
                    while (hunters[hunters.len()-1] == hunters[i]) || 
                           Island::distance_between(x_pirate, y_pirate, hunters[hunters.len()-1][0], hunters[hunters.len()-1][1]) < 3.0 ||
                           hunters[hunters.len()-1] == erratic.clone()
                    {
                        hunters.pop();
                        hunters.push([rng.gen_range(0,new_size-1) ,rng.gen_range(0,new_size-1)]);
                    }
                }
                
            }
        }
        hunters
    }

    fn distance_between(x1: usize, y1: usize, x2: usize, y2: usize) -> f64 {
        let diff_x: f64 = (x1 as f64 - x2 as f64).powf(2.0);
        let diff_y: f64 = (y1 as f64 - y2 as f64).powf(2.0);
        (diff_x + diff_y).sqrt()
    }

    /**
     * Function that creates the grid for the island
     */
    fn create_grid(size: usize) -> Vec<Vec<char>> {
        let grid: Vec<Vec<char>> = (0..size).map(|_| (0..size).map(|_| '-').collect()).collect(); // map : applique un fonction à l'itérateur | collect : prend un itérateur et créé un vecteur

        return grid;
    }

    /**
     * Function that sets the tiles according to what is in them
     */
    fn set_tiles(&mut self) {
        // '-' everywhere
        self.grid = (0..self.size).map(|_| (0..self.size).map(|_| ' ').collect()).collect();

        // 'B' where the bottles are
        for bottle in self.bottles.iter() {
            self.grid[bottle.get_x()][bottle.get_y()] = 'B'; 
        }

        // 'P' where the player is
        self.grid[self.pirate.get_x()][self.pirate.get_y()] = 'P'; 

        // 'H' where hunters are
        for hunter in self.hunters.iter() {
            self.grid[hunter.get_x() as usize][hunter.get_y() as usize] = 'H'; 
        }

        // 'E' where erratics are
        for erratic in self.erratics.iter() {
            self.grid[erratic.get_x() as usize][erratic.get_y() as usize] = 'E'; 
        }
        
        // 'T' where the treasure is [DEBUG ONLY :)]
        // self.grid[self.treasure.get_x()][self.treasure.get_y()] = 'T'; 

    }

    pub fn refresh_display(&mut self) {
        Island::set_tiles(self);

        for _ in 0..20{
            println!("\r");
        }

        for i in 0..self.grid.len() {
            println!("{:?}\r",self.grid[i])
        }

        println!("Pirate energy : {}/{}\r", self.pirate.get_current_energy(), self.pirate.get_max_energy())
    }

    pub fn handle_collisions(&mut self) {
        self.handle_pirate_monkey_collision();
        self.handle_pirate_bottle_collision();
    }

    pub fn handle_pirate_monkey_collision(&mut self){
        let hunters = self.get_hunters().clone();
        let erratics = self.get_erratics().clone();
        let mut is_dead = false;

        for hunter in hunters {
            if (hunter.get_x() == self.get_pirate().get_x() as i8) && (hunter.get_y() == self.get_pirate().get_y() as i8) {
                self.get_pirate().set_alive(false);
                is_dead = true;
                break;
            }
        }

        if !is_dead {
            for erratic in erratics {
                if (erratic.get_x() == self.get_pirate().get_x() as i8) && (erratic.get_y() == self.get_pirate().get_y() as i8) {
                    self.get_pirate().set_alive(false);
                    break;
                }
            }
        }
    }

    fn handle_pirate_bottle_collision(&mut self) {
        let mut empty_bottles: Vec<Bottle> = Vec::new();
        
        // Loop to drink to bottle
        for bottle in self.bottles.iter() {
            if (bottle.get_x() == self.pirate.get_x()) && (bottle.get_y() == self.pirate.get_y()) {
                self.pirate.drink_bottle(bottle.get_energetic_value());
                empty_bottles.push(bottle.clone());
            }
        }

        let mut i=0;
        for empty_bottle in empty_bottles.iter() {
            for (i, bottle) in self.bottles.clone().iter_mut().enumerate() {
                if empty_bottle.clone() == bottle.clone() {
                    self.bottles.remove(i);
                }
            }
        }
    }

    pub fn move_monkeys(&mut self) {
        let hunters_copy = self.hunters.clone();
        for hunter in self.hunters.iter_mut() {
            hunter.monkey_move(self.pirate, hunters_copy.clone(), self.erratics.clone());
        }

        let erratics_copy = self.erratics.clone();
        for erratic in self.erratics.iter_mut() {
            erratic.monkey_move(self.pirate, self.hunters.clone(), erratics_copy.clone());
        }
    }

    pub fn get_pirate(&mut self) -> &mut Pirate {
        &mut self.pirate
    }

    pub fn is_treasure_discovered(&mut self) -> bool {
        self.get_pirate().get_x() == self.get_treasure().get_x() && self.get_pirate().get_y() == self.get_treasure().get_y()
    }
    
    pub fn get_hunters(&mut self) -> &mut Vec<HunterMonkey> {
        &mut self.hunters
    }

    pub fn get_erratics(&mut self) -> &mut Vec<ErraticMonkey> {
        &mut self.erratics
    }

    pub fn get_treasure(&self) -> Treasure {
        self.treasure.clone()
    }

}
