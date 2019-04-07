use crate::pico::*;
use crate::sketch::*;
use std::cell::RefCell;

use std::cmp;

pub struct Erase2 {
    pub last_mouse: Option<Point>,
    pub radius: i32,
    pub count: u32,
}

impl Sketch for Erase2 {
    fn new() -> Erase2 {
        set_dimensions(1024, 1024);
        set_target(0);
        cls(0);
        set_target(1);
        cls(0);
        set_target(2);
        cls(0);
        let mut screen = screen(1);
        let real_width = WIDTH();
        let real_height = HEIGHT();
        // for y in 0..height {
        //     for x in 0..width {
        //         let i = y * width + x;
        //         let num: u8 = rand::random();
        //         screen[i] = (num % 15) + 1;
        //     }
        // }

        let width = 128;
        let width_mult = real_width / width;
        let height = 128;
        let height_mult = real_height / height;
        for y in 0..height {
            for x in 0..width {
                let x0 = (x * width_mult);
                let y0 = (y * height_mult);
                let x1 = x0 + width_mult;
                let y1 = y0 + height_mult;
                let c = (x + y) % 16;
                rect_fill(x0 as i32, y0 as i32, x1 as i32, y1 as i32, c as i32);
            }
        }
        // for y in 0..height {
        //     for x in 0..width {
        //         let i = y * width + x;
        //         screen[i] = (((x / 8) + (y / 8) as usize) % 16) as u8;
        //     }
        // }
        palt(0, false);
        palt(16, true);
        Erase2 {
            last_mouse: None,
            radius: 10,
            count: 0,
        }
    }
    fn update(&mut self, new_time: u32, old_time: u32) {
        set_target(1);
        self.count += 1;
        self.count = self.count % (16 * 8);
        let offset = self.count as usize;
        {
            let real_width = WIDTH();
            let real_height = HEIGHT();
            let width = 128;
            let width_mult = real_width / width;
            let height = 128;
            let height_mult = real_height / height;
            for y in 0..height {
                for x in 0..width {
                    let x0 = (x * width_mult);
                    let y0 = (y * height_mult);
                    let x1 = x0 + width_mult;
                    let y1 = y0 + height_mult;
                    let c = (x + y + (offset / 2) as usize) % 16;
                    rect_fill(x0 as i32, y0 as i32, x1 as i32, y1 as i32, c as i32);
                }
            }
        }
        set_target(2);
        let mouse_pos = get_mouse_pos();
        if let Some(Point { x: new_x, y: new_y }) = mouse_pos {
            // cls(0);
            if let Some(Point {
                x: last_x,
                y: last_y,
            }) = self.last_mouse
            {
                if new_x == last_x && new_y == last_y {
                    circ_fill(new_x, new_y, self.radius, 16);
                } else {
                    // let mut x = (new_x - last_x) as f32;
                    // let mut y = (new_y - last_y) as f32;
                    // let mag = ((x * x).abs() + (y * y).abs()).sqrt();
                    // x /= mag;
                    // y /= mag;

                    // let (norm_x, norm_y) = (y, (-x));
                    // let (norm_x, norm_y) = (
                    //     (norm_x * self.radius as f32) as i32,
                    //     (norm_y * self.radius as f32) as i32,
                    // );

                    // tri_fill(
                    //     last_x - norm_x,
                    //     last_y - norm_y,
                    //     new_x - norm_x,
                    //     new_y - norm_y,
                    //     last_x + norm_x,
                    //     last_y + norm_y,
                    //     1,
                    // );
                    // tri_fill(
                    //     new_x - norm_x,
                    //     new_y - norm_y,
                    //     last_x + norm_x,
                    //     last_y + norm_y,
                    //     new_x + norm_x,
                    //     new_y + norm_y,
                    //     1,
                    // );
                    // circ_fill(new_x, new_y, self.radius, 1);
                    // circ_fill(last_x, last_y, self.radius, 1);
                    fat_line(last_x, last_y, new_x, new_y, self.radius, true, 16);
                }
            } else {
                circ_fill(new_x, new_y, self.radius, 16);
            }
            self.last_mouse = Some(Point { x: new_x, y: new_y });
        } else {
            if let Some(Point { x, y }) = self.last_mouse {
                self.last_mouse = None;
            }
        }
        set_target(0);
        {
            let real_width = WIDTH();
            let real_height = HEIGHT();
            let width = 128;
            let width_mult = real_width / width;
            let height = 128;
            let height_mult = real_height / height;
            for y in 0..height {
                for x in 0..width {
                    let x0 = (x * width_mult);
                    let y0 = (y * height_mult);
                    let x1 = x0 + width_mult;
                    let y1 = y0 + height_mult;
                    let v = (y as i32) * -2 + x as i32 + (offset / 4) as i32;
                    let c = wrap_byte(v) % 16;
                    rect_fill(x0 as i32, y0 as i32, x1 as i32, y1 as i32, c as i32);
                }
            }
        }
        // copy_screen(1, 0);
        // copy_screen_with_transparency(2, 0);
        copy_screen_with_transparency_mask(1, 0, 2);
    }
}

pub fn new() -> Box<RefCell<Sketch>> {
    Box::new(RefCell::new(Erase2::new())) as Box<RefCell<Sketch>>
}

pub static sketch: SketchDescriptor = SketchDescriptor {
    name: "Erase 2",
    constructor: &new,
    mobile: true,
    desktop: true,
    public: true,
};
