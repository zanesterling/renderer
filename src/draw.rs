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

pub fn draw_triangle(
    screen: &mut Screen,
    p1: PointScreen,
    p2: PointScreen,
    p3: PointScreen,
    color: Color
) {
    let (bot, mid, top) = {
        let (mut p1, mut p2, mut p3) = (p1, p2, p3);
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
            x: (top.x as isize + dx * i / dy) as usize,
            y: mid.y
        }
    };

    fill_flat_top_tri(screen, bot, mid, mid2, color);
    fill_flat_bot_tri(screen, top, mid, mid2, color);

    draw_line(screen, bot, top, color);
    draw_line(screen, bot, mid, color);
    draw_line(screen, mid, top, color);
    draw_line(screen, mid, mid2, color);

    draw_point(screen, top, 3, Color::RED);
    draw_point(screen, mid, 3, Color::GREEN);
    draw_point(screen, mid2, 3, Color::GREEN);
    draw_point(screen, bot, 3, Color::BLUE);
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
    let dy1 = bot.y as isize - mid.y  as isize;
    let dy2 = bot.y as isize - mid2.y as isize;

    for i in 0..dy1+1 {
        let lt_x = mid.x  as isize + i * dx1 / dy1;
        let rt_x = mid2.x as isize + i * dx2 / dy2;
        fill_row(screen, mid.y + i as usize, lt_x as usize, rt_x as usize, color);
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
    let dy1 = mid.y  as isize - top.y as isize;
    let dy2 = mid2.y as isize - top.y as isize;

    for i in 0..dy1+1 {
        let lt_x = top.x as isize + i * dx1 / dy1;
        let rt_x = top.x as isize + i * dx2 / dy2;
        fill_row(screen, top.y + i as usize, lt_x as usize, rt_x as usize, color);
    }
}

fn fill_row(
    screen: &mut Screen,
    y: usize,
    x1: usize,
    mut x2: usize,
    color: Color
) {
    if y  >= screen.h { return; }
    if x1 >= screen.w { return; }
    if x2 >= screen.w { x2 = screen.w; }

    for x in x1..x2+1 {
        set_px_unsafe(screen, color, PointScreen { x: x, y: y });
    }
}