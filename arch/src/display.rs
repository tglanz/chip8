use {Byte};

const RESOLUTION_WIDTH: usize = 64;
const RESOLUTION_HEIGHT: usize = 32;
const PIXEL_COUNT: usize = RESOLUTION_HEIGHT * RESOLUTION_WIDTH;

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


        for (rowdex, row_pixels) in pixel_matrix.iter().enumerate() {
            let row_offset = rowdex * RESOLUTION_WIDTH;
            for (coldex, pixel) in row_pixels.iter().enumerate() {
                if *pixel {
                    let index = row_offset + coldex;
                    did_flip |= self.pixels[index];
                    self.pixels[index] ^= *pixel;
                }
            }
        }

        did_flip
    }
}

fn pixels_from_byte(byte: &Byte) -> [Pixel; 8] {
    [
        byte & 1 == 1,
        byte & 2 == 2,
        byte & 4 == 4,
        byte & 8 == 8,
        byte & 16 == 16,
        byte & 32 == 32,
        byte & 64 == 64,
        byte & 128 == 128,
    ]
}

fn pixels_matrix_from_sprite(sprite: &Sprite) -> Vec<[Pixel; 8]> {
    sprite.iter().map(pixels_from_byte).collect()
}