use rand::Rng;
use super::pirate::Pirate;
use super::island::Direction;

pub trait MonkeyMove {
    fn monkey_move(&mut self, pirate: Pirate, hunters: Vec<HunterMonkey>, erratics: Vec<ErraticMonkey>);
}

#[derive(Clone, Copy, PartialEq)]
pub struct HunterMonkey {
    x: i8,
    y: i8,
    island_size: usize
}


impl HunterMonkey {
    
    pub fn new(new_island_size: usize, new_x: i8, new_y: i8) -> Self {
        Self {
            x : new_x,
            y : new_y,
            island_size : new_island_size
        }
    }


    pub fn get_x(&self) -> i8 {
        self.x
    }

    pub fn set_x(&mut self, new_x: i8) {
        self.x = new_x
    }

    pub fn get_y(&self) -> i8 {
        self.y
    }

    pub fn set_y(&mut self, new_y: i8) {
        self.y = new_y
    }

    fn get_angle_to_pirate(&self, pirate: Pirate) -> f64 {
        
        let x_dist: f64 = pirate.get_x() as f64 - self.x as f64;
        let y_dist: f64 = pirate.get_y() as f64 - self.y as f64;
                
        let angle = y_dist.atan2(x_dist);

        angle
    }

    /**
     * Function that checks which directions are available to the monkey
     */
    fn get_unavailable_directions(&self, hunters: Vec<HunterMonkey>, erratics: Vec<ErraticMonkey>) -> Vec<Direction> {
        let mut unavailable_directions : Vec<Direction> = Vec::new();

        // Borders checking
        if self.x == 0 {
            unavailable_directions.push(Direction::UP);
        }
        if self.x == (self.island_size-1) as i8 {
            unavailable_directions.push(Direction::DOWN);
        }
        if self.y == 0 {
            unavailable_directions.push(Direction::LEFT);
        }
        if self.y == (self.island_size-1) as i8 {
            unavailable_directions.push(Direction::RIGHT);
        }

        // Monkeys checking
        for erratic in erratics.iter() {
            if (erratic.get_y() == self.y) && (erratic.get_x() == self.x-1) {
                unavailable_directions.push(Direction::UP);
            }
            else if (erratic.get_y() == self.y) && (erratic.get_x() == self.x+1) {
                unavailable_directions.push(Direction::DOWN);
            }
            else if (erratic.get_x() == self.x) && (erratic.get_y() == self.y-1) {
                unavailable_directions.push(Direction::LEFT);
            }
            else if (erratic.get_x() == self.x) && (erratic.get_y() == self.y+1) {
                unavailable_directions.push(Direction::RIGHT);
            }
        }

        for hunter in hunters.iter() {
            if hunter != self { // Excluding the current monkey 
                if (hunter.get_y() == self.y) && (hunter.get_x() == self.x-1) {
                    unavailable_directions.push(Direction::UP);
                }
                else if (hunter.get_y() == self.y) && (hunter.get_x() == self.x+1) {
                    unavailable_directions.push(Direction::DOWN);
                }
                else if (hunter.get_x() == self.x) && (hunter.get_y() == self.y-1) {
                    unavailable_directions.push(Direction::LEFT);
                }
                else if (hunter.get_x() == self.x) && (hunter.get_y() == self.y+1) {
                    unavailable_directions.push(Direction::RIGHT);
                }
            }
        }

        unavailable_directions
    }

