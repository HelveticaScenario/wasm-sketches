extern crate rand;
extern crate web_sys;
mod pico;
mod sketch;
mod sketches;

use pico::*;
use rand::prelude::*;
use sketch::*;
use sketches::*;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

// unsafe impl Sync for SketchContainer<T> {}
static ACTIVE_SKETCH: SketchContainer = SketchContainer(RefCell::new(None));

#[wasm_bindgen]
pub fn init(index: usize) {
    set_panic_hook();
    {
        let mut palette = PALETTE.0.borrow_mut();
        for i in 0..48 {
            palette[i] = DEFAULT_COLORS[i];
        }
    }
    {
        let mut palette_swap = PALETTE_SWAP.0.borrow_mut();
        for i in 0..NUM_COLORS {
            palette_swap[i] = i as u8;
        }
    }
    {
        let mut state = STATE.0.borrow_mut();
        state.transparency[0] = true;
    }

    let constructor_count = CONSTRUCTORS.0.len();
    if index < constructor_count {
        let mut active = ACTIVE_SKETCH.0.borrow_mut();
        (*active) = Some(CONSTRUCTORS.0[index]());
    }
}
fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn update(delta: f32) {
    // let mut sketch = ACTIVE_SKETCH.0.borrow_mut();
    // if let Some(sketch) = sketch {
    //     sketch.update();
    // }

    let (old, new) = {
        let mut state = STATE.0.borrow_mut();
        let oldTime = (*state).time;
        let newTime = oldTime + delta.round() as u32;
        (*state).time = newTime;
        let old = oldTime;
        let new = newTime;
        (old, new)
    };
    let active = ACTIVE_SKETCH.0.borrow();

    if let Some(sketch) = active.as_ref() {
        (*sketch).borrow_mut().update(old, new);
    }
    // rect_fill(1, 10, 126, -127, 12);
    // for _ in 0..1 {
    //     let x0: u32 = rand::random();
    //     let x0 = x0 % (WIDTH as u32);
    //     let x1: u32 = rand::random();
    //     let x1 = x1 % (WIDTH as u32);
    //     let y0: u32 = rand::random();
    //     let y0 = y0 % (HEIGHT as u32);
    //     let y1: u32 = rand::random();
    //     let y1 = y1 % (HEIGHT as u32);
    //     let c: u32 = rand::random();
    //     let c = c % 16;
    //     rect_fill(x0 as i32, y0 as i32, x1 as i32, y1 as i32, c as i32);
    // }

    // if let Some(Point { x: new_x, y: new_y }) = mouse_pos {
    //     let mut sketch_state = SKETCH1.0.borrow_mut();
    //     if let Some(Point {
    //         x: last_x,
    //         y: last_y,
    //     }) = &sketch_state.last_mouse
    //     {
    //         line(*last_x, *last_y, new_x, new_y, 12);
    //     } else {
    //         pset(new_x, new_y, 12);
    //     }
    //     (*sketch_state).last_mouse = Some(Point { x: new_x, y: new_y });
    // } else {
    //     let mut sketch_state = SKETCH1.0.borrow_mut();
    //     if let Some(Point { x, y }) = sketch_state.last_mouse {
    //         (*sketch_state).last_mouse = None;
    //     }
    // }
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
pub fn set_mouse_pos(x: i32, y: i32) {
    let mut state = STATE.0.borrow_mut();
    (*state).mouse_pos = Some(Point { x: x, y: y });
}

#[wasm_bindgen]
pub fn set_mouse_end() {
    let mut state = STATE.0.borrow_mut();
    (*state).mouse_pos = None;
}

#[wasm_bindgen]
pub fn screen_ptr() -> *mut [u8; MAX_SCREEN_SIZE] {
    SCREEN.0.as_ptr()
}

#[wasm_bindgen]
pub fn palette_ptr() -> *mut [u8; NUM_COLORS * 3] {
    PALETTE.0.as_ptr()
}

#[wasm_bindgen]
pub fn palette_swap_ptr() -> *mut [u8; NUM_COLORS] {
    PALETTE_SWAP.0.as_ptr()
}

#[wasm_bindgen]
pub fn palette_swap_size() -> usize {
    NUM_COLORS
}

#[wasm_bindgen]
pub fn screen_size() -> usize {
    WIDTH() * HEIGHT()
}

#[wasm_bindgen]
pub fn palette_size() -> usize {
    NUM_COLORS * 4
}

#[wasm_bindgen]
pub fn screen_width() -> usize {
    WIDTH()
}

#[wasm_bindgen]
pub fn screen_height() -> usize {
    HEIGHT()
}
