use super::island;
use island::Direction;

#[derive(Clone, Copy, Debug)]
pub struct Pirate {
    x: usize,
    y: usize,
    current_energy: i8,
    max_energy: i8,
    is_alive: bool,
    is_drunk: bool,
    island_size: usize
}

const DEFAULT_X: usize = 0;
const DEFAULT_Y: usize = 0;
const MAX_ENERGY: i8 = 100;


impl Pirate {

    pub fn new_default(new_island_size: usize) -> Self {
        Self{
            x : DEFAULT_X,
            y : DEFAULT_Y,
            current_energy : MAX_ENERGY,
            max_energy : MAX_ENERGY,
            is_alive : true,
            is_drunk : false,
            island_size : new_island_size
        }
    }


    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn set_x(&mut self, new_x: usize) {
        self.x = new_x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn set_y(&mut self, new_y: usize) {
        self.y = new_y
    }

    pub fn get_current_energy(&self) -> i8 {
        self.current_energy.clone()
    }

    pub fn set_current_energy(&mut self, new_energy: i8) {
        self.current_energy = new_energy
    }
    
    pub fn get_max_energy(&self) -> i8 {
        self.max_energy.clone()
    }

    pub fn get_alive(&self) -> bool {
        self.is_alive
    }

    pub fn set_alive(&mut self, new_alive: bool){
        self.is_alive = new_alive
    }

    pub fn drink_bottle(&mut self, energetic_value: usize){
        self.set_current_energy(self.get_current_energy() + energetic_value as i8);
    }

    pub fn move_pirate(&mut self, dir: Direction) {
        let mut moved = false;
        match dir {
            Direction::UP => 
            if self.x > 0 && dir == Direction::UP {
                self.set_x(self.x-1);
                moved = true;
            },
            Direction::DOWN =>
            if self.x < self.island_size-1 && dir == Direction::DOWN {
                self.set_x(self.x+1);
                moved = true;       
            },
            Direction::LEFT =>
            if self.y > 0 && dir == Direction::LEFT {
                self.set_y(self.y-1);
                moved = true;
            },
            Direction::RIGHT =>
            if self.y < self.island_size-1 && dir == Direction::RIGHT {
                self.set_y(self.y+1);
                moved = true;             
            }
            _ => ()
        }

        if moved == true {
            if self.get_current_energy() - 5 < 0 {
                self.set_current_energy(0);
            }
            else{
                self.set_current_energy(self.get_current_energy()-5);
            }

            if self.get_current_energy() == 0 {
                self.set_alive(false);
            }

        }
    }

}