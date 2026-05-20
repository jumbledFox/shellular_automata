use rand::{RngExt, rng, rngs::ThreadRng};

use crate::simulator::{Simulator, WORLD_SIZE};

const WORLD_AREA: usize = WORLD_SIZE.0*WORLD_SIZE.1;

// 0b00000000
// 0b____snnn - three bits for neighbours, s for state
pub struct Conway {
    cells:   [bool; WORLD_AREA],
    cells_b: [bool; WORLD_AREA],
    rand: ThreadRng,
}

impl Conway {
    pub fn new() -> Self {
        let mut rand = rng();
        let mut cells = [false; WORLD_AREA];
        for c in &mut cells {
            if rand.random_bool(0.5) {
                *c = true;
            }
        }
        Self {
            cells,
            cells_b: [false; WORLD_AREA],
            rand
        }
    }
}

impl Simulator for Conway {
    fn update(&mut self, frame: &mut [u8], window: &winit::window::Window) {
        self.cells_b = self.cells.clone();

        for (i, pix) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % WORLD_SIZE.0;
            let y = i / WORLD_SIZE.0;

            let mut n = 0;
            for (ox, oy) in [
                (-1, -1), ( 0, -1), ( 1, -1),
                (-1,  0),           ( 1,  0),
                (-1,  1), ( 0,  1), ( 1,  1),
            ] {
                let index = i
                    .wrapping_add_signed(ox)
                    .wrapping_add_signed(oy * WORLD_SIZE.1 as isize);
                if self.cells_b[index] {
                    n += 1;
                }
            }
            let alive = self.cells_b[i];
            self.cells[i] = (alive && (n == 2 || n == 3))
            || (!alive && n == 3);

            // self.cells[]

            let color = match self.cells[i] {
                false => [0x00, 0x00, 0x00, 0xff],
                _ => [
                    ((x * 255) / (WORLD_SIZE.0 - 1)) as u8,
                    ((y * 255) / (WORLD_SIZE.1 - 1)) as u8,
                    0xff,
                    0xff,
                ],
            };
            pix.clone_from_slice(&color);
        }
    }

    fn keypress(&mut self, key: winit::keyboard::Key, frame: &mut [u8], window: &winit::window::Window) {
        
    }
}