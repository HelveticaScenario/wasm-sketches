use crate::font::*;
use rand::prelude::*;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

pub trait FillExt<T> {
    fn fill(&mut self, v: T);
}

impl FillExt<u8> for [u8] {
    fn fill(&mut self, v: u8) {
        for i in self {
            *i = v
        }
    }
}

impl FillExt<i32> for [i32] {
    fn fill(&mut self, v: i32) {
        for i in self {
            *i = v
        }
    }
}

impl FillExt<bool> for [bool] {
    fn fill(&mut self, v: bool) {
        for i in self {
            *i = v
        }
    }
}

pub const MAX_WIDTH: usize = 1024;
pub const MAX_HEIGHT: usize = 1024;

pub const MAX_SCREEN_SIZE: usize = MAX_WIDTH * MAX_HEIGHT;
pub const NUM_COLORS: usize = 256;
pub const DEFAULT_COLORS: [u8; 16 * 3] = [
    0, 0, 0, 29, 43, 83, 126, 37, 83, 0, 135, 81, 171, 82, 54, 95, 87, 79, 194, 195, 199, 255, 241,
    232, 255, 0, 77, 255, 164, 0, 255, 236, 39, 0, 228, 54, 41, 173, 255, 131, 118, 156, 255, 119,
    168, 255, 204, 170,
];

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone, Debug)]
pub struct ClipRect {
    pub l: i32,
    pub t: i32,
    pub r: i32,
    pub b: i32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MouseButtonState {
    UpThisFrame,
    Up,
    DownThisFrame,
    Down,
}

pub const POINTER_COUNT: usize = 10;

pub struct State {
    pub time: f32,
    pub offset: Point,
    pub dimensions: (usize, usize),
    pub target: u8,
    pub sides_buffer_left: [i32; MAX_HEIGHT],
    pub sides_buffer_right: [i32; MAX_HEIGHT],
    pub clip_rect: ClipRect,
    pub transparency: [bool; NUM_COLORS],
    pub mouse_buttons: [MouseButtonState; 5],
    pub scroll: f64,
    pub scroll_delta: f64,
    pub pointer_pos: [Option<Point>; POINTER_COUNT],
    pub last_pointer_pos: [Option<Point>; POINTER_COUNT],
    pub pointer_state: [u32; POINTER_COUNT],
    pub last_pointer_state: [u32; POINTER_COUNT],
    pub pointer_pos_changed: bool,
    pub pointer_state_changed: bool,
}

/*
Device Button State	                                                    button	buttons
Mouse move with no buttons pressed	                                        -1	0
Left Mouse, Touch Contact, Pen contact (with no modifier buttons pressed)	0	1
Middle Mouse	                                                            1	4
Right Mouse, Pen contact with barrel button pressed	                        2	2
X1 (back) Mouse	                                                            3	8
X2 (forward) Mouse	                                                        4	16
Pen contact with eraser button pressed	                                    5	32
*/

const BUTTON_VALUES: [u32; 6] = [1, 2, 4, 8, 16, 32];

pub struct Container(pub RefCell<State>);
unsafe impl Sync for Container {}

pub struct Screen(pub RefCell<[u8; MAX_SCREEN_SIZE]>);
unsafe impl Sync for Screen {}

pub struct Palette(pub RefCell<[u8; NUM_COLORS * 3]>);
unsafe impl Sync for Palette {}

pub struct PaletteSwap(pub RefCell<[u8; NUM_COLORS]>);
unsafe impl Sync for PaletteSwap {}

pub static STATE: Container = Container(RefCell::new(State {
    time: 0.0,
    offset: Point { x: 0, y: 0 },
    dimensions: (128, 128),
    target: 0,
    sides_buffer_left: [0; MAX_HEIGHT],
    sides_buffer_right: [0; MAX_HEIGHT],
    clip_rect: ClipRect {
        l: 0,
        t: 0,
        r: MAX_WIDTH as i32,
        b: MAX_HEIGHT as i32,
    },
    transparency: [false; NUM_COLORS],
    mouse_buttons: [MouseButtonState::Up; 5],
    scroll: 0.0,
    scroll_delta: 0.0,
    pointer_pos: [None; 10],
    last_pointer_pos: [None; 10],
    pointer_state: [0; 10],
    last_pointer_state: [0; 10],
    pointer_pos_changed: false,
    pointer_state_changed: false,
}));

pub static SCREEN: Screen = Screen(RefCell::new([0; MAX_SCREEN_SIZE]));
pub static BUFFER1: Screen = Screen(RefCell::new([0; MAX_SCREEN_SIZE]));
pub static BUFFER2: Screen = Screen(RefCell::new([0; MAX_SCREEN_SIZE]));
pub static BUFFER3: Screen = Screen(RefCell::new([0; MAX_SCREEN_SIZE]));
pub static PALETTE: Palette = Palette(RefCell::new([0; NUM_COLORS * 3]));
pub static PALETTE_SWAP: PaletteSwap = PaletteSwap(RefCell::new([0; NUM_COLORS]));

pub fn wrap_byte(n: i32) -> u8 {
    let mut n = n;
    while n < 0 {
        n += 256;
    }
    return (n % 256) as u8;
}

pub fn rect_swap<A>(x0: A, y0: A, x1: A, y1: A) -> (A, A, A, A)
where
    A: PartialOrd,
{
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

pub fn screen<'a>(i: u8) -> std::cell::RefMut<'a, [u8; MAX_SCREEN_SIZE]> {
    match i {
        // Match a single value
        0 => SCREEN.0.borrow_mut(),
        1 => BUFFER1.0.borrow_mut(),
        2 => BUFFER2.0.borrow_mut(),
        3 => BUFFER3.0.borrow_mut(),
        // Handle the rest of cases
        _ => panic!("screen index {} no valid", i),
    }
}

pub fn get_target() -> u8 {
    STATE.0.borrow().target
}

pub fn set_target(target: u8) {
    STATE.0.borrow_mut().target = target;
}

pub fn WIDTH() -> usize {
    STATE.0.borrow().dimensions.0
}

pub fn HEIGHT() -> usize {
    STATE.0.borrow().dimensions.1
}

pub fn init_sides_buffer() {
    let width = WIDTH() as i32;
    let height = HEIGHT();
    let mut state = STATE.0.borrow_mut();
    state.sides_buffer_left[0..height].fill(width);
    state.sides_buffer_right[0..height].fill(-1);
}

pub fn set_side_pixel(x: i32, y: i32, _: i32) {
    let height = HEIGHT() as i32;
    if y >= 0 && y < height {
        let y = y as usize;
        let mut state = STATE.0.borrow_mut();
        if x < state.sides_buffer_left[y] {
            state.sides_buffer_left[y] = x;
        }
        if x > state.sides_buffer_right[y] {
            state.sides_buffer_right[y] = x;
        }
    }
}

pub fn set_dimensions(width: usize, height: usize) {
    if width * height > MAX_SCREEN_SIZE {
        panic!();
    }
    let mut state = STATE.0.borrow_mut();
    state.clip_rect.t = 0;
    state.clip_rect.l = 0;
    state.clip_rect.r = width as i32;
    state.clip_rect.b = height as i32;
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
    if let Some(Point { x, y }) = state.pointer_pos[0] {
        Some(Point { x, y })
    } else {
        None
    }
}

pub fn get_pointer_position(pointer: usize) -> Option<Point> {
    STATE.0.borrow().pointer_pos[pointer]
}
pub fn get_last_pointer_position(pointer: usize) -> Option<Point> {
    STATE.0.borrow().last_pointer_pos[pointer]
}

pub fn has_any_pointer_position_changed() -> bool {
    STATE.0.borrow().pointer_pos_changed
}
pub fn has_any_pointer_state_changed() -> bool {
    STATE.0.borrow().pointer_state_changed
}

pub fn get_scroll() -> (f64, f64) {
    let state = STATE.0.borrow();
    (state.scroll, state.scroll_delta)
}
pub fn has_scroll_changed() -> bool {
    STATE.0.borrow().scroll_delta != 0.0
}

pub fn get_mouse_btn(btn: u8) -> MouseButtonState {
    get_pointer_btn(0, btn)
}
pub fn get_pointer_btn(pointer: u8, btn: u8) -> MouseButtonState {
    let state = STATE.0.borrow();

    if (pointer as usize) < state.pointer_state.len() && (btn as usize) < BUTTON_VALUES.len() {
        let down_this_frame =
            (state.pointer_state[pointer as usize] & BUTTON_VALUES[btn as usize]) > 0;
        let down_last_frame =
            (state.last_pointer_state[pointer as usize] & BUTTON_VALUES[btn as usize]) > 0;

        match (down_this_frame, down_last_frame) {
            (true, true) => MouseButtonState::Down,
            (true, false) => MouseButtonState::DownThisFrame,
            (false, false) => MouseButtonState::Up,
            (false, true) => MouseButtonState::UpThisFrame,
        }
    } else {
        MouseButtonState::Up
    }
}

pub fn btn(btn_num: u8) -> bool {
    match get_mouse_btn(btn_num) {
        MouseButtonState::Up => false,
        MouseButtonState::UpThisFrame => false,
        MouseButtonState::Down => true,
        MouseButtonState::DownThisFrame => true,
    }
}

pub fn btn_this_frame(btn_num: u8) -> bool {
    match get_mouse_btn(btn_num) {
        MouseButtonState::Up => false,
        MouseButtonState::UpThisFrame => true,
        MouseButtonState::Down => false,
        MouseButtonState::DownThisFrame => true,
    }
}

pub fn pointer_btn(pointer: u8, btn_num: u8) -> bool {
    match get_pointer_btn(pointer, btn_num) {
        MouseButtonState::Up => false,
        MouseButtonState::UpThisFrame => false,
        MouseButtonState::Down => true,
        MouseButtonState::DownThisFrame => true,
    }
}

pub fn pointer_btn_this_frame(pointer: u8, btn_num: u8) -> bool {
    match get_pointer_btn(pointer, btn_num) {
        MouseButtonState::Up => false,
        MouseButtonState::UpThisFrame => true,
        MouseButtonState::Down => false,
        MouseButtonState::DownThisFrame => true,
    }
}

pub fn get_active_pointer_count() -> usize {
    STATE
        .0
        .borrow()
        .pointer_pos
        .iter()
        .filter(|x| {
            if let Some(Point { x: _x, y: _y }) = x {
                true
            } else {
                false
            }
        })
        .count()
}

pub fn get_last_active_pointer_count() -> usize {
    STATE
        .0
        .borrow()
        .last_pointer_pos
        .iter()
        .filter(|x| {
            if let Some(Point { x: _x, y: _y }) = x {
                true
            } else {
                false
            }
        })
        .count()
}

pub fn set_clip(x: i32, y: i32, w: i32, h: i32) {
    let mut state = STATE.0.borrow_mut();
    state.clip_rect.l = x;
    state.clip_rect.t = y;
    state.clip_rect.r = x + w;
    state.clip_rect.b = y + h;
    let Width = WIDTH() as i32;
    let Height = HEIGHT() as i32;
    if state.clip_rect.l < 0 {
        state.clip_rect.l = 0;
    }
    if state.clip_rect.t < 0 {
        state.clip_rect.t = 0;
    }
    if state.clip_rect.r > Width {
        state.clip_rect.r = Width;
    }
    if state.clip_rect.b > Height {
        state.clip_rect.b = Height;
    }
}

pub fn pset(x: i32, y: i32, c: i32) {
    let c = wrap_byte(c);
    let (x, y) = offset_point(x, y);
    if is_point_on_screen(x, y) {
        let mut screen = screen(get_target());
        screen[(y as usize) * WIDTH() + (x as usize)] = c;
    }
}

pub fn pget(x: i32, y: i32) -> Option<u8> {
    let (x, y) = offset_point(x, y);
    if is_point_on_screen(x, y) {
        let mut screen = screen(get_target());
        Some(screen[(y as usize) * WIDTH() + (x as usize)])
    } else {
        None
    }
}

pub fn palt(c: u8, t: bool) {
    let mut state = STATE.0.borrow_mut();
    state.transparency[c as usize] = t;
}

pub fn cls(c: i32) {
    let c = wrap_byte(c);
    let mut screen = screen(get_target());
    for i in &mut screen[..] {
        *i = c
    }
}

pub fn camera_set(x: i32, y: i32) {
    let mut state = STATE.0.borrow_mut();
    (*state).offset.x = x;
    (*state).offset.y = y;
}

fn line_with_pixel_func(x0: i32, y0: i32, x1: i32, y1: i32, c: i32, func: &Fn(i32, i32, i32)) {
    let (mut x0, mut y0) = offset_point(x0, y0);
    let (x1, y1) = offset_point(x1, y1);

    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = (y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = if dx > dy { dx } else { -dy } / 2;
    let mut e2;

    loop {
        func(x0, y0, c);
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

pub fn line(x0: i32, y0: i32, x1: i32, y1: i32, c: i32) {
    if y0 == y1 {
        rect_fill(x0, y0, x1, y1, c);
        return;
    }
    line_with_pixel_func(x0, y0, x1, y1, c, &pset);
}

pub fn fat_line(x0: i32, y0: i32, x1: i32, y1: i32, half_width: i32, caps: bool, c: i32) {
    let mut x = (x1 - x0) as f32;
    let mut y = (y1 - y0) as f32;
    let mag = ((x * x).abs() + (y * y).abs()).sqrt();
    x /= mag;
    y /= mag;

    let (norm_x, norm_y) = (y, (-x));
    let (norm_x, norm_y) = (
        (norm_x * half_width as f32) as i32,
        (norm_y * half_width as f32) as i32,
    );

    tri_fill(
        x0 - norm_x,
        y0 - norm_y,
        x1 - norm_x,
        y1 - norm_y,
        x0 + norm_x,
        y0 + norm_y,
        c,
    );
    tri_fill(
        x1 - norm_x,
        y1 - norm_y,
        x0 + norm_x,
        y0 + norm_y,
        x1 + norm_x,
        y1 + norm_y,
        c,
    );
    if caps {
        circ_fill(x1, y1, half_width, c);
        circ_fill(x0, y0, half_width, c);
    }
}

pub fn rect(x0: i32, y0: i32, x1: i32, y1: i32, c: i32) {
    let c = wrap_byte(c);
    let (x0, y0) = offset_point(x0, y0);
    let (x1, y1) = offset_point(x1, y1);
    let (x0, y0, x1, y1) = rect_swap(x0, y0, x1, y1);
    let mut screen = screen(get_target());
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
        let mut screen = screen(get_target());
        let width = WIDTH();
        for y in (y0 as usize)..((y1 as usize) + 1) {
            let start = y * width + (x0 as usize);
            let end = y * width + (x1 as usize) + 1;
            screen[start..end].fill(c);
        }
    }
}

pub fn circ_with_pixel_func(xm: i32, ym: i32, radius: i32, c: i32, func: &Fn(i32, i32, i32)) {
    if radius < 1 {
        return;
    };
    let mut r = radius;
    let mut x = -r;
    let mut y = 0;
    let mut err = 2 - 2 * r;

    let mut first = true;
    while first || x < 0 {
        first = false;
        func(xm - x, ym + y, c);
        func(xm - y, ym - x, c);
        func(xm + x, ym - y, c);
        func(xm + y, ym + x, c);

        r = err;
        if r <= y {
            y += 1;
            err += y * 2 + 1;
        }
        if r > x || err > y {
            x += 1;
            err += x * 2 + 1;
        }
    }
}

pub fn circ(x: i32, y: i32, r: i32, c: i32) {
    circ_with_pixel_func(x, y, r, c, &pset);
}

pub fn circ_fill(x: i32, y: i32, r: i32, c: i32) {
    if (r <= 0) {
        pset(x, y, c);
        return;
    }
    if (r == 1) {
        circ(x, y, r, c);
        pset(x, y, c);
        return;
    }
    init_sides_buffer();
    circ_with_pixel_func(x, y, r, c, &set_side_pixel);

    let state = STATE.0.borrow();
    let yt = cmp::max(state.clip_rect.t, y - r);
    let height = HEIGHT() as i32;
    let yb = cmp::min(state.clip_rect.b, y + r + 1);
    let c = wrap_byte(c);
    for _y in yt..yb {
        let _y = _y as usize;
        let xl = cmp::max(state.sides_buffer_left[_y], state.clip_rect.l) as usize;
        let xr = cmp::min(state.sides_buffer_right[_y], state.clip_rect.r - 1) as usize;
        hline(xl, xr, _y, c);
    }
}

pub fn tri(x0: i32, y0: i32, x1: i32, y1: i32, x2: i32, y2: i32, c: i32) {
    line(x0, y0, x1, y1, c);
    line(x1, y1, x2, y2, c);
    line(x2, y2, x0, y0, c);
}

pub fn tri_fill(x0: i32, y0: i32, x1: i32, y1: i32, x2: i32, y2: i32, c: i32) {
    init_sides_buffer();
    line_with_pixel_func(x0, y0, x1, y1, c, &set_side_pixel);
    line_with_pixel_func(x1, y1, x2, y2, c, &set_side_pixel);
    line_with_pixel_func(x2, y2, x0, y0, c, &set_side_pixel);

    let c = wrap_byte(c);
    let state = STATE.0.borrow();
    let yt = cmp::max(state.clip_rect.t, cmp::min(y0, cmp::min(y1, y2)));
    let height = HEIGHT() as i32;
    let yb = cmp::min(state.clip_rect.b, cmp::max(y0, cmp::max(y1, y2)) + 1);

    for _y in yt..yb {
        let _y = _y as usize;
        let xl = cmp::max(state.sides_buffer_left[_y], state.clip_rect.l) as usize;
        let xr = cmp::min(state.sides_buffer_right[_y] + 1, state.clip_rect.r - 1) as usize;
        hline(xl, xr, _y, c);
    }
}

pub fn hline(x0: usize, x1: usize, y: usize, c: u8) {
    let width = WIDTH();
    let start = y * width + (x0 as usize);
    let end = y * width + (x1 as usize) + 1;
    screen(get_target())[start..end].fill(c);
}

pub fn copy_screen(source: u8, target: u8) {
    let size = WIDTH() * HEIGHT();
    screen(target)[0..size].clone_from_slice(&screen(source)[0..size]);
}

pub fn copy_screen_with_transparency(source: u8, target: u8) {
    let size = WIDTH() * HEIGHT();
    let mut target_screen = screen(target);
    let source_screen = screen(source);
    let transparency = &(STATE.0.borrow().transparency);
    for i in 0..size {
        let source_color = source_screen[i];
        if (!transparency[source_color as usize]) {
            target_screen[i] = source_color;
        }
    }
}

pub fn copy_screen_with_transparency_mask(source: u8, target: u8, mask: u8) {
    let size = WIDTH() * HEIGHT();
    let mut target_screen = screen(target);
    let source_screen = screen(source);
    let mask_screen = screen(mask);
    let transparency = &(STATE.0.borrow().transparency);
    for i in 0..size {
        let source_color = source_screen[i];
        let mask_color = mask_screen[i];
        if (transparency[mask_color as usize]) {
            target_screen[i] = source_color;
        }
    }
}

// TODO: can be optomized to use clone_from_slice
pub fn copy_sprite(
    source: u8,
    target: u8,
    source_x: i32,
    source_y: i32,
    target_x: i32,
    target_y: i32,
    width: usize,
    height: usize,
) {
    let mut target_screen = screen(target);
    let source_screen = screen(source);
    let screen_width = WIDTH();
    let screen_height = HEIGHT();
    if screen_width == 0 || screen_height == 0 {
        return;
    }

    let source_clip_rect = ClipRect {
        l: cmp::max(0, source_x),
        r: cmp::min(screen_width as i32, source_x + width as i32),
        t: cmp::max(0, source_y),
        b: cmp::min(screen_height as i32, source_y + height as i32),
    };
    let target_clip_rect = ClipRect {
        l: cmp::max(0, target_x),
        r: cmp::min(screen_width as i32, target_x + width as i32),
        t: cmp::max(0, target_y),
        b: cmp::min(screen_height as i32, target_y + height as i32),
    };
    for y in 0..height {
        let source_pixel_y = source_y + y as i32;
        let target_pixel_y = target_y + y as i32;
        if source_pixel_y >= source_clip_rect.t
            && source_pixel_y < source_clip_rect.b
            && target_pixel_y >= target_clip_rect.t
            && target_pixel_y < target_clip_rect.b
        {
            for x in 0..width {
                let source_pixel_x = source_x + x as i32;
                let target_pixel_x = target_x + x as i32;
                if source_pixel_x >= source_clip_rect.l
                    && source_pixel_x < source_clip_rect.r
                    && target_pixel_x >= target_clip_rect.l
                    && target_pixel_x < target_clip_rect.r
                {
                    let source_pixel_x = source_pixel_x as usize;
                    let source_pixel_y = source_pixel_y as usize;
                    let target_pixel_x = target_pixel_x as usize;
                    let target_pixel_y = target_pixel_y as usize;
                    target_screen[(target_pixel_y * screen_width + target_pixel_x) as usize] =
                        source_screen[(source_pixel_y * screen_width + source_pixel_x) as usize];
                }
            }
        }
    }
}

pub fn copy_sprite_with_transparency(
    source: u8,
    target: u8,
    source_x: i32,
    source_y: i32,
    target_x: i32,
    target_y: i32,
    width: usize,
    height: usize,
) {
    let mut target_screen = screen(target);
    let source_screen = screen(source);
    let transparency = &(STATE.0.borrow().transparency);
    let screen_width = WIDTH();
    let screen_height = HEIGHT();
    if screen_width == 0 || screen_height == 0 {
        return;
    }

    let source_clip_rect = ClipRect {
        l: cmp::max(0, source_x),
        r: cmp::min(screen_width as i32, source_x + width as i32),
        t: cmp::max(0, source_y),
        b: cmp::min(screen_height as i32, source_y + height as i32),
    };
    let target_clip_rect = ClipRect {
        l: cmp::max(0, target_x),
        r: cmp::min(screen_width as i32, target_x + width as i32),
        t: cmp::max(0, target_y),
        b: cmp::min(screen_height as i32, target_y + height as i32),
    };
    for y in 0..height {
        let source_pixel_y = source_y + y as i32;
        let target_pixel_y = target_y + y as i32;
        if source_pixel_y >= source_clip_rect.t
            && source_pixel_y < source_clip_rect.b
            && target_pixel_y >= target_clip_rect.t
            && target_pixel_y < target_clip_rect.b
        {
            for x in 0..width {
                let source_pixel_x = source_x + x as i32;
                let target_pixel_x = target_x + x as i32;
                if source_pixel_x >= source_clip_rect.l
                    && source_pixel_x < source_clip_rect.r
                    && target_pixel_x >= target_clip_rect.l
                    && target_pixel_x < target_clip_rect.r
                {
                    let source_pixel_x = source_pixel_x as usize;
                    let source_pixel_y = source_pixel_y as usize;
                    let target_pixel_x = target_pixel_x as usize;
                    let target_pixel_y = target_pixel_y as usize;
                    let source_color =
                        source_screen[(source_pixel_y * screen_width + source_pixel_x) as usize];
                    if !transparency[source_color as usize] {
                        target_screen[(target_pixel_y * screen_width + target_pixel_x) as usize] =
                            source_color
                    }
                }
            }
        }
    }
}

fn as_u16_le(array: &[u8; 2]) -> u16 {
    ((array[0] as u16) << 0) + ((array[1] as u16) << 8)
}

pub fn load_spritesheet(bytes: &[u8]) {
    let width = as_u16_le(&[bytes[0], bytes[1]]) as usize;
    let height = as_u16_le(&[bytes[2], bytes[3]]) as usize;
    {
        PALETTE
            .0
            .borrow_mut()
            .clone_from_slice(&bytes[4..(NUM_COLORS * 3 + 4)]);
    };

    let offset = NUM_COLORS * 3 + 4;
    let mut buf = BUFFER3.0.borrow_mut();
    let (buf_width, buf_height) = (WIDTH(), HEIGHT());
    for i in 0..height {
        buf[(buf_width * i)..(buf_width * i + width)]
            .copy_from_slice(&bytes[(offset + (width * i))..(offset + (width * (i + 1)))]);
    }
}

// pub fn get_bit() {
//     let idx = (addr / 8) as u8;
//     let bit = (addr % 8) as u8;
//     return (bool)((arr[idx] >> bit) & 1);
// }

pub fn prnt(string: &str, x: i32, y: i32, w: i32, h: i32, c: i32) {
    let mut _x = x;
    let mut _y = y;
    for character in string.chars() {
        if character.is_ascii() {
            let character = character as usize;
            if character > 32 && character < 128 {
                for i in 0..8 {
                    let line = BIT_FONT[(((character - 33) * 8) + i) as usize];
                    for j in 0..8 {
                        if ((line >> j) & 1) == 1 {
                            rect_fill(
                                _x + j,
                                _y + i as i32,
                                w - 1 + _x + j,
                                h - 1 + _y + i as i32,
                                c,
                            );
                        }
                    }
                }
                _x += 8;
            } else if character == 13 {
                // carriage return, do nothing
            } else if character == 10 {
                _x = x;
                _y += 9;
            } else {
                _x += 8;
            }
        }
    }
    // for ( std::string::iterator it=str.begin(); it!=str.end(); ++it) {
    //     uint8_t character = (uint8_t) *it;
    //     if (character > 32 && character < 128) {
    //         for (int16_t i = 0; i < 8; i++) {
    //             uint8_t line = *(font_data.begin + ((character - 33)*8) + i);
    //             for (int16_t j =0; j < 8; j++) {
    //                 if (rose_get_bit(&line, (uint8_t) j)) {
    //                     pset(_x + j, _y + i, c);
    //                 }
    //             }
    //         }
    //         _x += 8;
    //     } else if (character == 13) {
    //         // carriage return, do nothing
    //     } else if (character == 10) {
    //         _x = x;
    //         _y += 9;
    //     } else {
    //         _x += 8;
    //     }
    // }
    // int16_t* ptr = (int16_t*) print_cursor.begin;
    // ptr[0] = _x;
    // ptr[1] = _y;
    // return ROSE_API_ERR_NONE;
}
