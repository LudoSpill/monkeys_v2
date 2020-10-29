use rand::Rng;

pub trait MonkeyMove {
    fn monkey_move(&mut self);
}

#[derive(Clone)]
pub struct Hunter_Monkey {
    x: usize,
    y: usize,
    speed: usize,
    island_size: usize
}

const DEFAULT_HUNTER_X: usize = 5;
const DEFAULT_HUNTER_Y: usize = 5;
const DEFAULT_SPEED: usize = 0;

impl Hunter_Monkey {

    pub fn new_default(new_island_size: usize) -> Self {
        Hunter_Monkey{
            x : DEFAULT_HUNTER_X,
            y : DEFAULT_HUNTER_Y,
            speed : DEFAULT_SPEED,
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

}

impl MonkeyMove for Hunter_Monkey{
    fn monkey_move(&mut self){
        //TODO
    }


}

#[derive(Clone)]
pub struct Erratic_Monkey {
    x: usize,
    y: usize,
    island_size: usize
}

const DEFAULT_ERRATIC_X: usize = 7;
const DEFAULT_ERRATIC_Y: usize = 3;

impl Erratic_Monkey {

    pub fn new_default(new_island_size: usize) -> Self {
        Erratic_Monkey{
            x : DEFAULT_ERRATIC_X,
            y : DEFAULT_ERRATIC_Y,
            island_size : new_island_size
        }
    }

    pub fn new(new_island_size: usize, new_x: usize, new_y: usize) -> Self {
        Erratic_Monkey{
            x : new_x,
            y : new_y,
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

}

impl MonkeyMove for Erratic_Monkey{
    fn monkey_move(&mut self){
        let mut rng = rand::thread_rng();
        let dir = rng.gen_range(0,3);
        match dir {
            // UP
            0 => if self.x > 0 {
                self.set_x(self.x-1);
            },
            // DOWN
            1 =>
            if self.x < self.island_size-1 {
                self.set_x(self.x+1);
            },
            // LEFT
            2 =>
            if self.y > 0 {
                self.set_y(self.y-1);
            },
            // RIGHT
            3 =>
            if self.y < self.island_size-1 {
                self.set_y(self.y+1);
            }

            _ => print!("This random generator is not so good :(\r")
        }
    }

}

