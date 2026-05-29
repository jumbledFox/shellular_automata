use rand::{RngExt, rng, rngs::ThreadRng};
use winit::{event::MouseButton, keyboard::{Key, NamedKey}, window::Window};

use crate::{input::Input, simulator::{Simulator, WORLD_SIZE, conway::artist::Artist}};

const WORLD_AREA: usize = WORLD_SIZE.0*WORLD_SIZE.1;

mod artist;

// 0b00000000
// 0b____snnn - three bits for neighbours, s for state
pub struct Conway {
    cells:   [Option<[u8; 3]>; WORLD_AREA],
    cells_b: [Option<[u8; 3]>; WORLD_AREA],
    generation: usize,
    stepping: bool,
    opaque:   bool,
    artist: Artist,
    rand: ThreadRng,
    shift_counter: usize,

    draw_hue: f32,
}

impl Conway {
    pub fn new(frame: &mut [u8]) -> Self {
        let mut rand = rng();
        let mut cells = [None; WORLD_AREA];
        for (i, c) in cells.iter_mut().enumerate() {
            break;
            if rand.random_bool(0.8) {
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

            *c = match (border, inside_middle, quadrant) {
                // (_, _, 0) | (.., 2) => Some([0xff, 0x44, 0x44]), // red
                // (_, _, _) => Some([0x44, 0x44, 0xff]), // blue
                (Some(0), ..) => Some([0x67, 0x67, 0xff]),
                (Some(1), ..) => Some([0xff, 0xff, 0xff]),
                (_, true,  0) |
                (_, true,  2) => Some([0xff, 0x55, 0x55]), // red
                (_, true,  _) => Some([0x55, 0xff, 0x55]), // green
                (_, _, 0)     => Some([0xfc, 0x92, 0xd2]), // pink
                (_, _, 2)     => Some([0x3e, 0xb8, 0xfa]), // blue
                (_, _, _)     => Some([0xff, 0xa8, 0x45]), // orange
            };
        }

        for (i, pix) in frame.chunks_exact_mut(4).enumerate() {
            if let Some(c) = cells[i] {
                pix.clone_from_slice(&[c[0], c[1], c[2], 0xff]);
            }
        }

        Self {
            cells,
            cells_b: [None; WORLD_AREA],
            generation: 0,
            stepping: false,
            opaque:   false,
            artist: Artist::new(),
            rand,
            shift_counter: 0,

            draw_hue: 0.0,
        }
    }

    fn update_title(&self, window: &Window) {
        window.set_title(&format!("silly funny colour conways - generation {:?}", self.generation));
    }
}

impl Simulator for Conway {
    fn update(&mut self, input: &Input, frame: &mut [u8], window: &Window) {
        // toggling settings
        if input.key_pressed(Key::Named(NamedKey::Space)) {
            self.stepping = !self.stepping;
        }
        if input.key_pressed(Key::Named(NamedKey::Tab)) {
            self.opaque = !self.opaque;
        }

        if self.stepping {
            self.artist.update(&mut self.cells, &mut self.rand);
            if self.shift_counter == 0 {
                let rot = WORLD_SIZE.0.saturating_add_signed(self.rand.random_range(-3..=3i8).signum() as isize);
                match self.rand.random_bool(0.5) {
                    true => self.cells[..].rotate_right(rot),
                    _    => self.cells[..].rotate_left(rot),
                };
                self.shift_counter = self.rand.random_range(8..16);
            }
            self.shift_counter -= 1;
        }

        self.cells_b = self.cells.clone();
        
        // updating and drawing to the screen
        for (i, pix) in frame.chunks_exact_mut(4).enumerate() {
            // if we're stepping through the grid, we want to update all cells and then draw them
            let cell = if self.stepping || input.key_pressed(Key::Character(".".into())) {
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
                                col[ci] += (c[ci] as usize).pow(2); 
                            }
                        }
                    }
                }
    
                let alive = self.cells_b[i].is_some();
                // if alive and have two neighbours, stay alive. if dead with three neighbours, become alive.
                match (alive && (n == 2 || n == 3)) || (!alive && (n == 3)) {
                // match (alive && (n == 2 || n == 3)) || (!alive && (n == 3 || n == 4)) { // blender
                    true => Some([
                        (col[0]/n).isqrt() as u8,
                        (col[1]/n).isqrt() as u8,
                        (col[2]/n).isqrt() as u8,
                    ]),
                    _    => None,
                }
            } else {
                // otherwise, we want to just redraw the cell
                self.cells[i]
            };
            match cell {
                Some(c) => pix.clone_from_slice(&[c[0], c[1], c[2], 0xff]),
                _ if self.opaque => pix[3] = (pix[3].saturating_add(pix[3] / 3)).min(0xe0),
                _ => pix[3] = (pix[3] - pix[3] / 3).max(0x08),
            }
            self.cells[i] = cell;
        }
        if self.stepping {
            self.generation += 1;
            self.update_title(window);
        }
    }
}