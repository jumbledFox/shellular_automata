use rand::{RngExt, rng, rngs::ThreadRng};
use winit::{keyboard::{Key, NamedKey}, window::Window};

use crate::{PIX_SIZE, simulator::{Simulator, WORLD_SIZE}};
use universe::{Universe, InitMode};

pub mod universe;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Shuffle {
    None,
    Random(usize),
    Count(usize, i32),
}

pub struct CustomWolfram {
    universe: Universe,
    stepping: bool,
    rule: u32,
    init_mode: InitMode,
    hue: f32,
    rand: ThreadRng,
    shuffle: Shuffle,
}

impl Simulator for CustomWolfram {
    fn keypress(&mut self, key: Key, frame: &mut [u8], window: &Window) {
        match key {
            Key::Named(NamedKey::Space)  => self.toggle_stepping(),
            Key::Named(NamedKey::ArrowLeft)  => self.shimmy(-1, frame, window),
            Key::Named(NamedKey::ArrowRight) => self.shimmy( 1, frame, window),
            Key::Character(v) if v.to_lowercase() == "q" => self.flip(frame, window),
            Key::Character(v) if v.to_lowercase() == "e" => self.toggle_init_mode(frame, window),
            Key::Character(v) if v.to_lowercase() == "r" => self.randomize(frame, window),
            _ => ()
        };
    }

    fn update(&mut self, frame: &mut [u8], window: &Window) {
        if self.stepping {
            self.step_simulation(frame, window);
        }
    }
}

impl CustomWolfram {
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
            shuffle: Shuffle::Random(0)
        };
        s.remake_universe(frame, window);
        s
    }

    pub fn set_rule(&mut self, rule: u32, frame: &mut [u8], window: &Window) {
        self.rule = rule;
        self.remake_universe(frame, window);
    }

    pub fn shimmy(&mut self, amount: i32, frame: &mut [u8], window: &Window) {
        self.rule = self.rule.wrapping_add_signed(amount);
        self.remake_universe(frame, window);
    }

    pub fn flip(&mut self, frame: &mut [u8], window: &Window) {
        self.rule = !self.rule;
        self.remake_universe(frame, window);
    }

    pub fn randomize(&mut self, frame: &mut [u8], window: &Window) {
        self.rule = self.rand.random_range(0..u32::MAX);
        self.remake_universe(frame, window);
    }

    pub fn toggle_init_mode(&mut self, frame: &mut [u8], window: &Window) {
        self.init_mode = match self.init_mode {
            InitMode::Center => InitMode::Random,
            InitMode::Random => InitMode::Center,
        };
        self.remake_universe(frame, window);
    }

    pub fn toggle_stepping(&mut self) {
        self.stepping = !self.stepping;
    }

    fn update_title(&self, window: &Window) {
        let init_mode = match self.init_mode {
            InitMode::Center => "",
            InitMode::Random => " - Random"
        };
        window.set_title(&format!("sillyular pawtomata - rule  {:?} - generation {:?}{init_mode}", self.rule, self.universe.generation()));
    }

    fn remake_universe(&mut self, frame: &mut [u8], window: &Window) {
        self.hue = ((self.rule%295).pow(2) as f32 * 0.09).rem_euclid(1.0); // psuedo-random number :trollface:
        self.universe = Universe::new(self.rule, self.init_mode, &mut self.rand);
        // when shuffling, we wanna make the transition seamless, so don't need to reset
        if self.shuffle == Shuffle::None {
            // clear the framebuffer to black
            for pix in frame.chunks_mut(4) {
                pix.clone_from_slice(&[0x00, 0x00, 0x00, 0xff]);
            }
            // step the universe forward so we can see a full screen
            for i in 0..PIX_SIZE.1 as usize {
                self.draw_step(i, frame);
                self.universe.step();
            }
        }

        self.update_title(window);
    }

    fn step_simulation(&mut self, frame: &mut [u8], window: &Window) {
        frame.copy_within(WORLD_SIZE.0 * 4.., 0);
        self.draw_step(PIX_SIZE.1 as usize - 1, frame);
        self.update_title(window);
        self.universe.step();

        let generation = self.universe.generation();
        let mut rand = || -> usize {
            self.rand.random_range(40..130) as usize
        };
        match self.shuffle {
            Shuffle::Random(g) if generation > g => {
                self.shuffle = Shuffle::Random(rand());
                self.randomize(frame, window);
            }
            Shuffle::Count(g, c) if generation > g => {
                self.shuffle = Shuffle::Count(rand(), c);
                self.set_rule(self.rule.wrapping_add_signed(c), frame, window);
            }
            _ => {}
        }
    }

    fn draw_step(&mut self, y: usize, frame: &mut [u8]) {
        let alive = hue_to_rgb(self.hue.rem_euclid(1.0));
        for x in 0..WORLD_SIZE.0 {
            let i = (x + y * WORLD_SIZE.0) * 4;
            let color = match self.universe.cells()[x] {
                true => alive,
                _    => [0x00, 0x00, 0x00, 0xff],
            };
            frame[i..i+4].copy_from_slice(&color);
        }
        self.hue += match self.shuffle {
            Shuffle::None => 0.0006,
            Shuffle::Random(g) |
            Shuffle::Count(g, _) => { (120-g) as f32 / 100.0 * 0.006 }
        };
    }
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