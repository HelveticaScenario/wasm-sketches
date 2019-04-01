use rand::prelude::*;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

trait FillExt<T> {
    fn fill(&mut self, v: T);
}

impl FillExt<u8> for [u8] {
    fn fill(&mut self, v: u8) {
        for i in self {
            *i = v
        }
    }
}
pub const MAX_SCREEN_SIZE: usize = 1024 * 1024;
pub const NUM_COLORS: usize = 256;
pub const DEFAULT_COLORS: [u8; 16 * 3] = [
    0, 0, 0, 29, 43, 83, 126, 37, 83, 0, 135, 81, 171, 82, 54, 95, 87, 79, 194, 195, 199, 255, 241,
    232, 255, 0, 77, 255, 164, 0, 255, 236, 39, 0, 228, 54, 41, 173, 255, 131, 118, 156, 255, 119,
    168, 255, 204, 170,
];

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct State {
    pub time: u32,
    pub offset: Point,
    pub mouse_pos: Option<Point>,
    pub dimensions: (usize, usize)
}

pub struct Container(pub RefCell<State>);
unsafe impl Sync for Container {}

pub struct Screen(pub RefCell<[u8; MAX_SCREEN_SIZE]>);
unsafe impl Sync for Screen {}

pub struct Palette(pub RefCell<[u8; NUM_COLORS * 3]>);
unsafe impl Sync for Palette {}

pub static STATE: Container = Container(RefCell::new(State {
    time: 0,
    offset: Point { x: 0, y: 0 },
    mouse_pos: None,
    dimensions: (128,128)
}));

pub static SCREEN: Screen = Screen(RefCell::new([0; MAX_SCREEN_SIZE]));
pub static PALETTE: Palette = Palette(RefCell::new([0; NUM_COLORS * 3]));

pub fn wrap_byte(n: i32) -> u8 {
    let mut n = n;
    while n < 0 {
        n += 256;
    }
    return (n % 256) as u8;
}

pub fn rect_swap(x0: i32, y0: i32, x1: i32, y1: i32) -> (i32, i32, i32, i32) {
    let mut x0 = x0;
    let mut x1 = x1;
    let mut y0 = y0;
    let mut y1 = y1;
    let swap_x = x0 > x1;
    let swap_y = y0 > y1;
    if swap_x {
        let tmp = x0;
        x0 = x1;
        x1 = tmp;
    }
    if swap_y {
        let tmp = y0;
        y0 = y1;
        y1 = tmp;
    }
    (x0, y0, x1, y1)
}

pub fn WIDTH() -> usize {
    STATE.0.borrow().dimensions.0
}

pub fn HEIGHT() -> usize {
    STATE.0.borrow().dimensions.1
}

pub fn set_dimensions(width: usize, height: usize) {
    if width * height > MAX_SCREEN_SIZE {
        panic!();
    }
    let mut state = STATE.0.borrow_mut();
    (*state).dimensions = (width, height);
}

pub fn offset_point(x: i32, y: i32) -> (i32, i32) {
    let state = STATE.0.borrow();
    ((-state.offset.x) + x, (-state.offset.y) + y)
}

pub fn is_x_on_screen(x: i32) -> bool {
    x >= 0 && x < (WIDTH() as i32)
}

pub fn is_y_on_screen(y: i32) -> bool {
    y >= 0 && y < (HEIGHT() as i32)
}

pub fn is_point_on_screen(x: i32, y: i32) -> bool {
    is_x_on_screen(x) && is_y_on_screen(y)
}

pub fn limit_x(x: i32) -> i32 {
    cmp::max(0, cmp::min((WIDTH() as i32) - 1, x))
}
pub fn limit_y(y: i32) -> i32 {
    cmp::max(0, cmp::min((HEIGHT() as i32) - 1, y))
}

pub fn limit_point(x: i32, y: i32) -> (i32, i32) {
    (limit_x(x), limit_y(y))
}

pub fn get_mouse_pos() -> Option<Point> {
    let state = STATE.0.borrow();
    if let Some(Point { x, y }) = state.mouse_pos {
        Some(Point { x, y })
    } else {
        None
    }
}

pub fn pset(x: i32, y: i32, c: i32) {
    let c = wrap_byte(c);
    let (x, y) = offset_point(x, y);
    if is_point_on_screen(x, y) {
        let mut screen = SCREEN.0.borrow_mut();
        screen[(y as usize) * WIDTH() + (x as usize)] = c;
    }
}

pub fn pget(x: i32, y: i32) -> Option<u8> {
    let (x, y) = offset_point(x, y);
    if is_point_on_screen(x, y) {
        let mut screen = SCREEN.0.borrow_mut();
        Some(screen[(y as usize) * WIDTH() + (x as usize)])
    } else {
        None
    }
}

pub fn cls(c: i32) {
    let c = wrap_byte(c);
    let mut screen = SCREEN.0.borrow_mut();
    for i in &mut screen[..] {
        *i = c
    }
}

pub fn camera_set(x: i32, y: i32) {
    let mut state = STATE.0.borrow_mut();
    (*state).offset.x = x;
    (*state).offset.y = y;
}

