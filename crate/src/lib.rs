extern crate rand;
mod pico;

use pico::*;
use rand::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() {
    set_panic_hook();
    let mut palette = PALETTE.0.borrow_mut();
    let default_colors: [u8; 16 * 3] = [
        0, 0, 0, 29, 43, 83, 126, 37, 83, 0, 135, 81, 171, 82, 54, 95, 87, 79, 194, 195, 199, 255,
        241, 232, 255, 0, 77, 255, 164, 0, 255, 236, 39, 0, 228, 54, 41, 173, 255, 131, 118, 156,
        255, 119, 168, 255, 204, 170,
    ];
    for i in 0..48 {
        palette[i] = default_colors[i];
    }
}
fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub struct Cc(pub RefCell<i32>);
unsafe impl Sync for Cc {}
static CC: Cc = Cc(RefCell::new(0));

#[wasm_bindgen]
pub fn update(delta: f32) {
    let (old, new) = {
        let mut state = STATE.0.borrow_mut();
        let oldTime = (*state).time;
        let newTime = oldTime + delta.floor() as u32;
        (*state).time = newTime;
        let old = oldTime / 3000;
        let new = newTime / 3000;
        (old, new)
    };
    // rect_fill(1, 10, 126, -127, 12);
    for _ in 0..1 {
        let x0: u32 = rand::random();
        let x0 = x0 % (WIDTH as u32);
        let x1: u32 = rand::random();
        let x1 = x1 % (WIDTH as u32);
        let y0: u32 = rand::random();
        let y0 = y0 % (HEIGHT as u32);
        let y1: u32 = rand::random();
        let y1 = y1 % (HEIGHT as u32);
        let c: u32 = rand::random();
        let c = c % 16;
        rect_fill(x0 as i32, y0 as i32, x1 as i32, y1 as i32, c as i32);
    }
    // camera_set(10, 10);
    // let mut state = STATE.0.borrow_mut();
    // (*state).time += delta.floor() as u32;
    // let o = (*state).time / 16;
    // // (*state).time = (*state).time + delta;
    // let mut screen = SCREEN.0.borrow_mut();
    // for y in 0..HEIGHT {
    //     for x in 0..WIDTH {
    //         let i = y * WIDTH + x;
    //         screen[i] = ((x + y + o as usize) % 16) as u8;
    //         // let num: u8 = rand::random();
    //         // screen[i] = num % 16;
    //         // screen[i] = 1;
    //     }
    // }
}

#[wasm_bindgen]
pub fn screen_ptr() -> *mut [u8; PIXELS] {
    SCREEN.0.as_ptr()
}

#[wasm_bindgen]
pub fn palette_ptr() -> *mut [u8; NUM_COLORS * 3] {
    PALETTE.0.as_ptr()
}

#[wasm_bindgen]
pub fn screen_size() -> usize {
    PIXELS
}

#[wasm_bindgen]
pub fn palette_size() -> usize {
    NUM_COLORS * 4
}

#[wasm_bindgen]
pub fn screen_width() -> usize {
    WIDTH
}

#[wasm_bindgen]
pub fn screen_height() -> usize {
    HEIGHT
}
