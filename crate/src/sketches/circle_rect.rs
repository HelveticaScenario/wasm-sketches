use rand::prelude::*;
use std::cell::RefCell;
use std::cmp;

use crate::pico::*;
use crate::sketch::*;

pub struct CircleRect {}

impl Sketch for CircleRect {
    fn new() -> CircleRect {
        cls(0);
        CircleRect {}
    }
    fn update(&mut self, new_time: f32, old_time: f32) {
        cls(0);
        // let mouse_pos = get_mouse_pos();
        let center_x = (WIDTH() / 2) as i32;
        let center_y = (HEIGHT() / 2) as i32;

        let t = (new_time as f32) / 300.0;
        let mouse_pos = Some(Point {
            x: ((t.sin() * center_x as f32) as i32) + center_x,
            y: ((t.cos() * center_y as f32) as i32) + center_y,
        });

        if let Some(Point { x, y }) = mouse_pos {
            let (x0, y0, x1, y1) = rect_swap(center_x, center_y, x, y);
            let diff_x = (x - center_x).abs();
            let diff_y = (y - center_y).abs();
            let min_diff = (cmp::min(diff_x, diff_y) / 2) + 1;
            for i in 0..min_diff {
                rect(x0 + i, y0 + i, x1 - i, y1 - i, (i % 15) + 1);
            }
        }
        circ(center_x, center_y, center_x - 1, 9);
    }
}

pub fn new() -> Box<RefCell<Sketch>> {
    Box::new(RefCell::new(CircleRect::new())) as Box<RefCell<Sketch>>
}

pub static sketch: SketchDescriptor = SketchDescriptor {
    name: "Circle-Rectangles",
    constructor: &new,
    mobile: true,
    desktop: true,
    public: true,
    url: "circle-rectangles",
};