    fn get_potential_direction(angle: f64) -> Vec<Direction> {
        let mut potential_directions: Vec<Direction> = Vec::new();

        // 4 special cases:
        if angle == HunterMonkey::to_radian(180.0) {
            potential_directions.push(Direction::UP);
            // potential_directions.push(Direction::LEFT);
            // potential_directions.push(Direction::RIGHT);

        }
        else if angle == HunterMonkey::to_radian(-90.0) {
            potential_directions.push(Direction::LEFT);
            // potential_directions.push(Direction::UP);
            // potential_directions.push(Direction::DOWN);

        }
        else if angle == 0.0 {
            potential_directions.push(Direction::DOWN);
            // potential_directions.push(Direction::LEFT);
            // potential_directions.push(Direction::RIGHT);
        }
        else if angle == HunterMonkey::to_radian(90.0) {
            potential_directions.push(Direction::RIGHT);
            // potential_directions.push(Direction::UP);
            // potential_directions.push(Direction::DOWN);
        }

        // 8 nominal cases:
        else if angle > HunterMonkey::to_radian(-180.0) && angle <= HunterMonkey::to_radian(-135.0) {   // #1
            potential_directions.push(Direction::UP);
            potential_directions.push(Direction::LEFT);            
        }
        else if angle >= HunterMonkey::to_radian(-135.0) && angle < HunterMonkey::to_radian(-90.0) {    // #2
            potential_directions.push(Direction::LEFT);
            potential_directions.push(Direction::UP);            

        }
        else if angle > HunterMonkey::to_radian(-90.0) && angle <= HunterMonkey::to_radian(-45.0) {     // #3
            potential_directions.push(Direction::LEFT);
            potential_directions.push(Direction::DOWN);            

        }
        else if angle >= HunterMonkey::to_radian(-45.0) && angle < 0.0 {           // #4
            potential_directions.push(Direction::DOWN);
            potential_directions.push(Direction::LEFT);            
        }
        else if angle > 0.0 && angle <= HunterMonkey::to_radian(45.0) {            // #5
            potential_directions.push(Direction::DOWN);
            potential_directions.push(Direction::RIGHT);            
        }
        else if angle >= HunterMonkey::to_radian(45.0) && angle < HunterMonkey::to_radian(90.0) {    // #6
            potential_directions.push(Direction::RIGHT);
            potential_directions.push(Direction::DOWN);            
        }
        else if angle > HunterMonkey::to_radian(90.0) && angle <= HunterMonkey::to_radian(135.0) {   // #7
            potential_directions.push(Direction::RIGHT);
            potential_directions.push(Direction::UP);            
        }
        else if angle >= HunterMonkey::to_radian(135.0) && angle < HunterMonkey::to_radian(180.0) {  // #8
            potential_directions.push(Direction::UP);
            potential_directions.push(Direction::RIGHT);            
        }
        else {
            println!("Wrong angle : {}\r", angle);
        }
        potential_directions.push(Direction::NONE);

        potential_directions
    }

    fn to_radian(angle: f64) -> f64 {
        angle * std::f64::consts::PI / 180.0
    }

}

