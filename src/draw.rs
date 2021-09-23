use crate::data::*;

pub struct Screen {
    pub w: usize,
    pub h: usize,
    pub data: Vec<u8>,
}

impl Screen {
    pub fn new(w: usize, h: usize) -> Screen {
        Screen {
            w: w,
            h: h,
            data: vec![0; w * h * 4]
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.w * self.h {
            set_px_unsafe_index(self, Color::BLACK, i);
        }
    }
}

#[inline]
fn set_px_unsafe(screen: &mut Screen, color: Color, point: PointScreen) {
    let i = (point.x + point.y * screen.w as isize) as usize;
    set_px_unsafe_index(screen, color, i);
}

#[inline]
fn set_px_unsafe_index(screen: &mut Screen, color: Color, i: usize) {
    screen.data[i * 4    ] = color.b;
    screen.data[i * 4 + 1] = color.g;
    screen.data[i * 4 + 2] = color.r;
}

#[inline]
fn set_px_safe(screen: &mut Screen, color: Color, point: PointScreen) {
    if point.x >= screen.w as isize { return; }
    if point.y >= screen.h as isize { return; }
    set_px_unsafe(screen, color, point);
}

fn clamp<T>(x: T, min: T, max: T) -> T
where T: PartialOrd<T> {
    if x <  min { return min; }
    if x >= max { return max; }
    return x;
}

pub fn draw_point(screen: &mut Screen, point: PointScreen, r: usize, color: Color) {
    let px = point.x as isize;
    let py = point.y as isize;
    let w = r as isize;

    let left  = clamp(px-w, 0, screen.w as isize-1);
    let right = clamp(px+w, 0, screen.w as isize-1);
    let top   = clamp(py-w, 0, screen.h as isize-1);
    let bot   = clamp(py+w, 0, screen.h as isize-1);

    for x in left..right+1 {
        for y in top..bot+1 {
            set_px_safe(screen, color, PointScreen { x: x, y: y, });
        }
    }
}

pub fn draw_line(
    screen: &mut Screen,
    mut p1: PointScreen,
    mut p2: PointScreen,
    color: Color
) {
    if p2.x < p1.x {
        let tmp = p1;
        p1 = p2;
        p2 = tmp;
    }

    let mut x1 = p1.x as isize;
    let mut y1 = p1.y as isize;
    let x2 = p2.x as isize;
    let y2 = p2.y as isize;
    let mut dx: isize = x2 - x1;
    let mut dy: isize = y2 - y1;

    if dx.abs() >= dy.abs() {
        if dx == 0 { fill_col(screen, p1.x, p1.y, p2.y, color); return }
        for x in 0..dx+1 {
            let y = y1 + x * dy / dx;
            set_px_safe(screen, color,
                PointScreen {
                    x: (x1 + x),
                    y: y
                });
        }
    } else {
        if (p2.x < p1.x && p2.y >= p1.y) || (p2.x >= p1.x && p2.y < p1.y) {
            x1 = x2;
            y1 = y2;
            dx *= -1;
            dy *= -1;
        }
        if dy == 0 { fill_row(screen, p1.y, p1.x, p2.x, color); return }
        for y in 0..dy+1 {
            let x = x1 + y * dx / dy;
            set_px_safe(screen, color,
                PointScreen {
                    x: x,
                    y: (y1 + y)
                });
        }
    }
}

pub fn draw_triangle(
    screen: &mut Screen,
    p1: PointScreen,
    p2: PointScreen,
    p3: PointScreen,
    color: Color
) {
    let (bot, mid, top) = {
        let (p1, p2, p3) = (p1, p2, p3);
        if p1.y >= p2.y {
            if p1.y >= p3.y {
                if p2.y >= p3.y { (p1, p2, p3) }
                else { (p1, p3, p2) }
            } else { (p3, p1, p2) }
        } else {
            if p2.y >= p3.y {
                if p1.y >= p3.y { (p2, p1, p3) }
                else { (p2, p3, p1) }
            } else { (p3, p2, p1) }
        }
    };

    let mid2 = {
        let dx = bot.x as isize - top.x as isize;
        let dy = bot.y as isize - top.y as isize;
        let i  = mid.y as isize - top.y as isize;
        PointScreen {
            x: (top.x as isize + dx * i / dy),
            y: mid.y
        }
    };

    fill_flat_top_tri(screen, bot, mid, mid2, color);
    fill_flat_bot_tri(screen, top, mid, mid2, color);
}

fn fill_flat_top_tri(
    screen: &mut Screen,
    bot: PointScreen,
    mut mid: PointScreen,
    mut mid2: PointScreen,
    color: Color
) {
    if mid.x > mid2.x {
        let tmp = mid;
        mid = mid2;
        mid2 = tmp;
    }

    let dx1 = bot.x as isize - mid.x  as isize;
    let dx2 = bot.x as isize - mid2.x as isize;
    let dy  = bot.y as isize - mid.y  as isize;

    if dy == 0 { return; }

    for i in 0..dy+1 {
        let lt_x = mid.x  as isize + i * dx1 / dy;
        let rt_x = mid2.x as isize + i * dx2 / dy;
        fill_row(screen, mid.y + i, lt_x, rt_x, color);
    }
}

fn fill_flat_bot_tri(
    screen: &mut Screen,
    top: PointScreen,
    mut mid: PointScreen,
    mut mid2: PointScreen,
    color: Color
) {
    if mid.x > mid2.x {
        let tmp = mid;
        mid = mid2;
        mid2 = tmp;
    }

    let dx1 = mid.x  as isize - top.x as isize;
    let dx2 = mid2.x as isize - top.x as isize;
    let dy = mid.y  as isize - top.y as isize;

    if dy == 0 { return; }

    for i in 0..dy+1 {
        let lt_x = top.x as isize + i * dx1 / dy;
        let rt_x = top.x as isize + i * dx2 / dy;
        fill_row(screen, top.y + i, lt_x, rt_x, color);
    }
}

fn fill_col(
    screen: &mut Screen,
    x: isize,
    mut y1: isize,
    mut y2: isize,
    color: Color
) {
    if x < 0  || x  >= (screen.h as isize) { return; }
    if y2 < 0 || y1 >= (screen.w as isize) { return; }
    if y1 < 0                    { y1 = 0; }
    if y2 >= (screen.h as isize) { y2 = screen.h as isize - 1; }

    for y in y1..y2+1 {
        set_px_unsafe(screen, color, PointScreen { x: x, y: y });
    }
}

fn fill_row(
    screen: &mut Screen,
    y: isize,
    mut x1: isize,
    mut x2: isize,
    color: Color
) {
    if y < 0  || y  >= (screen.h as isize) { return; }
    if x2 < 0 || x1 >= (screen.w as isize) { return; }
    if x1 < 0                    { x1 = 0; }
    if x2 >= (screen.w as isize) { x2 = screen.w as isize - 1; }

    for x in x1..x2+1 {
        set_px_unsafe(screen, color, PointScreen { x: x, y: y });
    }
}