use std::cell::RefCell;

pub trait Sketch {
     fn new() -> Self
     where
          Self: Sized;
     fn update(&mut self, new_time: u32, old_time: u32);
}
pub struct SketchContainer(pub RefCell<Option<Box<RefCell<Sketch>>>>);
unsafe impl Sync for SketchContainer {}

pub struct SketchDescriptor {
     pub name: &'static str,
     pub constructor: &'static (Fn() -> Box<RefCell<Sketch>>),
     pub mobile: bool,
     pub desktop: bool,
     pub public: bool,
     pub url: &'static str,
}
unsafe impl Sync for SketchDescriptor {}
