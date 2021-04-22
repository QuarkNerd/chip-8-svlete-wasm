use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;

const COLS: usize = 64;
const COLS_U8: u8 = 64;
const ROWS: usize = 32;
const ROWS_U8: u8 = 32;

#[wasm_bindgen]
pub struct Display {
    pixels: [u8; COLS*ROWS],
    y_wrap: bool,
}

impl Display {
    pub fn new() -> Display {
        let mut disp = Display {
            pixels: [0; COLS*ROWS],
            y_wrap: false
        };

        for n in 0..COLS*ROWS {
            disp.pixels[n] = rand::random::<u8>()%2;
        };

        disp
    }

    pub fn set_pixel(&mut self, mut x: u8, mut y: u8) -> bool {
        x = x % COLS_U8;
        if y > ROWS_U8 {
            if !self.y_wrap { return false };
            y = y % ROWS_U8;
        }; 

        let pixel_num = x as usize + y as usize * COLS;

        self.pixels[pixel_num] ^= 1;
        self.pixels[pixel_num] == 0
    }

    pub fn pixels(&self) -> Uint8Array {
        unsafe { Uint8Array::view(&self.pixels) }
    }
}
