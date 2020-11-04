#[derive(Clone, PartialEq)]
pub struct Bottle {
    x : usize,
    y : usize,
    energetic_value : usize,
    is_visible : bool,
    island_size : usize
}

const DEFAULT_X: usize = 7;
const DEFAULT_Y: usize = 7;
const DEFAULT_ENERGETIC_VALUE: usize = 50;

impl Bottle {
    pub fn new_default(new_island_size: usize) -> Self {
        Self{
            x : DEFAULT_X,
            y : DEFAULT_Y,
            energetic_value : DEFAULT_ENERGETIC_VALUE,
            is_visible : true,
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