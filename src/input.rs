use std::collections::HashSet;

use winit::{event::{ElementState, KeyEvent, MouseButton}, keyboard::{Key, SmolStr}};

pub struct Input {
    keys_pressed:  HashSet<Key>,
    keys_held:     HashSet<Key>,
    keys_released: HashSet<Key>,
    mouse_pressed:  HashSet<MouseButton>,
    mouse_held:     HashSet<MouseButton>,
    mouse_released: HashSet<MouseButton>,
    mouse_pos: Option<(usize, usize)>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys_pressed:  HashSet::with_capacity(16),
            keys_held:     HashSet::with_capacity(16),
            keys_released: HashSet::with_capacity(16),
            mouse_pressed:  HashSet::with_capacity(8),
            mouse_held:     HashSet::with_capacity(8),
            mouse_released: HashSet::with_capacity(8),
            mouse_pos: None,
        }
    }

    fn key_ignore_case(set: &HashSet<Key>, key: Key) -> bool {
        if let Key::Character(c) = key {
            set.contains(&Key::Character(SmolStr::new(c.to_lowercase()))) ||
            set.contains(&Key::Character(SmolStr::new(c.to_uppercase())))
        } else {
            set.contains(&key)
        }
    } 

    pub fn key_pressed(&self, key: Key) -> bool {
        Self::key_ignore_case(&self.keys_pressed, key)
    }
    pub fn key_held(&self, key: Key) -> bool {
        Self::key_ignore_case(&self.keys_held, key)
    }
    pub fn key_released(&self, key: Key) -> bool {
        Self::key_ignore_case(&self.keys_released, key)
    }

    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_pressed.contains(&button)
    }
    pub fn mouse_held(&self, button: MouseButton) -> bool {
        self.mouse_held.contains(&button)
    }
    pub fn mouse_released(&self, button: MouseButton) -> bool {
        self.mouse_released.contains(&button)
    }
    pub fn mouse_pos(&self) -> Option<(usize, usize)> {
        self.mouse_pos
    }

    pub fn lost_focus(&mut self) {
        self.keys_held.clear();
        self.mouse_held.clear();
    }

    pub fn begin(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.mouse_pressed.clear();
        self.mouse_released.clear();
        if self.mouse_pos == None {
            self.lost_focus();
        }
    }

    pub fn keypress(&mut self, event: KeyEvent) {
        if event.repeat {
            return;
        }
        if event.state == ElementState::Pressed {
            self.keys_pressed.insert(event.logical_key.clone());
            self.keys_held   .insert(event.logical_key);
        } else {
            self.keys_held    .remove(&event.logical_key);
            self.keys_released.insert(event.logical_key);
        }
    }

    pub fn mouse_input(&mut self, pix: (usize, usize), button: MouseButton, state: ElementState) {
        self.mouse_pos = Some(pix);
        if state == ElementState::Pressed {
            self.mouse_pressed.insert(button);
            self.mouse_held   .insert(button);
        } else {
            self.mouse_held    .remove(&button);
            self.mouse_released.insert(button);
        }
    }

    pub fn mouse_move(&mut self, pix: (usize, usize)) {
        self.mouse_pos = Some(pix);
    }
    pub fn mouse_gone(&mut self) {
        self.mouse_pos = None;
    }
}