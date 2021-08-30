use crate::data::*;

struct Screen {
    pub w: usize,
    pub h: usize,
    pub data: Vec<Color>,
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
    if (x <  min) { return min; }
    if (x >= max) { return max; }
    return x;
}

fn draw_point(screen: &mut Screen, point: PointScreen, w: usize, color: Color) {
    let left  = clamp(point.x - w, 0, screen.w-1);
    let right = clamp(point.x + w, 0, screen.w-1);
    let top = clamp(point.y - w, 0, screen.h-1);
    let bot = clamp(point.y + w, 0, screen.h-1);

    for x in left..right {
        for y in top..bot {
            set_px_unsafe(screen, color, PointScreen { x: x, y:y });
        }
    }
}