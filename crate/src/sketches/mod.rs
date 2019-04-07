pub mod circle_rect;
pub mod diagonals;
pub mod drawing;
pub mod erase;
pub mod erase2;
pub mod lines;
pub mod rand_static;
pub mod rects;
use crate::sketch::*;

pub static CONSTRUCTORS: SketchConstructors = SketchConstructors(&[
    &rand_static::new_rand_static,
    &diagonals::new_diagonals,
    &lines::new_lines,
    &rects::new_rects,
    &circle_rect::new_circleRect,
    &drawing::new_drawing,
    &erase::new_erase,
    &erase2::new_erase2,
]);
