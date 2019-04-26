use crate::pico::*;
use crate::sketch::*;
use std::cell::RefCell;
use std::cmp;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

pub struct Face {}
fn as_u16_le(array: &[u8; 2]) -> u16 {
    ((array[0] as u16) << 0) + ((array[1] as u16) << 8)
}

impl Sketch for Face {
    fn new() -> Face {
        set_dimensions(64, 64);
        load_spritesheet(include_bytes!("face.pico"));

        // // let height = as_u16_le(&bytes[2..4]);
        // log_1(&JsValue::from(format!("{:?} {:?} {:?} {:?} {:?}", spritesheet.len(), bytes.len(), width, height, palette.len())));
        cls(7);
        copy_screen(3, 0);
        prnt("Hello", 1, 1, 1, 1, 7);
        Face {}
    }
    fn update(&mut self, new_time: f32, old_time: f32) {}
}

pub fn new() -> Box<RefCell<Sketch>> {
    Box::new(RefCell::new(Face::new())) as Box<RefCell<Sketch>>
}

pub static sketch: SketchDescriptor = SketchDescriptor {
    name: "Face",
    constructor: &new,
    mobile: true,
    desktop: true,
    public: true,
    url: "face",
};
