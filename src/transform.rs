use crate::data::*;

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    //  Row-dominant.
    data: [f32; 16]
}

impl Transform {
    pub const IDENTITY: Transform = Transform {
        data: [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ],
    };

    pub fn scale(x: f32, y: f32, z:f32) -> Transform {
        Transform {
            data: [
                x,   0.0, 0.0, 0.0,
                0.0, y,   0.0, 0.0,
                0.0, 0.0, z,   0.0,
                0.0, 0.0, 0.0, 1.0,
            ]
        }
    }

    pub fn translate(x: f32, y: f32, z:f32) -> Transform {
        Transform {
            data: [
                0.0, 0.0, 0.0, x,
                0.0, 0.0, 0.0, y,
                0.0, 0.0, 0.0, z,
                0.0, 0.0, 0.0, 1.0,
            ]
        }
    }
}

impl std::ops::Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, rhs: Transform) -> Self::Output {
        let t1 = self.data;
        let t2 = rhs.data;
        Transform {
            data: [
                t1[0]*t2[0]  + t1[1]*t2[4]  + t1[2]*t2[8]   + t1[3]*t2[12],
                t1[0]*t2[1]  + t1[1]*t2[5]  + t1[2]*t2[9]   + t1[3]*t2[13],
                t1[0]*t2[2]  + t1[1]*t2[6]  + t1[2]*t2[10]  + t1[3]*t2[14],
                t1[0]*t2[3]  + t1[1]*t2[7]  + t1[2]*t2[11]  + t1[3]*t2[15],

                t1[4]*t2[0]  + t1[5]*t2[4]  + t1[6]*t2[8]   + t1[7]*t2[12],
                t1[4]*t2[1]  + t1[5]*t2[5]  + t1[6]*t2[9]   + t1[7]*t2[13],
                t1[4]*t2[2]  + t1[5]*t2[6]  + t1[6]*t2[10]  + t1[7]*t2[14],
                t1[4]*t2[3]  + t1[5]*t2[7]  + t1[6]*t2[11]  + t1[7]*t2[15],

                t1[8]*t2[0]  + t1[9]*t2[4]  + t1[10]*t2[8]  + t1[11]*t2[12],
                t1[8]*t2[1]  + t1[9]*t2[5]  + t1[10]*t2[9]  + t1[11]*t2[13],
                t1[8]*t2[2]  + t1[9]*t2[6]  + t1[10]*t2[10] + t1[11]*t2[14],
                t1[8]*t2[3]  + t1[9]*t2[7]  + t1[10]*t2[11] + t1[11]*t2[15],

                t1[12]*t2[0] + t1[13]*t2[4] + t1[14]*t2[8]  + t1[15]*t2[12],
                t1[12]*t2[1] + t1[13]*t2[5] + t1[14]*t2[9]  + t1[15]*t2[13],
                t1[12]*t2[2] + t1[13]*t2[6] + t1[14]*t2[10] + t1[15]*t2[14],
                t1[12]*t2[3] + t1[13]*t2[7] + t1[14]*t2[11] + t1[15]*t2[15],
            ]
        }
    }
}

impl std::ops::Mul<Point3> for Transform {
    type Output = Point3;

    fn mul(self, p: Point3) -> Self::Output {
        let m = &self.data;
        Point3 {
            x: m[0]*p.x  + m[1]*p.y  + m[2]*p.z  + m[3]*1.0,
            y: m[4]*p.x  + m[5]*p.y  + m[6]*p.z  + m[7]*1.0,
            z: m[8]*p.x  + m[9]*p.y  + m[10]*p.z + m[11]*1.0,
            //w: m[12]*p.x + m[13]*p.y + m[14]*p.z, + m[15]*p.w,
        }
    }
}