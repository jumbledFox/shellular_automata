
// TODO: Idea for an 'ambient mode', where it draws rainbow automatically
// simulating a cursors natural movement through a selection of randomised
// patterns. cursor has velocity and wiggles a bit and slows down and stuff

use std::f32::consts::PI;

use rand::{RngExt, random_range, rngs::ThreadRng};

use crate::{PIX_SIZE, simulator::{WORLD_SIZE, conway::WORLD_AREA}};

enum BrushMode {
    Square,
    // Circle,
}

enum State {
    Waiting(f32),
    Lines {
        i:   usize,
        max: usize,
        dist: f32,
        rot:      f32,
        max_dist: f32,
    },
    Sine {
        origin: (f32, f32),
        rot: f32,
        len: f32,
        frequency: f32,
        amplitude: f32,
        max_len:   f32,
    }
    // TODO: Other shapes, circles, stars, hearts
    // also multiple cursors drawing sine waves and opposing patterns in another colour
}

struct Palette {
    c: f32,
    p: usize,
    palettes: Vec<Vec<u32>>,
}

impl Palette {
    pub fn new() -> Self {
        let mut palettes = Vec::new();
        palettes.push(vec![ // rainbow
            0xff0000, 0xff8700, 0xffd300, 0xdeff0a, 0xa1ff0a, 0x0aff99, 0x0aefff, 0x147df5, 0x580aff, 0xbe0aff,
        ]);
        palettes.push(vec![ // two
            0xfef430, 0xffffff, 0x9b58d0, 0x888888,
        ]);
        palettes.push(vec![ // pastel
            0xeae4e9, 0xfff1e6, 0xfde2e4, 0xfad2e1, 0xe2ece9,
            0xbee1e6, 0xf0efeb, 0xdfe7fd, 0xcddafd,
        ]);
        palettes.push(vec![ // purp
            0xff2222, 0x2222ff,
        ]);
        palettes.push(vec![ // https://lospec.com/palette-list/curiosities
            0x00b9be, 0xff6973, 0xffb0a3, 0xffeecc,
        ]);
        palettes.push(vec![ // https://lospec.com/palette-list/citrink
            0x52c33f, 0xb2d942, 0xfcf660, 0xffffff,
        ]);
        palettes.push(vec![ // https://lospec.com/palette-list/nostalgia
            0xd0d058, 0xa0a840, 0x708028, 0x405010,
        ]);
        palettes.push(vec![ // https://lospec.com/palette-list/blk-aqu4
            0x9ff4e5, 0x00b9be, 0x005f8c
        ]);
        palettes.push(vec![ // https://lospec.com/palette-list/vividmemory8
            0x381631, 0xe21c61, 0xe26159, 0xfea85f, 0xd8dcb4, 0x5eb6ad, 0x1b958d, 0x105390
        ]);

        Self { c: 0.0, p: 0, palettes }
    }

    pub fn init(&mut self, rand: &mut ThreadRng) {
        self.p = rand.random_range(0..self.palettes.len());
        self.c = 0.0;
    }

    pub fn color(&self) -> [u8; 3] {
        let c = &self.palettes[self.p];

        let i1 = self.c.floor() as usize;
        let i2 = (i1 + 1).rem_euclid(c.len());

        let unpackrgb = |d: u32| -> [f32; 3] {
            [
                ((d >> 16) & 0xff) as f32,
                ((d >> 8)  & 0xff) as f32,
                ((d)       & 0xff) as f32,
            ]
        };
        let (c1, c2) = (unpackrgb(c[i1]), unpackrgb(c[i2]));

        let t = self.c.fract();
        let (r, g, b) = (
            ((1.0 - t) * c1[0].powi(2) + t * c2[0].powi(2)).sqrt(),
            ((1.0 - t) * c1[1].powi(2) + t * c2[1].powi(2)).sqrt(),
            ((1.0 - t) * c1[2].powi(2) + t * c2[2].powi(2)).sqrt(),
        );

        [r as u8, g as u8, b as u8]
    }

