use winit::{keyboard::Key, window::Window};

use crate::PIX_SIZE;

pub mod custom_wolfram;
pub mod conway;

const WORLD_SIZE: (usize, usize) = (PIX_SIZE.0 as usize, PIX_SIZE.1 as usize);

pub trait Simulator {
    fn keypress(&mut self, key: Key, frame: &mut [u8], window: &Window);
    fn update(&mut self, frame: &mut [u8], window: &Window);
}