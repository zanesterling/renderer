#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}

// X from left to right,
// Y from top to bottom.
#[derive(Debug, Copy, Clone)]
pub struct PointScreen {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct Line {
    pub p1: Point3,
    pub p2: Point3,
}

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub p1: Point3,
    pub p2: Point3,
    pub p3: Point3,
}