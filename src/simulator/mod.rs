use rand::{rng, rngs::ThreadRng};
use winit::window::Window;

use crate::{PIX_SIZE, simulator::universe::{InitMode, Universe}};

pub mod universe;

const WORLD_WIDTH: usize = PIX_SIZE.0 as usize;

pub struct UniverseSimulator {
    universe: Universe,
    stepping: bool,
    rule: u32,
    init_mode: InitMode,
    hue: f32,
    rand: ThreadRng,
}

impl UniverseSimulator {
    pub fn new(rule: u32, init_mode: InitMode, frame: &mut [u8], window: &Window) -> Self {
        let mut rand = rng();
        // TODO: there must be a nicer possible 'new' function...
        let mut s = Self {
            universe: Universe::new(rule, init_mode, &mut rand),
            rule,
            init_mode,
            stepping: true,
            hue: 0.0,
            rand: rng(),
        };
        s.remake_universe(frame, window);
        s
    }

    pub fn shimmy(&mut self, amount: i32, frame: &mut [u8], window: &Window) {
        self.rule = self.rule.wrapping_add_signed(amount);
        self.remake_universe(frame, window);
    }

    pub fn flip(&mut self, frame: &mut [u8], window: &Window) {
        self.rule = !self.rule;
        self.remake_universe(frame, window);
    }

    pub fn toggle_init_mode(&mut self, frame: &mut [u8], window: &Window) {
        self.init_mode = match self.init_mode {
            InitMode::Center => InitMode::Random,
            InitMode::Random => InitMode::Center,
        };
        self.remake_universe(frame, window);
    }


    pub fn update_title(&self, window: &Window) {
        let init_mode = match self.init_mode {
            InitMode::Center => "",
            InitMode::Random => " - Random"
        };
        window.set_title(&format!("sillyular pawtomata - rule  {:?} - generation {:?}{init_mode}", self.rule, self.universe.generation()));
    }

    pub fn remake_universe(&mut self, frame: &mut [u8], window: &Window) {
        // clear the framebuffer to black
        for pix in frame.chunks_mut(4) {
            pix.clone_from_slice(&[0x00, 0x00, 0x00, 0xff]);
        }
        self.hue = ((self.rule%295).pow(2) as f32 * 0.09).rem_euclid(1.0); // psuedo-random number :trollface:
        self.universe = Universe::new(self.rule, self.init_mode, &mut self.rand);
        // step the universe forward so we can see a full screen
        for i in 0..PIX_SIZE.1 as usize {
            self.draw_step(i, frame);
            self.universe.step();
        }

        self.update_title(window);
    }

    pub fn stepping(&self) -> bool {
        self.stepping
    }
    pub fn toggle_stepping(&mut self) {
        self.stepping = !self.stepping;
    }

    pub fn step_simulation(&mut self, frame: &mut [u8], window: &Window) {
        frame.copy_within(WORLD_WIDTH * 4.., 0);
        self.draw_step(PIX_SIZE.1 as usize - 1, frame);
        self.update_title(window);
        self.universe.step();
    }

    fn draw_step(&mut self, y: usize, frame: &mut [u8]) {
        let alive = hue_to_rgb(self.hue);
        for x in 0..WORLD_WIDTH {
            let i = (x + y * WORLD_WIDTH) * 4;
            let color = match self.universe.cells()[x] {
                true => alive,
                _    => [0x00, 0x00, 0x00, 0xff],
            };
            frame[i..i+4].copy_from_slice(&color);
        }
        self.hue += 0.0006;
    }

    // pub fn remake_frame(&self, frame: &mut [u8], window: &Window) {
    //     window.set_title(&format!("sillyular pawtomata - rule  {:?}", self.rule));
        
    //     let mut universe = Universe::new_center(self.rule);
    //     for y in 0..PIX_SIZE.1 as usize {
    //         let alive = hue_to_rgb(y as f32 / PIX_SIZE.1 as f32);
    //         let dead = [0x20, 0x20, 0x20, 0xff];

    //         let cells = universe.cells;
    //         for x in 0..WORLD_WIDTH {
    //             let i = (x + y * WORLD_WIDTH) * 4; 
    //             let color = match cells[x] {
    //                 true => alive,
    //                 _    => dead,
    //             };
    //             frame[i..i+4].copy_from_slice(&color); 
    //         }
    //         universe.step();
    //     }
    // }

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