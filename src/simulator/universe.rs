use rand::{RngExt, rngs::ThreadRng};

use crate::simulator::WORLD_WIDTH;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitMode {
    Random,
    Center,
}

// these are crazy universes that check TWO SPACES left and right, not just 1 ..!
pub struct Universe {
    cells:   [bool; WORLD_WIDTH],
    cells_b: [bool; WORLD_WIDTH],
    rule: u32,
    //   91989028 - isometric-ish
    //   91989105 - cool branches in organic noise
    //   91988539 - pipes
    //   91988546 - cool stripes
    //   91988610 - roller coaster tycoon
    //   91988617 - sierpinski park
    //  631494308 - motherboard
    // 4202978740 - puzzle triangles
    // 4202978768 - cool pattern converges
    // 4202978834 - cityblock
    // 4202978859 - a trip down the side of the spikey building
    generation: usize,
}

impl Universe {
    pub fn new(rule: u32, init_mode: InitMode, rand: &mut ThreadRng) -> Self {
        let mut cells = [false; WORLD_WIDTH];

        if init_mode == InitMode::Center {
            cells[WORLD_WIDTH/2] = true;
        } else if init_mode == InitMode::Random {
            for i in 0..WORLD_WIDTH {
                cells[i] = rand.random_bool(0.5)
            }
        }

        Self {
            cells,
            cells_b: [false; WORLD_WIDTH],
            rule,
            generation: 0,
        }
    }

    pub fn cells(&self) -> &[bool; WORLD_WIDTH] {
        &self.cells
    }
    pub fn generation(&self) -> usize {
        self.generation
    }

    pub fn step(&mut self) {
        self.cells_b = self.cells.clone();
        // For each cell, look at its past neighbours
        for i in 0..WORLD_WIDTH {
            // rem_euclid for a looping universe
            let index = |offset: isize| -> usize {
                // TODO: probably a much nicer way to do this...
                (i as isize + offset).rem_euclid(WORLD_WIDTH as isize) as usize
            };

            let l2 = index(-2);
            let l1 = index(-1);
            let c  = index( 0);
            let r1 = index( 1);
            let r2 = index( 2);

            // construct a 5 digit binary number from 0 - 32 out of these values
            let v = 
              ((self.cells_b[r2] as usize) << 0)
            + ((self.cells_b[r1] as usize) << 1)
            + ((self.cells_b[c]  as usize) << 2)
            + ((self.cells_b[l1] as usize) << 3)
            + ((self.cells_b[l2] as usize) << 4);
            self.cells[i] = ((self.rule >> v) & 0b1) == 0b1;
        }
        self.generation += 1;
    }
}