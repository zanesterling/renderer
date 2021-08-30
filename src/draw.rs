use crate::data::*;

pub struct Screen {
    pub w: usize,
    pub h: usize,
    pub data: Vec<Color>,
}

impl Screen {
    pub fn new(w: usize, h: usize) -> Screen {
        Screen {
            w: w,
            h: h,
            data: vec![Color {r: 0, g: 0, b: 0}; w * h]
        }
    }
}

fn set_px_unsafe(screen: &mut Screen, color: Color, point: PointScreen) {
    screen.data[point.x + point.y * screen.w] = color;
}

fn set_px_safe(screen: &mut Screen, color: Color, point: PointScreen) {
    if point.x >= screen.w { return; }
    if point.y >= screen.w { return; }
    set_px_unsafe(screen, color, point);
}

fn clamp<T>(x: T, min: T, max: T) -> T
where T: PartialOrd<T> {
    if x <  min { return min; }
    if x >= max { return max; }
    return x;
}

pub fn draw_point(screen: &mut Screen, point: PointScreen, w: usize, color: Color) {
    let left  = clamp(point.x - w, 0, screen.w-1);
    let right = clamp(point.x + w, 0, screen.w-1);
    let top = clamp(point.y - w, 0, screen.h-1);
    let bot = clamp(point.y + w, 0, screen.h-1);

    for x in left..right {
        for y in top..bot {
            set_px_unsafe(screen, color, PointScreen { x: x, y: y });
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
    let mut x2 = p2.x as isize;
    let mut y1 = p1.y as isize;
    let mut y2 = p2.y as isize;
    let mut dx: isize = x2 - x1;
    let mut dy: isize = y2 - y1;

    if dx.abs() >= dy.abs() {
        for x in 0..dx+1 {
            let y = y1 + x * dy / dx;
            set_px_safe(screen, color,
                PointScreen {
                    x: (x1 + x) as usize,
                    y: y as usize
                });
        }
    } else {
        if (p2.x < p1.x && p2.y >= p1.y) || (p2.x >= p1.x && p2.y < p1.y) {
            let (tx1, ty1) = (x1, y1);
            x1 = x2;
            y1 = y2;
            x2 = tx1;
            y2 = ty1;
            dx *= -1;
            dy *= -1;
        }
        for y in 0..dy+1 {
            let x = x1 + y * dx / dy;
            set_px_safe(screen, color,
                PointScreen {
                    x: x as usize,
                    y: (y1 + y) as usize
                });
        }
    }
}