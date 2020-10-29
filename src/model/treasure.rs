#[derive(Clone)]
pub struct Treasure {
    x: usize,
    y: usize
}

const DEFAULT_TREASURE_X: usize = 9;
const DEFAULT_TREASURE_Y: usize = 2;

impl Treasure {

    pub fn new() -> Self {
        Self{
            x: DEFAULT_TREASURE_X,
            y: DEFAULT_TREASURE_Y
        }
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }
}
