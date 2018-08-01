use std::collections::HashSet;
use {Byte};

pub struct Keyboard {
    pressed: HashSet<Byte>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            pressed: HashSet::with_capacity(16)
        }
    }

    pub fn set_pressed(&mut self, key: Byte) {
        self.pressed.insert(key);
    }

    pub fn set_released(&mut self, key: Byte) {
        self.pressed.remove(&key);
    }

    pub fn is_pressed(&self, key: Byte) -> bool {
        self.pressed.contains(&key)
    }

    pub fn is_released(&self, key: Byte) -> bool {
        !self.is_pressed(key)
    }
}