    pub fn step_c(&mut self) {
        let c = &self.palettes[self.p];
        let len = c.len() as f32;
        self.c = (self.c + (6.0 / 60.0)).rem_euclid(len);
    }
}

pub struct Artist {
    brush_pos: (f32, f32),
    brush_mode: BrushMode,
    state: State,
    palette: Palette,
}

impl Artist {
    pub fn new() -> Self {
        Self {
            brush_pos: (0.0, 0.0),
            brush_mode: BrushMode::Square,
            state: State::Waiting(0.0),
            palette: Palette::new()
        }
    }
    pub fn init_random(&mut self, rand: &mut ThreadRng) {
        let rand_point = |rand: &mut ThreadRng| -> (f32, f32) {(
            rand.random_range(0.0..(PIX_SIZE.0 as f32)),
            rand.random_range(0.0..(PIX_SIZE.1 as f32))
        )};
        let rand_rot = |rand: &mut ThreadRng| -> f32 {
            rand.random_range(-PI..PI)
        };

        self.brush_pos = rand_point(rand);

        self.state = match rand.random_range(0..2) {
            0 => State::Lines {
                i: 0,
                max: rand.random_range(7..19),
                dist: 0.0,
                rot: rand_rot(rand),
                max_dist: rand.random_range(30.0..200.0),
            },
            _ => State::Sine {
                origin: rand_point(rand),
                rot: rand_rot(rand),
                len: 0.0,
                max_len: random_range(800.0..1400.0),
                frequency: rand.random_range(0.005..0.06),
                amplitude: rand.random_range(50.0..160.0),
            }
        };

        self.palette.init(rand);
    }

    pub fn update(&mut self, cells: &mut [Option<[u8; 3]>; WORLD_AREA], rand: &mut ThreadRng) {
        // updating
        let mut drawing = true;
        let mut wait = false;
        match &mut self.state {
            State::Waiting(t) => {
                drawing = false;
                *t -= 1.0 / 60.0;
                if *t <= 0.0 {
                    self.init_random(rand);
                }
            }
            State::Lines { i, max, dist, rot, max_dist } => {
                let s = 4.0;
                self.brush_pos.0 = (self.brush_pos.0 + s*rot.sin()).rem_euclid(PIX_SIZE.0 as f32);
                self.brush_pos.1 = (self.brush_pos.1 + s*rot.cos()).rem_euclid(PIX_SIZE.1 as f32);
                *dist += s;
                if *dist >= *max_dist {
                    *dist = 0.0;
                    *rot += rand.random_range(PI/3.0..=PI/2.0) * if rand.random_bool(0.5) { 1.0 } else { -1.0 };
                    *i += 1;
                    if i > max {
                        wait = true;
                    }
                }
            }
            State::Sine { origin, rot, len, frequency, amplitude, max_len } => {
                let s = 3.0;
                *len += s;
                let x = origin.0 + *len;
                let y = origin.1 + (*len * *frequency).sin() * (*amplitude);
                // rotate x and y
                self.brush_pos.0 = (x * rot.cos() - y * rot.sin()).rem_euclid(PIX_SIZE.0 as f32);
                self.brush_pos.1 = (x * rot.sin() + y * rot.cos()).rem_euclid(PIX_SIZE.1 as f32);
                if *len >= *max_len {
                    wait = true;
                }
            }
        }
        if wait {
            // self.state = State::Waiting(rand.random_range(5.0..10.0));
            self.state = State::Waiting(0.0);
        }

        // drawing
        if !drawing {
            return;
        }
        self.palette.step_c();
        let pix = (self.brush_pos.0 as usize, self.brush_pos.1 as usize);
        let r = 5;
        for y_offset in -r..=r {
            for x_offset in -r..=r {
                let x = (pix.0 as isize + x_offset).rem_euclid(WORLD_SIZE.0 as isize) as usize;
                let y = (pix.1 as isize + y_offset).rem_euclid(WORLD_SIZE.1 as isize) as usize;
                cells[x + y*WORLD_SIZE.0] = match rand.random_bool(0.8) {
                    true => None,
                    _ => Some(self.palette.color()),
                };
            }
        }
    }
}