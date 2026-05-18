use crate::PIX_SIZE;

const WORLD_WIDTH: usize = PIX_SIZE.0 as usize;

pub struct Universe {
    cells:   [bool; WORLD_WIDTH],
    cells_b: [bool; WORLD_WIDTH],
    rule: u8,
    generation: usize
}

impl Universe {
    pub fn new_middle(rule: u8) -> Self {
        let mut cells = [false; WORLD_WIDTH];
        cells[WORLD_WIDTH/2] = true;
        Self {
            cells,
            cells_b: [false; WORLD_WIDTH],
            rule,
            generation: 0,
        }
    }
    
    pub fn update(&mut self, frame: &mut [u8]) {
        self.cells_b = self.cells.clone();
        // For each cell, look at its past neighbours
        for i in 0..WORLD_WIDTH {
            // left, center, and right cells
            let l = match i != 0 {
                true => self.cells_b[i-1],
                _ => false
            };
            let c = self.cells_b[i];
            let r = match i+1 < WORLD_WIDTH {
                true => self.cells_b[i+1],
                _ => false,
            };
            // construct a 3 digit binary number from 0 - 7 out of these values
            let v = (r as usize) + ((c as usize) << 1) + ((l as usize) << 2);
            self.cells[i] = ((self.rule >> v) & 0b1) == 0b1;
            // self.cells[i] = c;
        }
        // Shift previous lines up, if out of space
        let max_h = PIX_SIZE.1 as usize - 1;
        let y = self.generation.clamp(0, max_h);
        if self.generation > max_h {
            frame.copy_within(WORLD_WIDTH * 4.., 0);
        }

        // Draw the next line
        for x in 0..WORLD_WIDTH {
            let i = (x + y * WORLD_WIDTH) * 4;
            let color = match self.cells[x] {
                true => [0xff, 0x00, 0x00, 0xff],
                _    => [0x20, 0x20, 0x20, 0xff],
            };
            frame[i..i+4].copy_from_slice(&color);
        }
        self.generation += 1;
    }
}