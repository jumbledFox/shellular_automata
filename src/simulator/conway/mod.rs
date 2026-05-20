use rand::{RngExt, rng, rngs::ThreadRng};
use winit::{keyboard::{Key, NamedKey}, window::Window};

use crate::simulator::{Simulator, WORLD_SIZE};

const WORLD_AREA: usize = WORLD_SIZE.0*WORLD_SIZE.1;

// 0b00000000
// 0b____snnn - three bits for neighbours, s for state
pub struct Conway {
    cells:   [Option<[u8; 3]>; WORLD_AREA],
    cells_b: [Option<[u8; 3]>; WORLD_AREA],
    generation: usize,
    stepping: bool,
    rand: ThreadRng,
}

impl Conway {
    pub fn new() -> Self {
        let mut rand = rng();
        let mut cells = [None; WORLD_AREA];
        for (i, c) in cells.iter_mut().enumerate() {
            if rand.random_bool(0.2) {
                let x = i % WORLD_SIZE.0;
                let y = i / WORLD_SIZE.0;

                let inside_middle =
                   x > WORLD_SIZE.0/4 && x < (WORLD_SIZE.0/4)*3
                && y > WORLD_SIZE.1/4 && y < (WORLD_SIZE.1/4)*3;

                let quadrant = ((y*32)/WORLD_SIZE.1+(x*32)/WORLD_SIZE.0) % 4;

                *c = match (inside_middle, quadrant) {
                    (true,  0) |
                    (true,  2) => Some([0xff, 0x55, 0x55]), // red
                    (true,  _) => Some([0x55, 0xff, 0x55]), // green
                    (_, 0) => Some([0xfc, 0x92, 0xd2]), // pink
                    (_, 2) => Some([0x3e, 0xb8, 0xfa]), // blue
                    (_, _) => Some([0xff, 0xa8, 0x45]), // orange
                };
            }
        }
        Self {
            cells,
            cells_b: [None; WORLD_AREA],
            generation: 0,
            stepping: false,
            rand
        }
    }

    fn update_title(&self, window: &Window) {
        window.set_title(&format!("silly funny colour conways - generation {:?}", self.generation));
    }
}

impl Simulator for Conway {
    fn update(&mut self, frame: &mut [u8], window: &Window) {
        if !self.stepping && self.generation != 0 {
            return; 
        }
        self.update_title(window);
        self.cells_b = self.cells.clone();

        for (i, pix) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % WORLD_SIZE.0;
            let y = i / WORLD_SIZE.0;

            let mut n = 0;
            let mut col = [0, 0, 0];
            for offset_y in -1..=1 {
                // we add the height/width to make sure we properly wrap at boundaries!
                let y = (WORLD_SIZE.1 + y)
                    .wrapping_add_signed(offset_y)
                    .rem_euclid(WORLD_SIZE.1);
                for offset_x in -1..=1 {
                    if offset_x == 0 && offset_y == 0 {
                        continue;
                    }

                    let x = (WORLD_SIZE.0 + x) 
                        .wrapping_add_signed(offset_x)
                        .rem_euclid(WORLD_SIZE.0);
                    
                    if let Some(c) = self.cells_b[x + y * WORLD_SIZE.0] {
                        n += 1;
                        for ci in 0..=2 {
                            col[ci] += c[ci] as usize; 
                        }
                    }
                }
            }

            let alive = self.cells_b[i].is_some();
            // if alive and have two neighbours, stay alive. if dead with three neighbours, become alive.
            let cell = match (alive && (n == 2 || n == 3)) || (!alive && n == 3) {
                true => Some([
                    (col[0]/n) as u8,
                    (col[1]/n) as u8,
                    (col[2]/n) as u8,
                ]),
                _    => None,
            };
            match cell {
                Some(c) => pix.clone_from_slice(&[c[0], c[1], c[2], 0xff]),
                _ => pix[3] = (pix[3] - pix[3] / 3).max(8),
            }
            self.cells[i] = cell;
        }
        self.generation += 1;
    }

    fn keypress(&mut self, key: Key, _frame: &mut [u8], _window: &Window) {
        if key == Key::Named(NamedKey::Space) {
            self.stepping = !self.stepping;
        }
    }
}