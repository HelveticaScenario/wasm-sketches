use crate::pico::*;
use crate::sketch::*;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

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
        let pointer_state_changed = has_any_pointer_state_changed();
        let pointer_pos_changed = has_any_pointer_position_changed();
        let scroll_changed = has_scroll_changed();
        let active_pointer_count = get_active_pointer_count();
        let last_active_pointer_count = get_last_active_pointer_count();
        let mut should_draw = false;
        if pointer_pos_changed || pointer_state_changed || scroll_changed {
            if active_pointer_count == 1 {
                let pointer_pos = get_pointer_position(0);
                let last_pointer_pos = get_last_pointer_position(0);

                if let (true, Some(pointer_pos), Some(last_pointer_pos)) =
                    (pointer_btn(0, 0), pointer_pos, last_pointer_pos)
                {
                    self.pan_update(pointer_pos, last_pointer_pos);
                    should_draw = true;
                }

                if let (true, Some(pointer_pos)) = (scroll_changed, pointer_pos) {
                    let (_, scroll_delta) = get_scroll();
                    self.scroll_update(pointer_pos, scroll_delta);
                    should_draw = true;
                }
            } else if let (
                true,
                Some(primary_pointer_pos),
                Some(last_primary_pointer_pos),
                Some(secondary_pointer_pos),
                Some(last_secondary_pointer_pos),
            ) = (
                active_pointer_count > 1,
                get_pointer_position(0),
                get_last_pointer_position(0),
                get_pointer_position(1),
                get_last_pointer_position(1),
            ) {
                self.multitouch_update(
                    primary_pointer_pos,
                    last_primary_pointer_pos,
                    secondary_pointer_pos,
                    last_secondary_pointer_pos,
                );
                should_draw = true;
            }
        }

        if should_draw {
            set_target(1);
            self.draw();
        }

        set_target(0);
        copy_screen(1, 0);
        if let (true, Some(Point { x, y })) = (active_pointer_count == 1, get_pointer_position(0)) {
            circ(x, y, 1, 12);
        }
    }
}

const MAX_ITERATION: u32 = 128;

impl Mandlebrot {
    fn pan_update(
        &mut self,
        Point { x, y }: Point,
        Point {
            x: last_x,
            y: last_y,
        }: Point,
    ) {
        let (diff_x, diff_y) = (
            (last_x - x) as f64 / self.width,
            (last_y - y) as f64 / self.height,
        );

        let width = self.clip_rect.r - self.clip_rect.l;
        let height = self.clip_rect.b - self.clip_rect.t;
        let mut new_clip_rect = ClipRectFloat {
            t: self.clip_rect.t + (height * diff_y),
            b: self.clip_rect.b + (height * diff_y),
            l: self.clip_rect.l + (width * diff_x),
            r: self.clip_rect.r + (width * diff_x),
        };
        if new_clip_rect.t == new_clip_rect.b || new_clip_rect.l == new_clip_rect.r {
            return;
        }
        if new_clip_rect.t > new_clip_rect.b {
            let tmp = new_clip_rect.t;
            new_clip_rect.t = new_clip_rect.b;
            new_clip_rect.b = tmp;
        }
        if new_clip_rect.l > new_clip_rect.r {
            let tmp = new_clip_rect.l;
            new_clip_rect.l = new_clip_rect.r;
            new_clip_rect.r = tmp;
        }
        self.clip_rect = new_clip_rect;
    }

    fn scroll_update(&mut self, Point { x, y }: Point, delta: f64) {
        let norm_x = x as f64 / self.width;
        let norm_y = y as f64 / self.height;
        let clip_width = self.clip_rect.r - self.clip_rect.l;
        let clip_height = self.clip_rect.b - self.clip_rect.t;
        let scroll_delta = delta / 100.0;
        let t_diff = clip_height * norm_y * scroll_delta;
        let b_diff = clip_height * (norm_y - 1.0).abs() * scroll_delta;
        let l_diff = clip_width * norm_x * scroll_delta;
        let r_diff = clip_width * (norm_x - 1.0).abs() * scroll_delta;

        let (l, t, r, b) = rect_swap(
            self.clip_rect.l + l_diff,
            self.clip_rect.t + t_diff,
            self.clip_rect.r - r_diff,
            self.clip_rect.b - b_diff,
        );
        let new_clip_rect = ClipRectFloat {
            t: t,
            b: b,
            l: l,
            r: r,
        };
        self.clip_rect = new_clip_rect;
    }

    fn multitouch_update(
        &mut self,
        Point {
            x: primary_x,
            y: primary_y,
        }: Point,
        Point {
            x: last_primary_x,
            y: last_primary_y,
        }: Point,
        Point {
            x: secondary_x,
            y: secondary_y,
        }: Point,
        Point {
            x: last_secondary_x,
            y: last_secondary_y,
        }: Point,
    ) {
        let center_x = (primary_x + secondary_x) / 2;
        let center_y = (primary_y + secondary_y) / 2;
        let last_center_x = (last_primary_x + last_secondary_x) / 2;
        let last_center_y = (last_primary_y + last_secondary_y) / 2;
        self.pan_update(
            Point {
                x: center_x,
                y: center_y,
            },
            Point {
                x: last_center_x,
                y: last_center_y,
            },
        );

        let length = {
            let x = (secondary_x - primary_x) as f64 / self.width;
            let y = (secondary_y - primary_y) as f64 / self.height;
            ((x * x) + (y * y)).sqrt()
        };

        let last_length = {
            let x = (last_secondary_x - last_primary_x) as f64 / self.width;
            let y = (last_secondary_y - last_primary_y) as f64 / self.height;
            ((x * x) + (y * y)).sqrt()
        };
        let diff_length = (length - last_length) * 2.0;
        self.scroll_update(
            Point {
                x: center_x,
                y: center_y,
            },
            diff_length * 100.0,
        );
    }
    /*
        fn update_fractal_and_draw(
            &mut self,
            pointer_pos: Point,
            down: bool,
            this_frame: bool,
            scroll_delta: f64,
        ) {
            let Point { x, y } = pointer_pos;
            if down {
                if this_frame {
                    self.first_point = Some(Point { x, y });
                }
            } else if !down && this_frame {
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

    */
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
    mobile: true,
    desktop: true,
    public: true,
    url: "mandlebrot",
};
