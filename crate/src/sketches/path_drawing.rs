use crate::pico::*;
use crate::sketch::*;
use nalgebra::Point2;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

use std::cmp;

pub struct PathDrawing {
    pub points: Vec<Point2<f64>>,
}

impl Sketch for PathDrawing {
    fn new() -> PathDrawing {
        set_dimensions(1024, 1024);
        cls(0);
        PathDrawing { points: vec![] }
    }
    fn update(&mut self, new_time: f32, old_time: f32) {
        // let mouse_pos = get_pointer_position(0);
        // let last_mouse_pos = get_last_pointer_position(0);
        // if let (Some(Point { x: new_x, y: new_y }), Some(Point { x: old_x, y: old_y })) =
        //     (mouse_pos, last_mouse_pos)
        // {
        //     let m = Point2::new(new_x as f64, new_y as f64);
        //     if new_x != old_x || new_y != old_y {
        //         self.points.push(m.clone());
        //     }
        //     circ_nalgebra(&m, 5.0, 9);

        //     // if self.points.len() > 0 {
        //     //     let i = self.points.len() - 1;
        //     //     self.points[i] = m.clone();
        //     // }
        // }
        // cls(0);
        // fat_line_strip(&self.points, 2.0, 7);
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
