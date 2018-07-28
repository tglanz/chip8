use {Byte};

pub const RESOLUTION_WIDTH: usize = 64;
pub const RESOLUTION_HEIGHT: usize = 32;
pub const PIXEL_COUNT: usize = RESOLUTION_HEIGHT * RESOLUTION_WIDTH;

type Sprite = Vec<Byte>;
type Pixel = bool;

pub struct Display {
    pixels: [Pixel; PIXEL_COUNT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            pixels: [false; PIXEL_COUNT],
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.pixels.iter_mut() {
            *pixel = false;
        }
    }

    pub fn set_sprite(&mut self, dx: usize, dy: usize, sprite: &Vec<Byte>) -> bool {
        let mut did_flip = false;

        let pixel_matrix = pixels_matrix_from_sprite(sprite);

        for rowdex in 0..sprite.len() {
            let byte = sprite[rowdex];
            let pixels = pixels_from_byte(&byte);

            for coldex in 0..pixels.len() {
                let offseted_x = dx + coldex;
                let wrapped_row = dy + rowdex + (offseted_x / RESOLUTION_WIDTH);
                let pixel_index = wrapped_row * RESOLUTION_WIDTH + (dx + coldex) % RESOLUTION_WIDTH;

                let sprite_pixel = pixels[coldex];
                did_flip |= (sprite_pixel && self.pixels[pixel_index]);
                self.pixels[pixel_index] ^= sprite_pixel;
            }
        }

        did_flip
    }

    pub fn traverse<F>(&self, mut f: F)
        where F: FnMut(usize, usize, Pixel) {
            for rowdex in 0..RESOLUTION_HEIGHT {
                let offset = rowdex * RESOLUTION_WIDTH;
                for coldex in 0..RESOLUTION_WIDTH {
                    let pixel = self.pixels[offset + coldex];
                    f(coldex, rowdex, pixel);
                }
            }
        }
}

fn pixels_from_byte(byte: &Byte) -> [Pixel; 8] {
    [
        byte & 128 == 128,
        byte & 64 == 64,
        byte & 32 == 32,
        byte & 16 == 16,
        byte & 8 == 8,
        byte & 4 == 4,
        byte & 2 == 2,
        byte & 1 == 1,
    ]
}

fn pixels_matrix_from_sprite(sprite: &Sprite) -> Vec<[Pixel; 8]> {
    sprite.iter().map(pixels_from_byte).collect()
}