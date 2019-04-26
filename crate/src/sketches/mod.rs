pub mod circle_rect;
pub mod diagonals;
pub mod drawing;
pub mod erase;
pub mod erase2;
pub mod lines;
pub mod mandlebrot;
pub mod rand_static;
pub mod rects;
pub mod text;
pub mod face;
use crate::sketch::*;

pub static SKETCHES: &[&SketchDescriptor] = &[
    &rand_static::sketch,
    &diagonals::sketch,
    &lines::sketch,
    &rects::sketch,
    &circle_rect::sketch,
    &drawing::sketch,
    &erase::sketch,
    &erase2::sketch,
    &mandlebrot::sketch,
    &text::sketch,
    &face::sketch
];
