pub mod circle_rect;
pub mod diagonals;
pub mod drag;
pub mod drawing;
pub mod erase;
pub mod erase2;
pub mod face;
pub mod lines;
pub mod mandlebrot;
pub mod path_drawing;
pub mod rand_static;
pub mod rects;
pub mod text;
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
    &face::sketch,
    &drag::sketch,
    &path_drawing::sketch,
];
