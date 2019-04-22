use crate::pico::*;
use crate::sketch::*;
use std::cell::RefCell;

use std::cmp;

pub struct Text {}

impl Sketch for Text {
    fn new() -> Text {
        set_dimensions(512, 512);
        cls(0);
        prnt(
            r##"use crate::pico::*;
use crate::sketch::*;
use std::cell::RefCell;

use std::cmp;

pub struct Text {}

impl Sketch for Text {
    fn new() -> Text {
        set_dimensions(512, 512);
        cls(0);
        prnt(r#"..."#.to_owned(), 0, 0, 7);
        Text {}
    }
    fn update(&mut self, new_time: f32, old_time: f32) {}
}

pub fn new() -> Box<RefCell<Sketch>> {
    Box::new(RefCell::new(Text::new())) as Box<RefCell<Sketch>>
}

pub static sketch: SketchDescriptor = SketchDescriptor {
    name: "Text",
    constructor: &new,
    mobile: true,
    desktop: true,
    public: true,
    url: "text",
};
"##
            .to_owned(),
            0,
            0,
            7,
        );
        Text {}
    }
    fn update(&mut self, new_time: f32, old_time: f32) {}
}

pub fn new() -> Box<RefCell<Sketch>> {
    Box::new(RefCell::new(Text::new())) as Box<RefCell<Sketch>>
}

pub static sketch: SketchDescriptor = SketchDescriptor {
    name: "Text",
    constructor: &new,
    mobile: true,
    desktop: true,
    public: true,
    url: "text",
};
