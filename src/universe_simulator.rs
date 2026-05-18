use winit::window::Window;

use crate::PIX_SIZE;

const WORLD_WIDTH: usize = PIX_SIZE.0 as usize;

// these are crazy universes that check TWO SPACES left and right, not just 1 ..!
pub struct Universe {
    cells:   [bool; WORLD_WIDTH],
    cells_b: [bool; WORLD_WIDTH],
    rule: u32,
    generation: usize
}

impl Universe {
    pub fn new_center(rule: u32) -> Self {
        let mut cells = [false; WORLD_WIDTH];
        cells[WORLD_WIDTH/2] = true;
        cells[WORLD_WIDTH-1] = true;
        cells[0] = true;
        Self {
            cells,
            cells_b: [false; WORLD_WIDTH],
            rule,
            generation: 0,
        }
    }

    pub fn step(&mut self) {
        self.cells_b = self.cells.clone();
        // For each cell, look at its past neighbours
        for i in 0..WORLD_WIDTH {
            // left2, left1, center, right1, right2 cells
            let l2 = match i > 1 {
                true => self.cells_b[i-2],
                _ => false
            };
            let l1 = match i > 0 {
                true => self.cells_b[i-1],
                _ => false
            };
            let c = self.cells_b[i];
            let r1 = match i+1 < WORLD_WIDTH {
                true => self.cells_b[i+1],
                _ => false,
            };
            let r2 = match i+2 < WORLD_WIDTH {
                true => self.cells_b[i+2],
                _ => false,
            };

            // construct a 5 digit binary number from 0 - 32 out of these values
            let v = 
              ((r2 as usize) << 0)
            + ((r1 as usize) << 1)
            + ((c  as usize) << 2)
            + ((l1 as usize) << 3)
            + ((l2 as usize) << 4);
            self.cells[i] = ((self.rule >> v) & 0b1) == 0b1;
        }
        self.generation += 1;
    }
}

pub struct UniverseSimulator {
    rule: u32,
}

impl UniverseSimulator {
    pub fn new(rule: u32, frame: &mut [u8], window: &Window) -> Self {
        let s = Self {
            rule
        };
        s.remake_frame(frame, window);
        s
    }

    pub fn shimmy(&mut self, amount: i32, frame: &mut [u8], window: &Window) {
        self.rule = self.rule.wrapping_add_signed(amount);
        self.remake_frame(frame, window);
    }

    pub fn flip(&mut self, frame: &mut [u8], window: &Window) {
        self.rule = !self.rule;
        self.remake_frame(frame, window);
    }

    pub fn remake_frame(&self, frame: &mut [u8], window: &Window) {
        window.set_title(&format!("sillyular pawtomata - rule  {:?}", self.rule));
        
        let mut universe = Universe::new_center(self.rule);
        for y in 0..PIX_SIZE.1 as usize {
            let alive = hue_to_rgb(y as f32 / PIX_SIZE.1 as f32);
            let dead = [0x20, 0x20, 0x20, 0xff];

            let cells = universe.cells;
            for x in 0..WORLD_WIDTH {
                let i = (x + y * WORLD_WIDTH) * 4; 
                let color = match cells[x] {
                    true => alive,
                    _    => dead,
                };
                frame[i..i+4].copy_from_slice(&color); 
            }
            universe.step();
        }
    }

    /*
    for x in 0..WORLD_WIDTH {
        let i = (x + y * WORLD_WIDTH) * 4;
        let color = match self.cells[x] {
            true => [0xff, 0x00, 0x00, 0xff],
            _    => [0x20, 0x20, 0x20, 0xff],
        };
        frame[i..i+4].copy_from_slice(&color);
    }
     */
}

fn hue_to_rgb(h: f32) -> [u8; 4] {
    let min_3 = |a: f32, b: f32, c: f32| -> f32 {
        a.min(b.min(c))
    };

    let kr = (5.0 + h*6.0) % 6.0;
    let kg = (3.0 + h*6.0) % 6.0;
    let kb = (1.0 + h*6.0) % 6.0;
    let r = 1.0 - min_3(kr, 4.0-kr, 1.0).max(0.0);
    let g = 1.0 - min_3(kg, 4.0-kg, 1.0).max(0.0);
    let b = 1.0 - min_3(kb, 4.0-kb, 1.0).max(0.0);
    [
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        0xff
    ]
}