use crate::pico::*;
use crate::sketch::*;
use std::cell::RefCell;

use std::cmp;

pub struct Rects {}

impl Sketch for Rects {
    fn new() -> Rects {
        cls(0);
        Rects {}
    }
    fn update(&mut self, new_time: f32, old_time: f32) {
        let width = WIDTH();
        let height = HEIGHT();
        for _ in 0..1 {
            let x0: u32 = rand::random();
            let x0 = x0 % (width as u32);
            let x1: u32 = rand::random();
            let x1 = x1 % (width as u32);
            let y0: u32 = rand::random();
            let y0 = y0 % (height as u32);
            let y1: u32 = rand::random();
            let y1 = y1 % (height as u32);
            let c: u32 = rand::random();
            let c = c % 16;
            rect(x0 as i32, y0 as i32, x1 as i32, y1 as i32, c as i32);
        }
    }
}

pub fn new() -> Box<RefCell<Sketch>> {
    Box::new(RefCell::new(Rects::new())) as Box<RefCell<Sketch>>
}

pub static sketch: SketchDescriptor = SketchDescriptor {
    name: "Rectangles",
    constructor: &new,
    mobile: true,
    desktop: true,
    public: true,
    url: "rectangles",
};
