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
    pub fn new(window: &Window) -> Self {
        Self::update_title(window, 0);

        Self {
            cells:   [None; WORLD_AREA],
            cells_b: [None; WORLD_AREA],
            generation: 0,
            stepping: false,
            opaque:   false,
            artist: Artist::new(),
            rand: rng(),
            shift_counter: 0,

            draw_hue: 0.0,
        }
    }

    fn update_title(window: &Window, generation: usize) {
        window.set_title(&format!("colour drawing conways - generation {:?} - made by jumbledFox :3", generation));
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
                self.cells[..].rotate_right(WORLD_SIZE.0.saturating_add_signed(self.rand.random_range(-1..=1) as isize));
                self.shift_counter = self.rand.random_range(16..24);
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
            Self::update_title(window, self.generation);
        }
    }
}