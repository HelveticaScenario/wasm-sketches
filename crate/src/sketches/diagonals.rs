use crate::pico::*;
use crate::sketch::*;
use std::cell::RefCell;

use std::cmp;

pub struct Diagonals {
    pub count: u8
}

impl Sketch for Diagonals {
    fn new() -> Diagonals {
        cls(0);
        // set_dimensions(256, 512);
        Diagonals {
            count:  0,
        }
    }
    fn update(&mut self, new_time: u32, old_time: u32) {
        // let o = new_time / 16;
        self.count += 1;
        
        self.count = self.count % 16;
        let o = self.count;
        let mut screen = screen(0);
        let height = HEIGHT();
        let width = WIDTH();
        for y in 0..height {
            for x in 0..width {
                let i = y * width + x;
                screen[i] = ((x + y + o as usize) % 16) as u8;
            }
        }
    }
}

pub fn new_diagonals() -> Box<RefCell<Sketch>> {
    Box::new(RefCell::new(Diagonals::new())) as Box<RefCell<Sketch>>
}
