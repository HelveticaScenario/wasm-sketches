use std::cell::RefCell;

pub trait Sketch {
     fn new() -> Self where Self: Sized;
     fn update(&mut self, new_time: u32, old_time: u32);
}
pub struct SketchContainer(pub RefCell<Option<Box<RefCell<Sketch>>>>);
unsafe impl Sync for SketchContainer {}

pub struct SketchConstructors(pub &'static [&'static (Fn() -> Box<RefCell<Sketch>>)]);
unsafe impl Sync for SketchConstructors {}