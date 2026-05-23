use winit::{keyboard::Key, window::Window};

use crate::{PIX_SIZE, input::Input};

// pub mod custom_wolfram;
pub mod conway;

const WORLD_SIZE: (usize, usize) = (PIX_SIZE.0 as usize, PIX_SIZE.1 as usize);

pub trait Simulator {
    fn update(&mut self, input: &Input, frame: &mut [u8], window: &Window);
}

// TODO: Make a class that handles all of this
// such as different universe types (1d scroll, 2d) and giving a framework for
// easily drawing the board. different scroll types for 1d ones (e.g. from bottom, filled, scrolling)