pub fn line(x0: i32, y0: i32, x1: i32, y1: i32, c: i32) {
    if y0 == y1 {
        rect_fill(x0, y0, x1, y1, c);
        return;
    }

    let (mut x0, mut y0) = offset_point(x0, y0);
    let (x1, y1) = offset_point(x1, y1);

    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = (y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = if dx > dy { dx } else { -dy } / 2;
    let mut e2;

    loop {
        pset(x0, y0, c);
        if x0 == x1 && y0 == y1 {
            break;
        }
        e2 = err;
        if e2 > -dx {
            err -= dy;
            x0 += sx;
        }
        if e2 < dy {
            err += dx;
            y0 += sy;
        }
    }
}

pub fn rect(x0: i32, y0: i32, x1: i32, y1: i32, c: i32) {
    let c = wrap_byte(c);
    let (x0, y0) = offset_point(x0, y0);
    let (x1, y1) = offset_point(x1, y1);
    let (x0, y0, x1, y1) = rect_swap(x0, y0, x1, y1);
    let mut screen = SCREEN.0.borrow_mut();
    let is_x0_on_screen = is_x_on_screen(x0);
    let is_y0_on_screen = is_y_on_screen(y0);
    let is_x1_on_screen = is_x_on_screen(x1);
    let is_y1_on_screen = is_y_on_screen(y1);
    if (is_x0_on_screen && is_y0_on_screen) || (is_x1_on_screen && is_y1_on_screen) {
        let (x0, y0) = limit_point(x0, y0);
        let (x1, y1) = limit_point(x1, y1);
        if is_y0_on_screen {
            let start = (y0 as usize) * WIDTH() + (x0 as usize);
            let end = (y0 as usize) * WIDTH() + (x1 as usize) + 1;
            screen[start..end].fill(c);
        }
        if is_y1_on_screen {
            let start = (y1 as usize) * WIDTH() + (x0 as usize);
            let end = (y1 as usize) * WIDTH() + (x1 as usize) + 1;
            screen[start..end].fill(c);
        }
        if is_x0_on_screen {
            let x0 = x0 as usize;
            for y in y0..(y1 + 1) {
                let y = y as usize;
                screen[y * WIDTH() + x0] = c;
            }
        }
        if is_x1_on_screen {
            let x1 = x1 as usize;
            for y in y0..(y1 + 1) {
                let y = y as usize;
                screen[y * WIDTH() + x1] = c;
            }
        }
    }
}
pub fn rect_fill(x0: i32, y0: i32, x1: i32, y1: i32, c: i32) {
    let c = wrap_byte(c);
    let (x0, y0) = offset_point(x0, y0);
    let (x1, y1) = offset_point(x1, y1);
    if is_point_on_screen(x0, y0) || is_point_on_screen(x1, y1) {
        let (x0, y0) = limit_point(x0, y0);
        let (x1, y1) = limit_point(x1, y1);
        let (x0, y0, x1, y1) = rect_swap(x0, y0, x1, y1);
        let mut screen = SCREEN.0.borrow_mut();
        for y in (y0 as usize)..((y1 as usize) + 1) {
            let start = y * WIDTH() + (x0 as usize);
            let end = y * WIDTH() + (x1 as usize) + 1;
            screen[start..end].fill(c);
        }
    }
}

pub fn circ(x: i32, y: i32, r: i32, c: i32) {
    if r < 1 {
        return;
    };
    let mut offx = r;
    let mut offy = 0;
    let mut decisionOver2 = 1 - offx; // Decision criterion divided by 2 evaluated at x=r, y=0
    while offy <= offx {
        pset(offx + x, offy + y, c); // Octant 1
        pset(offy + x, offx + y, c); // Octant 2
        pset(-offx + x, offy + y, c); // Octant 4
        pset(-offy + x, offx + y, c); // Octant 3
        pset(-offx + x, -offy + y, c); // Octant 5
        pset(-offy + x, -offx + y, c); // Octant 6
        pset(offx + x, -offy + y, c); // Octant 7
        pset(offy + x, -offx + y, c); // Octant 8
        offy += 1;
        if decisionOver2 <= 0 {
            decisionOver2 += 2 * offy + 1; // Change in decision criterion for y -> y+1
        } else {
            offx -= 1;
            decisionOver2 += 2 * (offy - offx) + 1; // Change for y -> y+1, x -> x-1
        }
    }
}

pub fn circ_fill(x: i32, y: i32, r: i32, c: i32) {
    if r < 1 {
        return;
    };
    let mut offx = r;
    let mut offy = 0;
    let mut decisionOver2 = 1 - offx; // Decision criterion divided by 2 evaluated at x=y=0

    while offy <= offx {
        line(offx + x, offy + y, -offx + x, offy + y, c); // Octant 1
        line(offy + x, offx + y, -offy + x, offx + y, c); // Octant 2
        line(-offx + x, -offy + y, offx + x, -offy + y, c); // Octant 5
        line(-offy + x, -offx + y, offy + x, -offx + y, c); // Octant 6
        offy += 1;
        if decisionOver2 <= 0 {
            decisionOver2 += 2 * offy + 1; // Change in decision criterion for y -> y+1
        } else {
            offx -= 1;
            decisionOver2 += 2 * (offy - offx) + 1; // Change for y -> y+1, x -> x-1
        }
    }
}
