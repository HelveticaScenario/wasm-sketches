pub mod rand_static;
pub mod diagonals;
pub mod lines;
pub mod rects;
pub mod circle_rect;
use crate::sketch::*;


pub static CONSTRUCTORS: SketchConstructors = SketchConstructors(&[
    &rand_static::new_rand_static,
    &diagonals::new_diagonals,
    &lines::new_lines,
    &rects::new_rects,
    &circle_rect::new_circleRect,
]);

