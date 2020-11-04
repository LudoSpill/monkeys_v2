#[derive(Clone)]
pub struct Treasure {
    x: usize,
    y: usize
}

impl Treasure {

    pub fn new(new_x: usize, new_y: usize) -> Self {
        Self{
            x: new_x,
            y: new_y
        }
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }
}
