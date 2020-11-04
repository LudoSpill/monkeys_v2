#[derive(Clone, PartialEq)]
pub struct Bottle {
    x : usize,
    y : usize,
    energetic_value : usize,
    is_visible : bool,
    is_respawning : bool,
    island_size : usize
}

const DEFAULT_ENERGETIC_VALUE: usize = 50;

impl Bottle {


    pub fn new(new_island_size: usize, new_x: usize, new_y: usize) -> Self {
        Self{
            x : new_x,
            y : new_y,
            energetic_value : DEFAULT_ENERGETIC_VALUE,
            is_visible : true,
            is_respawning : false,
            island_size : new_island_size
        }
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn get_energetic_value(&self) -> usize {
        self.energetic_value
    }

}