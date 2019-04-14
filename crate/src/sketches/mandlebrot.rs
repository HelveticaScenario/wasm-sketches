use crate::pico::*;
use crate::sketch::*;
use std::cell::RefCell;

use std::cmp;

#[derive(Debug)]
struct ClipRectFloat {
    t: f64,
    b: f64,
    l: f64,
    r: f64,
}

pub struct Mandlebrot {
    clip_rect: ClipRectFloat,
    first_point: Option<Point>,
    started: bool,
    width: f64,
    height: f64,
    offset: u8,
}

impl Sketch for Mandlebrot {
    fn new() -> Mandlebrot {
        let width = 224;
        let height = 128;
        set_dimensions(width, height);
        cls(0);
        let m = Mandlebrot {
            clip_rect: ClipRectFloat {
                t: 0.0,
                b: height as f64,
                l: 0.0,
                r: width as f64,
            },
            // clip_rect: ClipRectFloat {
            //     t: 0.0 + height as f64 / 4.0,
            //     b: height as f64 - height as f64 / 4.0,
            //     l: 0.0 + width as f64 / 4.0,
            //     r: width as f64 - width as f64 / 4.0,
            // },
            first_point: None,
            started: false,
            width: width as f64,
            height: height as f64,
            offset: 0,
        };
        set_target(1);
        m.draw();
        m
    }
    fn update(&mut self, new_time: f32, old_time: f32) {
        let (scroll, scroll_delta) = get_scroll();
        self.offset = wrap_byte(scroll as i32) % 16;
        if scroll_delta != 0.0 {
            set_target(1);
            self.draw();
        }
        let mouse_pos = get_mouse_pos();
        if let Some(Point { x, y }) = mouse_pos {
            if btn(0) {
                if btn_this_frame(0) {
                    self.first_point = Some(Point { x, y });
                }
            } else if !btn(0) && btn_this_frame(0) {
                if let Some(Point {
                    x: first_x,
                    y: first_y,
                }) = self.first_point
                {
                    if x != first_x {
                        let i = (x - first_x) as f64 * (self.height / self.width);
                        let y = (first_y as f64) + i;
                        let (l, t, r, b) =
                            rect_swap(first_x as f64, first_y as f64, x as f64, y as f64);
                        let norm_t = t / self.height;
                        let norm_b = b / self.height;
                        let norm_l = l / self.width;
                        let norm_r = r / self.width;

                        let width = self.clip_rect.r - self.clip_rect.l;
                        let height = self.clip_rect.b - self.clip_rect.t;
                        let new_clip_rect = ClipRectFloat {
                            t: self.clip_rect.t + (height * norm_t),
                            b: self.clip_rect.t + (height * norm_b),
                            l: self.clip_rect.l + (width * norm_l),
                            r: self.clip_rect.l + (width * norm_r),
                        };
                        self.clip_rect = new_clip_rect;
                        set_target(1);
                        self.draw();
                    }
                    self.first_point = None;
                }
            }

            if scroll_delta != 0.0 {
                let norm_x = x as f64 / self.width;
                let norm_y = y as f64 / self.height;
                let clip_width = self.clip_rect.r - self.clip_rect.l;
                let clip_height = self.clip_rect.b - self.clip_rect.t;
                let scroll_delta = scroll_delta / 100.0;
                let t_diff = clip_height * norm_y * scroll_delta;
                let b_diff = clip_height * (norm_y - 1.0).abs() * scroll_delta;
                let l_diff = clip_width * norm_x * scroll_delta;
                let r_diff = clip_width * (norm_x - 1.0).abs() * scroll_delta;
                let t = if self.clip_rect.t + t_diff < 0.0 {
                    0.0
                } else {
                    self.clip_rect.t + t_diff
                };
                let b = if self.clip_rect.b - b_diff > self.height {
                    self.height
                } else {
                    self.clip_rect.b - b_diff
                };
                let l = if self.clip_rect.l + l_diff < 0.0 {
                    0.0
                } else {
                    self.clip_rect.l + l_diff
                };
                let r = if self.clip_rect.r - r_diff > self.width {
                    self.width
                } else {
                    self.clip_rect.r - r_diff
                };
                let (l, t, r, b) = rect_swap(l, t, r, b);
                let new_clip_rect = ClipRectFloat {
                    t: t,
                    b: b,
                    l: l,
                    r: r,
                };
                self.clip_rect = new_clip_rect;
                set_target(1);
                self.draw();
            }
        }
        if btn(2) && btn_this_frame(2) {
            self.clip_rect = ClipRectFloat {
                t: 0.0,
                b: self.height,
                l: 0.0,
                r: self.width,
            };
            set_target(1);
            self.draw();
        }
        set_target(0);
        copy_screen(1, 0);
        if let Some(Point { x, y }) = mouse_pos {
            if let Some(Point {
                x: first_x,
                y: first_y,
            }) = self.first_point
            {
                let i = (x - first_x) as f64 * (self.height / self.width);
                let y = (first_y as f64) + i;
                rect(x, (y) as i32, first_x, first_y, 12);
                // rect(x, y, first_x, first_y, 12);
            }
            circ(x, y, 1, 12);
        }
    }
}

const MAX_ITERATION: u32 = 128;
const BLOCK_SIZE: u32 = MAX_ITERATION / 16;

impl Mandlebrot {
    fn get_color(&self, px: usize, py: usize) -> u8 {
        let x0 = {
            let x = px as f64;
            let x = x / (self.width - 1.0);
            let width = (self.clip_rect.r - self.clip_rect.l) / (self.width - 1.0);
            let x_off = self.clip_rect.l / (self.width - 1.0);
            let scaled_x = (x * width) + x_off;
            (scaled_x * 3.5) - 2.5
        };
        let y0 = {
            let y = py as f64;
            let y = y / (self.height - 1.0);
            let height = (self.clip_rect.b - self.clip_rect.t) / (self.height - 1.0);
            let y_off = self.clip_rect.t / (self.height - 1.0);
            let scaled_y = (y * height) + y_off;
            (scaled_y * 2.0) - 1.0
        };
        let yy = y0 * y0;
        let q = (x0 - (0.25));
        let q = (q * q) + yy;
        if q * (q + (x0 - 0.25)) <= 0.25 * yy {
            return 0;
        }

        let mut x = 0.0;
        let mut y = 0.0;
        let mut iteration = 0;
        while (x * x + y * y) <= 4.0 && iteration < MAX_ITERATION {
            let x_temp = x * x - y * y + x0;
            let y_temp = 2.0 * x * y + y0;
            // if (x - x_temp).abs() <= std::f64::EPSILON && (y - y_temp).abs() <= std::f64::EPSILON {
            //     return 12;
            // }
            // if x == x_temp && y == y_temp {
            //     return 12;
            // }
            y = y_temp;
            x = x_temp;
            iteration += 1;
        }
        // (iteration / BLOCK_SIZE) as u8
        ((iteration) % 16) as u8
    }

    fn draw(&self) {
        for y in 0..(self.height as usize) {
            for x in 0..(self.width as usize) {
                pset(x as i32, y as i32, self.get_color(x, y) as i32);
            }
        }
    }
}

pub fn new() -> Box<RefCell<Sketch>> {
    Box::new(RefCell::new(Mandlebrot::new())) as Box<RefCell<Sketch>>
}

pub static sketch: SketchDescriptor = SketchDescriptor {
    name: "Mandlebrot",
    constructor: &new,
    mobile: false,
    desktop: true,
    public: true,
    url: "mandlebrot",
};
