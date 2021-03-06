use crate::pico::*;
use crate::sketch::*;
use std::cell::RefCell;

use std::cmp;

pub struct RandStatic {}

impl Sketch for RandStatic {
    fn new() -> RandStatic {
        cls(0);
        RandStatic {}
    }
    fn update(&mut self, new_time: f32, old_time: f32) {
        let mut screen = screen(0);
        let width = WIDTH();
        let height = HEIGHT();
        for y in 0..height {
            for x in 0..width {
                let i = y * width + x;
                let num: u8 = rand::random();
                screen[i] = num % 16;
            }
        }
    }
}

pub fn new() -> Box<RefCell<Sketch>> {
    Box::new(RefCell::new(RandStatic::new())) as Box<RefCell<Sketch>>
}

pub static sketch: SketchDescriptor = SketchDescriptor {
    name: "Random Static",
    constructor: &new,
    mobile: true,
    desktop: true,
    public: true,
    url: "random-static",
};
