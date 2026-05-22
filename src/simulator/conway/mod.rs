use rand::{RngExt, rng, rngs::ThreadRng};
use winit::{keyboard::{Key, NamedKey}, window::Window};

use crate::simulator::{Simulator, WORLD_SIZE};

const WORLD_AREA: usize = WORLD_SIZE.0*WORLD_SIZE.1;

// rrrrrgggggbbbbba
// 5 bit rgb, 1 for state
// rrrrrrrr gggggggg bbbbbbbb _______1

// 0b00000000
// 0b____snnn - three bits for neighbours, s for state
pub struct Conway {
    cells:   [u16; WORLD_AREA],
    cells_b: [u16; WORLD_AREA],
    generation: usize,
    stepping: bool,
    opaque:   bool,
    rand: ThreadRng,
}

impl Conway {
    pub fn new() -> Self {
        let mut rand = rng();
        let mut cells = [0; WORLD_AREA];
        for (i, c) in cells.iter_mut().enumerate() {
            if rand.random_bool(0.5) {
                continue;
            }
            let x = i % WORLD_SIZE.0;
            let y = i / WORLD_SIZE.0;

            let inside_middle =
                x > WORLD_SIZE.0/4 && x < (WORLD_SIZE.0/4)*3
            && y > WORLD_SIZE.1/4 && y < (WORLD_SIZE.1/4)*3;

            let quadrant = ((y*32)/WORLD_SIZE.1+(x*32)/WORLD_SIZE.0) % 4;

            let border = match
                x <= 20 || x >= WORLD_SIZE.0-20
            || y <= 20 || y >= WORLD_SIZE.1-20 {
                true => Some(y % 2),
                _ => None,
            };

            *c = 0b1 + (match (border, inside_middle, quadrant) {
                (Some(0), ..) => 0b01100_01100_11111,
                (Some(1), ..) => 0b11111_11111_11111,
                (_, true,  0) |
                (_, true,  2) => 0b11111_01010_01010, // red
                (_, true,  _) => 0b01010_11111_01010, // green
                (_, _, 0)     => 0b11111_10010_11010, // pink
                (_, _, 2)     => 0b00111_10111_11111, // blue
                (_, _, _)     => 0b11111_10101_01000, // orange
            } << 1);
            // println!("{:b}", c);
        }
        Self {
            cells,
            cells_b: [0; WORLD_AREA],
            generation: 0,
            stepping: false,
            opaque:   false,
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
                    
                    let cell = self.cells_b[x + y * WORLD_SIZE.0];
                    if (cell & 1) == 1 {
                        n += 1;
                        for ci in 0..=2 {
                            col[ci] += ((cell >> ((2-ci)*5+1)) & 0b11111) as usize; 
                        }
                    }
                }
            }

            let alive = (self.cells_b[i] & 1) == 1;
            // if alive and have two neighbours, stay alive. if dead with three neighbours, become alive.
            let cell = match (alive && (n == 2 || n == 3)) || (!alive && n == 3) {
                true => {
                    let (r, g, b) = (
                        (col[0]/n) as u8,
                        (col[1]/n) as u8,
                        (col[2]/n) as u8,
                    );
                    pix.clone_from_slice(&[r << 3, g << 3, b << 3, 0xff]);
                    ((r as u16) << 11) +
                    ((g as u16) <<  6) +
                    ((b as u16) <<  1) + 1
                }
                _    => {
                    pix[3] = match self.opaque {
                        true => (pix[3].saturating_add(pix[3] / 3)).min(0xe0),
                        _    => (pix[3] - pix[3] / 3).max(0x08),
                    };
                    pix[3] = 0;
                    0
                },
            };
            // match cell {
            //     Some(c) => pix.clone_from_slice(&[c[0], c[1], c[2], 0xff]),
            //     _ if self.opaque => pix[3] = (pix[3].saturating_add(pix[3] / 3)).min(0xe0),
            //     _ => pix[3] = (pix[3] - pix[3] / 3).max(0x08),
            // }
            self.cells[i] = cell;
        }
        self.generation += 1;
    }

    fn keypress(&mut self, key: Key, _frame: &mut [u8], _window: &Window) {
        if key == Key::Named(NamedKey::Space) {
            self.stepping = !self.stepping;
        }
        if key == Key::Named(NamedKey::Tab) {
            self.opaque = !self.opaque;
        }
    }
}