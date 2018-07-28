use {Byte};

pub struct Keyboard {
    pressed: Vec<Byte>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            pressed: Vec::new()
        }
    }

    pub fn is_pressed(&self, key: Byte) -> bool {

        // TODO@TalG : find

        for v in &self.pressed {
            if *v == key {
                return true;
            }
        }

        return false;
    }
}