use super::bottle::Bottle;
use super::pirate::Pirate;
use super::monkeys;
use monkeys::Hunter_Monkey;
use monkeys::Erratic_Monkey;
use super::treasure::Treasure;

pub const DEFAULT_ISLAND_SIZE: usize = 10;

#[derive(PartialEq,Clone)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Clone)]
pub struct Island {
    size : usize,
    grid: Vec<Vec<char>>,
    difficulty_level: usize,
    pub pirate: Pirate,
    hunters: Vec<Hunter_Monkey>,
    erratics: Vec<Erratic_Monkey>,
    bottles: Vec<Bottle>,
    treasure: Treasure
}

impl Island {

    pub fn new(new_size: usize, new_difficulty_level: usize) -> Self {
        Self {
            size : new_size,
            grid : Island::create_grid(new_size),
            difficulty_level : new_difficulty_level,
            pirate : Pirate::new_default(new_size),
            hunters : vec![Hunter_Monkey::new_default(new_size)],
            erratics: vec![Erratic_Monkey::new_default(new_size)],
            bottles : vec![Bottle::new_default(new_size)],
            treasure : Treasure::new()
        }
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
        self.grid = (0..self.size).map(|_| (0..self.size).map(|_| '-').collect()).collect();
        
        // 'P' where the player is
        self.grid[self.pirate.get_x()][self.pirate.get_y()] = 'P'; 
        
        // 'H' where hunters are
        for hunter in self.hunters.iter() {
            self.grid[hunter.get_x()][hunter.get_y()] = 'H'; 
        }

        // 'E' where erratics are
        for erratic in self.erratics.iter() {
            self.grid[erratic.get_x()][erratic.get_y()] = 'E'; 
        }

        // 'B' where the bottles are
        for bottle in self.bottles.iter() {
            self.grid[bottle.get_x()][bottle.get_y()] = 'B'; 
        }

    }

    pub fn refresh_display(&mut self) {
        Island::set_tiles(self);

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
            if (hunter.get_x() == self.get_pirate().get_x()) && (hunter.get_y() == self.get_pirate().get_y()) {
                self.get_pirate().set_alive(false);
                is_dead = true;
                break;
            }
        }

        if !is_dead {
            for erratic in erratics {
                if (erratic.get_x() == self.get_pirate().get_x()) && (erratic.get_y() == self.get_pirate().get_y()) {
                    self.get_pirate().set_alive(false);
                    break;
                }
            }
        }
        
    }

    fn handle_pirate_bottle_collision(&mut self) {
        let mut pirate = self.get_pirate();
        let mut bottles = self.get_size();

        // TODO collisions boutrilles / pirate
        // for i in 0..bottles.len() {
        //     if (bottles[i].get_x() == pirate.get_x()) && (bottles[i].get_y() == pirate.get_y()) {
        //         pirate.drink_bottle(bottles[i].get_energetic_value());
        //         bottles.remove(i);
        //     }
        // }
    }

    pub fn is_treasure_discovered(&mut self) -> bool {
        self.get_pirate().get_x() == self.get_treasure().get_x() && self.get_pirate().get_y() == self.get_treasure().get_y()
    }

    pub fn get_size(&mut self) -> usize {
        self.size
    }

    pub fn set_size(&mut self, new_size: usize) {
        self.size = new_size
    }

    pub fn get_pirate(&mut self) -> &mut Pirate {
        &mut self.pirate
    }

    pub fn get_hunters(&mut self) -> &mut Vec<Hunter_Monkey> {
        &mut self.hunters
    }

    pub fn get_erratics(&mut self) -> &mut Vec<Erratic_Monkey> {
        &mut self.erratics
    }

    pub fn get_bottles(&mut self) -> &mut Vec<Bottle> {
        &mut self.bottles
    }

    pub fn get_treasure(&self) -> Treasure {
        self.treasure.clone()
    }

}
