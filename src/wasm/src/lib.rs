use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;
use display::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Emulator {
    display: Display
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Emulator {
        Emulator {
            display: Display::new()
        }
    }

    pub fn get_display(&self) -> Uint8Array {
        self.display.pixels()
    }
}

mod display;