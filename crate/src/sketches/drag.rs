use crate::pico::*;
use crate::sketch::*;
use nalgebra::{Isometry2, Point2, Vector2};
use ncollide2d::query;
use ncollide2d::shape::{Ball, Cuboid, Shape, ShapeHandle};
use ncollide2d::world::CollisionWorld;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

pub trait Drawable {
    fn draw(&self);
}

pub trait Collidable {
    fn collides(&self, other: &Collidable) -> bool;
    fn isometry(&self) -> Isometry2<f64>;
    fn shape(&self) -> &Shape<f64>;
}

pub struct Circle {
    pub center: Vector2<f64>,
    pub radius: f64,
    pub ball: Ball<f64>,
    pub should_draw: bool,
    pub color: i32,
}

impl Circle {
    pub fn new(center: Vector2<f64>, radius: f64, color: i32) -> Circle {
        let ball = Ball::new(radius);
        Circle {
            color,
            center,
            radius,
            should_draw: true,
            ball,
        }
    }

    pub fn update_center(&mut self, new_center: &Vector2<f64>) {
        self.center.x = new_center.x;
        self.center.y = new_center.y;
    }

    pub fn update_radius(&mut self, new_radius: f64) {
        self.radius = new_radius;
        self.ball = Ball::new(new_radius);
    }

    pub fn update_color(&mut self, new_color: i32) {
        self.color = new_color;
    }

    pub fn contains_point(&self, point: &Point2<f64>) -> bool {
        ShapeHandle::new(self.ball.clone())
            .as_point_query()
            .unwrap()
            .contains_point(&Isometry2::new(self.center, 0.0), point)
    }
}

impl Collidable for Circle {
    fn collides(&self, other: &Collidable) -> bool {
        match query::contact(
            &self.isometry(),
            &self.ball,
            &other.isometry(),
            other.shape(),
            1.0,
        ) {
            None => false,
            Some(_) => true,
        }
    }
    fn isometry(&self) -> Isometry2<f64> {
        Isometry2::new(self.center, 0.0)
    }
    fn shape(&self) -> &Shape<f64> {
        &self.ball
    }
}

impl Drawable for Circle {
    fn draw(&self) {
        if self.should_draw {
            let center_x = self.center.x.round() as i32;
            let center_y = self.center.y.round() as i32;
            let radius = self.radius.round() as i32;

            circ_fill(center_x, center_y, radius, self.color);
        }
    }
}

pub struct Rectangle {
    pub center: Vector2<f64>,
    pub dimensions: Vector2<f64>,
    pub rectangle: Cuboid<f64>,
    pub should_draw: bool,
    pub color: i32,
}

impl Rectangle {
    pub fn new(center: Vector2<f64>, dimensions: Vector2<f64>, color: i32) -> Rectangle {
        let rectangle = Cuboid::new(dimensions / 2.0);
        Rectangle {
            color,
            center,
            dimensions,
            should_draw: true,
            rectangle,
        }
    }

    pub fn update_center(&mut self, new_center: &Vector2<f64>) {
        self.center.x = new_center.x;
        self.center.y = new_center.y;
    }

    pub fn update_dimensions(&mut self, new_dimensions: &Vector2<f64>) {
        self.dimensions.x = new_dimensions.x;
        self.dimensions.y = new_dimensions.y;
        self.rectangle = Cuboid::new(new_dimensions / 2.0);
    }

    pub fn update_color(&mut self, new_color: i32) {
        self.color = new_color;
    }

    pub fn contains_point(&self, point: &Point2<f64>) -> bool {
        ShapeHandle::new(self.rectangle.clone())
            .as_point_query()
            .unwrap()
            .contains_point(&Isometry2::new(self.center, 0.0), point)
    }
}

impl Collidable for Rectangle {
    fn collides(&self, other: &Collidable) -> bool {
        match query::contact(
            &self.isometry(),
            &self.rectangle,
            &other.isometry(),
            other.shape(),
            1.0,
        ) {
            None => false,
            Some(_) => true,
        }
    }
    fn isometry(&self) -> Isometry2<f64> {
        Isometry2::new(self.center, 0.0)
    }
    fn shape(&self) -> &Shape<f64> {
        &self.rectangle
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        if !self.should_draw {
            return;
        }
        let center_x = self.center.x.round() as i32;
        let center_y = self.center.y.round() as i32;
        let width = self.dimensions.x;
        let height = self.dimensions.y;
        let (left, top) = (-width / 2.0, -height / 2.0);
        let (right, bottom) = (left + width, top + height);
        let (left, top, right, bottom) = (
            left.round() as i32,
            top.round() as i32,
            right.round() as i32,
            bottom.round() as i32,
        );
        rect_fill(
            left + center_x,
            top + center_y,
            right + center_x,
            bottom + center_y,
            self.color,
        );
    }
}

pub struct Drag {
    rectangle: Rectangle,
    circle: Circle,
    dragging: bool,
}

impl Sketch for Drag {
    fn new() -> Drag {
        set_dimensions(512, 512);
        cls(0);

        Drag {
            rectangle: Rectangle::new(Vector2::new(256.0, 256.0), Vector2::new(30.0, 30.0), 12),
            circle: Circle::new(Vector2::identity(), 2.0, 7),
            dragging: true,
        }
    }
    fn update(&mut self, new_time: f32, old_time: f32) {
        cls(1);
        let t = ((new_time / 200.0).sin() * 10.0).round() as i32;
        rect_fill(40, 40, 80 + (t), 80, 9);

        let mouse_pos = get_pointer_position(0);
        let last_mouse_pos = get_last_pointer_position(0);
        if let Some(Point { x: new_x, y: new_y }) = mouse_pos {
            let point = Vector2::new(new_x as f64, new_y as f64);
            self.circle.should_draw = true;
            self.circle.update_center(&point);
            if self.rectangle.collides(&self.circle)
                && pointer_btn(0, 0)
                && pointer_btn_this_frame(0, 0)
            {
                self.dragging = true;
            } else if !pointer_btn(0, 0) {
                self.dragging = false;
            }
            if let (Some(Point { x: old_x, y: old_y }), true) = (last_mouse_pos, self.dragging) {
                let last_point = Vector2::new(old_x as f64, old_y as f64);
                self.circle.update_center(&last_point);
                if self.rectangle.collides(&self.circle) {
                    let p = self.rectangle.center + (point - last_point);
                    self.rectangle.update_center(&p);
                }
                self.circle.update_center(&point);
            }
        } else {
            self.circle.should_draw = false;
        }

        self.rectangle.draw();
        self.circle.draw();

        // self.rectangle.color = 12;
        // rot_rect(
        //     256,
        //     256,
        //     70, // + ((t.sin() * 30.0).round() as i32),
        //     50, // + ((t.cos() * 31.0).round() as i32),
        //     ((t.cos() * 50.0).round() as i32),
        //     0.0,
        //     12,
        // );
    }
}

pub fn new() -> Box<RefCell<Sketch>> {
    Box::new(RefCell::new(Drag::new())) as Box<RefCell<Sketch>>
}

pub static sketch: SketchDescriptor = SketchDescriptor {
    name: "Drag",
    constructor: &new,
    mobile: true,
    desktop: true,
    public: true,
    url: "drag",
};
