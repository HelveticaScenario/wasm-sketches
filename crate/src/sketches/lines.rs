use crate::pico::*;
use crate::sketch::*;
use std::cell::RefCell;

use std::cmp;

pub struct Lines {}

impl Sketch for Lines {
    fn new() -> Lines {
        set_dimensions(2048, 2048);
        cls(0);
        Lines {}
    }
    fn update(&mut self, new_time: u32, old_time: u32) {
        let width = WIDTH();
        let height = HEIGHT();
        for _ in 0..200 {
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
            line(x0 as i32, y0 as i32, x1 as i32, y1 as i32, c as i32);
        }
    }
}

pub fn new() -> Box<RefCell<Sketch>> {
    Box::new(RefCell::new(Lines::new())) as Box<RefCell<Sketch>>
}

pub static sketch: SketchDescriptor = SketchDescriptor {
    name: "Lines",
    constructor: &new,
    mobile: true,
    desktop: true,
    public: true,
    url: "lines",
};
