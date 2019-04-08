use crate::pico::*;
use crate::sketch::*;
use std::cell::RefCell;

use std::cmp;

pub struct Drawing {
    pub last_mouse: Option<Point>,
}

impl Sketch for Drawing {
    fn new() -> Drawing {
        set_target(0);
        cls(0);
        set_target(1);
        cls(0);
        // set_dimensions(256, 512);
        Drawing { last_mouse: None }
    }
    fn update(&mut self, new_time: u32, old_time: u32) {
        set_target(1);
        let mouse_pos = get_mouse_pos();
        if let Some(Point { x: new_x, y: new_y }) = mouse_pos {
            let c: u8 = rand::random();
            let c = (c % 15) + 1;
            if let Some(Point {
                x: last_x,
                y: last_y,
            }) = self.last_mouse
            {
                line(last_x, last_y, new_x, new_y, c as i32);
            } else {
                pset(new_x, new_y, c as i32);
            }
            self.last_mouse = Some(Point { x: new_x, y: new_y });
            set_target(0);
            copy_screen(1, 0);
            circ_fill(new_x, new_y, 5, 9);
        } else {
            if let Some(Point { x, y }) = self.last_mouse {
                self.last_mouse = None;
            }
            copy_screen(1, 0);
        }
    }
}

pub fn new() -> Box<RefCell<Sketch>> {
    Box::new(RefCell::new(Drawing::new())) as Box<RefCell<Sketch>>
}

pub static sketch: SketchDescriptor = SketchDescriptor {
    name: "Drawing",
    constructor: &new,
    mobile: true,
    desktop: true,
    public: true,
    url: "drawing",
};
