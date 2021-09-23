#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Color = Color { r:0,   g:0,   b:0   };
    pub const WHITE: Color = Color { r:255, g:255, b:255 };
    pub const RED:   Color = Color { r:255, g:0,   b:0   };
    pub const GREEN: Color = Color { r:0,   g:255, b:0   };
    pub const BLUE:  Color = Color { r:0,   g:0,   b:255 };
}

#[derive(Debug, Copy, Clone)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub fn magnitude(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
}

impl std::ops::Mul<f32> for Point3 {
    type Output = Point3;
    fn mul(self, x: f32) -> Self::Output {
        Point3 {
            x: self.x * x,
            y: self.y * x,
            z: self.z * x,
        }
    }
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
    pub x: isize,
    pub y: isize,
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