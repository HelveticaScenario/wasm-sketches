// #![allow(dead_code)]
extern crate rand;
extern crate web_sys;
extern crate ncollide2d;
extern crate nalgebra;
extern crate euclid;
mod font;
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
use wasm_bindgen::prelude::wasm_bindgen as bindgen;
use wasm_bindgen::prelude::JsValue;
use web_sys::console::log_1;
// use wasm_bindgen::wasm_bindgen_macro::wasm_bindgen as bindgen:

// unsafe impl Sync for SketchContainer<T> {}
static ACTIVE_SKETCH: SketchContainer = SketchContainer(RefCell::new(None));

#[bindgen]
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
        (state.transparency).fill(false);
        state.transparency[0] = true;
        state.time = 0.0;
        for i in 0..state.pointer_pos.len() {
            state.pointer_pos[i] = None;
        }
        state.offset.x = 0;
        state.offset.y = 0;
    }
    set_dimensions(128, 128);
    set_target(0);
    let constructor_count = SKETCHES.len();
    if index < constructor_count {
        let mut active = ACTIVE_SKETCH.0.borrow_mut();
        (*active) = Some((SKETCHES[index].constructor)());
    }
}
fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[bindgen]
pub fn update(delta: f32) {
    let (old, new) = {
        let mut state = STATE.0.borrow_mut();
        let oldTime = (*state).time;
        let newTime = oldTime + delta;
        (*state).time = newTime;
        let old = oldTime;
        let new = newTime;
        (old, new)
    };
    let active = ACTIVE_SKETCH.0.borrow();

    if let Some(sketch) = active.as_ref() {
        (*sketch).borrow_mut().update(old, new);
    }
    for i in STATE.0.borrow_mut().mouse_buttons.iter_mut() {
        *i = match *i {
            MouseButtonState::UpThisFrame => MouseButtonState::Up,
            MouseButtonState::DownThisFrame => MouseButtonState::Down,
            MouseButtonState::Up => MouseButtonState::Up,
            MouseButtonState::Down => MouseButtonState::Down,
        };
    }
    {
        let mut state = STATE.0.borrow_mut();
        for i in 0..POINTER_COUNT {
            state.last_pointer_pos[i] = state.pointer_pos[i];
            state.last_pointer_state[i] = state.pointer_state[i];
        }
        state.pointer_pos_changed = false;
        state.pointer_state_changed = false;
    }

    {
        let mut state = STATE.0.borrow_mut();
        state.scroll_delta = 0.0;
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

#[bindgen]
pub fn set_pointer_pos(idx: usize, x: i32, y: i32) {
    let mut state = STATE.0.borrow_mut();
    if idx < (*state).pointer_pos.len() {
        (*state).pointer_pos[idx] = Some(Point { x: x, y: y });
        state.pointer_pos_changed = true;
    }
}

#[bindgen]
pub fn set_pointer_end(idx: usize) {
    let mut state = STATE.0.borrow_mut();
    if idx < (*state).pointer_pos.len() {
        (*state).pointer_pos[idx] = None;
        state.pointer_pos_changed = true;
    }
}

#[bindgen]
pub fn set_pointer_state(idx: usize, btns: u32) {
    let mut state = STATE.0.borrow_mut();
    if idx < (*state).pointer_state.len() {
        (*state).pointer_state[idx] = btns;
        state.pointer_state_changed = true;
    }
}

#[bindgen]
pub fn set_mouse_button(btn_num: u8, down: bool) {
    let mut state = STATE.0.borrow_mut();
    if (btn_num as usize) < state.mouse_buttons.len() {
        (*state).mouse_buttons[btn_num as usize] = if down {
            MouseButtonState::DownThisFrame
        } else {
            MouseButtonState::UpThisFrame
        };
    }
}

#[bindgen]
pub fn screen_ptr() -> *mut [u8; MAX_SCREEN_SIZE] {
    SCREEN.0.as_ptr()
}

#[bindgen]
pub fn palette_ptr() -> *mut [u8; NUM_COLORS * 3] {
    PALETTE.0.as_ptr()
}

#[bindgen]
pub fn palette_swap_ptr() -> *mut [u8; NUM_COLORS] {
    PALETTE_SWAP.0.as_ptr()
}

#[bindgen]
pub fn palette_swap_size() -> usize {
    NUM_COLORS
}

#[bindgen]
pub fn screen_size() -> usize {
    WIDTH() * HEIGHT()
}

#[bindgen]
pub fn palette_size() -> usize {
    NUM_COLORS * 3
}

#[bindgen]
pub fn screen_width() -> usize {
    WIDTH()
}

#[bindgen]
pub fn screen_height() -> usize {
    HEIGHT()
}

#[bindgen]
pub fn get_sketch_count() -> usize {
    SKETCHES.len()
}

#[bindgen]
pub fn get_sketch_url(i: usize) -> Option<String> {
    if i < SKETCHES.len() {
        Some(SKETCHES[i].url.into())
    } else {
        None
    }
}

#[bindgen]
pub fn get_sketch_name(i: usize) -> Option<String> {
    if i < SKETCHES.len() {
        Some(SKETCHES[i].name.into())
    } else {
        None
    }
}

#[bindgen]
pub fn get_sketch_is_mobile(i: usize) -> Option<bool> {
    if i < SKETCHES.len() {
        Some(SKETCHES[i].mobile)
    } else {
        None
    }
}

#[bindgen]
pub fn get_sketch_is_desktop(i: usize) -> Option<bool> {
    if i < SKETCHES.len() {
        Some(SKETCHES[i].desktop)
    } else {
        None
    }
}

#[bindgen]
pub fn get_sketch_is_public(i: usize) -> Option<bool> {
    if i < SKETCHES.len() {
        Some(SKETCHES[i].public)
    } else {
        None
    }
}

#[bindgen]
pub fn get_memory() -> JsValue {
    wasm_bindgen::memory()
}

#[bindgen]
pub fn set_wheel(delta: f64) {
    let mut state = STATE.0.borrow_mut();
    (*state).scroll += delta;
    (*state).scroll_delta = delta;
}
