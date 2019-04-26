use crate::pico::*;
use crate::sketch::*;
use std::cell::RefCell;

use std::cmp;

const string: &'static str = r##"use crate::pico::*;
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
"##;

pub struct Text {
    count: i32,
}

impl Sketch for Text {
    fn new() -> Text {
        set_dimensions(240 * 2, 136 * 2);
        cls(0);
        // let owned = string.to_owned();
        // let offset = Point { x: 2, y: 2 };
        // for y in -1..2 {
        //     for x in -1..2 {
        //         prnt(&owned, offset.x + x, offset.y + y, y + x + 3);
        //     }
        // }
        // prnt(&owned, offset.x, offset.y, 7);

        Text { count: 0 }
    }
    fn update(&mut self, new_time: f32, old_time: f32) {
        // cls(0);
        self.count += 1;
        self.count %= 64;
        if self.count % 16 == 0 {
            cls(0);
            let offset = Point { x: 2, y: 2 };
            prnt(
                &string,
                offset.x - 1,
                if (self.count % 32) == 0 {
                    offset.y
                } else {
                    offset.y - 1
                },
                3,
                if (self.count % 32) == 0 { 3 } else { 4 },
                7,
            );
            prnt(
                &string,
                offset.x,
                if (self.count % 32) == 0 {
                    offset.y + 1
                } else {
                    offset.y
                },
                1,
                1,
                0,
            );
        }
    }
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