impl MonkeyMove for HunterMonkey{
    fn monkey_move(&mut self, pirate: Pirate, hunters: Vec<HunterMonkey>, erratics: Vec<ErraticMonkey>){
        let angle_to_pirate = self.get_angle_to_pirate(pirate);
        println!("Angle to pirate : {}\r",angle_to_pirate);

        let pot_directions = HunterMonkey::get_potential_direction(angle_to_pirate);
        println!("potential dirs : {:?}\r", pot_directions);
        let unavailable_directions = self.get_unavailable_directions(hunters, erratics);
        println!("unavail dirs : {:?}\r", unavailable_directions);
        let mut actual_direction_indice = 0;

        for i in 0..pot_directions.len() {
            for unavail_dir in unavailable_directions.iter() {
                if pot_directions[i] == unavail_dir.clone() {
                    actual_direction_indice +=1;
                }
            }
        }

        let actual_direction = pot_directions[actual_direction_indice].clone();
        // println!("Chosen dir: {:?}\r", actual_direction);

        match actual_direction {
            Direction::UP => {
                self.set_x(self.x-1);
                if self.x < 0 {self.x = 0};
            },
            Direction::DOWN => self.set_x(self.x+1),
            Direction::LEFT => {
                self.set_y(self.y-1);
                if self.y < 0 {self.y = 0};
            },
            Direction::RIGHT => self.set_y(self.y+1),
            _ => ()
        }
        
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct ErraticMonkey {
    x: i8,
    y: i8,
    island_size: usize
}

impl ErraticMonkey {


    pub fn new(new_island_size: usize, new_x: i8, new_y: i8) -> Self {
        ErraticMonkey{
            x : new_x,
            y : new_y,
            island_size : new_island_size
        }
    }

    pub fn get_x(&self) -> i8 {
        self.x
    }

    pub fn set_x(&mut self, new_x: i8) {
        self.x = new_x
    }

    pub fn get_y(&self) -> i8 {
        self.y
    }

    pub fn set_y(&mut self, new_y: i8) {
        self.y = new_y
    }

    /**
     * Function that checks which directions are available to the monkey
     */
    fn get_available_directions(&self, hunters: Vec<HunterMonkey>, erratics: Vec<ErraticMonkey>) -> Vec<Direction> {
        let mut available_directions : Vec<Direction> = Vec::new();

        // Borders checking
        if self.x > 0 {
            available_directions.push(Direction::UP);
        }
        if self.x < (self.island_size-1) as i8 {
            available_directions.push(Direction::DOWN);
        }
        if self.y > 0 {
            available_directions.push(Direction::LEFT);
        }
        if self.y < (self.island_size-1) as i8 {
            available_directions.push(Direction::RIGHT);
        }

        // Monkeys checking
        let mut unavailable_directions : Vec<Direction> = Vec::new();
        for erratic in erratics.iter() {
            if erratic != self { // Excluding the current monkey
                if (erratic.get_y() == self.y) && (erratic.get_x() == self.x-1) {
                    unavailable_directions.push(Direction::UP);
                }
                else if (erratic.get_y() == self.y) && (erratic.get_x() == self.x+1) {
                    unavailable_directions.push(Direction::DOWN);
                }
                else if (erratic.get_x() == self.x) && (erratic.get_y() == self.y-1) {
                    unavailable_directions.push(Direction::LEFT);
                }
                else if (erratic.get_x() == self.x) && (erratic.get_y() == self.y+1) {
                    unavailable_directions.push(Direction::RIGHT);
                }
            }
        }

        for hunter in hunters.iter() {
            if (hunter.get_y() == self.y) && (hunter.get_x() == self.x-1) {
                unavailable_directions.push(Direction::UP);
            }
            else if (hunter.get_y() == self.y) && (hunter.get_x() == self.x+1) {
                unavailable_directions.push(Direction::DOWN);
            }
            else if (hunter.get_x() == self.x) && (hunter.get_y() == self.y-1) {
                unavailable_directions.push(Direction::LEFT);
            }
            else if (hunter.get_x() == self.x) && (hunter.get_y() == self.y+1) {
                unavailable_directions.push(Direction::RIGHT);
            }
        }

        for wrong_dir in unavailable_directions.iter() {
            for i in 0..available_directions.len()-1 {
                if wrong_dir.clone() == available_directions[i] {
                    available_directions.remove(i);
                }
            }
        }

        // println!("dir : {:?}\r", available_directions);
        available_directions
    }

}

impl MonkeyMove for ErraticMonkey{
    fn monkey_move(&mut self, _pirate: Pirate, hunters: Vec<HunterMonkey>, erratics: Vec<ErraticMonkey>) {

        let available_directions = self.get_available_directions(hunters, erratics);
        let mut rng = rand::thread_rng();
        let dir = rng.gen_range(0,available_directions.len());

        match available_directions[dir].clone() {
            Direction::UP => self.set_x(self.x-1),
            Direction::DOWN => self.set_x(self.x+1),
            Direction::LEFT => self.set_y(self.y-1),
            Direction::RIGHT => self.set_y(self.y+1),
            _ => ()
        }
    }
}

