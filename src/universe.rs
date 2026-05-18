use crate::WIDTH;

const WORLD_SIZE: usize = WIDTH as usize;

pub struct Universe {
    cells:   [bool; WORLD_SIZE],
    cells_b: [bool; WORLD_SIZE],
    rule: u8,
}

impl Universe {
    pub fn new_middle(rule: u8) -> Self {
        let mut cells = [false; WORLD_SIZE];
        cells[WORLD_SIZE/2] = true;
        Self {
            cells,
            cells_b: [false; WORLD_SIZE],
            rule,
        }
    }

    pub fn step(&mut self) {
        self.cells_b = self.cells.clone();
        // For each cell, look at its past neighbours
        for i in 0..WORLD_SIZE {
            let left  = i.checked_sub(1);
            let above = i;
            let right = match i+1 < WORLD_SIZE {
                true => Some(i+1),
                false => None
            };

            left.unwrap();
        }
    }
}