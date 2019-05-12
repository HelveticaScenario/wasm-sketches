use crate::pico::*;
use crate::sketch::*;
use euclid::Point2D;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

use std::cmp;

pub struct PathDrawing {
    pub points: Vec<Point2D<f64>>,
}

impl Sketch for PathDrawing {
    fn new() -> PathDrawing {
        set_dimensions(512, 512);
        cls(0);
        set_target(1);
        cls(0);
        PathDrawing { points: vec![] }
    }
    fn update(&mut self, new_time: f32, old_time: f32) {
        let mouse_pos = get_pointer_position(0);
        let last_mouse_pos = get_last_pointer_position(0);
        if !pointer_btn(0, 0) && pointer_btn_this_frame(0, 0) {
            set_target(1);
            fat_line_strip(&self.points, 3.0, 7);
            self.points = vec![];
        }
        set_target(0);
        cls(0);
        copy_screen(1, 0);
        if let (Some(Point { x: new_x, y: new_y }), Some(Point { x: old_x, y: old_y })) =
            (mouse_pos, last_mouse_pos)
        {
            let m = Point2D::new(new_x as f64, new_y as f64);
            if (new_x != old_x || new_y != old_y) && pointer_btn(0, 0) {
                self.points.push(m.clone());
            }
            circ_euclid(&m, 8.0, 9);
        }

        fat_line_strip(&self.points, 3.0, 12);
    }
}

pub fn new() -> Box<RefCell<Sketch>> {
    Box::new(RefCell::new(PathDrawing::new())) as Box<RefCell<Sketch>>
}

pub static sketch: SketchDescriptor = SketchDescriptor {
    name: "Path Drawing",
    constructor: &new,
    mobile: true,
    desktop: true,
    public: true,
    url: "path-drawing",
};
