extern crate rand;
mod pico;

use pico::*;
use rand::prelude::*;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() {
    set_panic_hook();
    let mut palette = PALETTE.0.borrow_mut();

    for i in 0..48 {
        palette[i] = DEFAULT_COLORS[i];
    }
}
fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub struct Sketch1 {
    last_mouse: Option<Point>,
}

pub struct Sketch1Container(pub RefCell<Sketch1>);
unsafe impl Sync for Sketch1Container {}

static SKETCH1: Sketch1Container = Sketch1Container(RefCell::new(Sketch1 { last_mouse: None }));

#[wasm_bindgen]
pub fn update(delta: f32) {
    let (old, new) = {
        let mut state = STATE.0.borrow_mut();
        let oldTime = (*state).time;
        let newTime = oldTime + delta.floor() as u32;
        (*state).time = newTime;
        let old = oldTime;
        let new = newTime;
        (old, new)
    };
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
    cls(0);
    let mouse_pos = get_mouse_pos();
    let center_x = (WIDTH / 2) as i32;
    let center_y = (HEIGHT / 2) as i32;

    // let t = (new as f32) / 5000.0;
    // let mouse_pos = Some(Point {
    //     x: ((t.sin() * center_x as f32) as i32) + center_x,
    //     y: ((t.cos() * center_y as f32) as i32) + center_y,
    // });

    if let Some(Point { x, y }) = mouse_pos {
        let (x0, y0, x1, y1) = rect_swap(center_x, center_y, x, y);
        let diff_x = (x - center_x).abs();
        let diff_y = (y - center_y).abs();
        let min_diff = (cmp::min(diff_x, diff_y) / 2) + 1;
        for i in 0..min_diff {
            rect(x0 + i, y0 + i, x1 - i, y1 - i, (i % 15) + 1);
        }
        // rect((WIDTH / 2) as i32, (HEIGHT / 2) as i32, x, y, 12);
    }
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
    if x == -1 && y == -1 {
        (*state).mouse_pos = None;
    } else {
        (*state).mouse_pos = Some(Point { x: x, y: y });
    }
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
