use std::ops::{Mul, MulAssign};

#[derive(Copy, Clone, Default, Debug)]
pub struct Vec4(pub [f32; 4]);

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self([x, y, z, w])
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Mat4([Vec4; 4]);

impl Mat4 {
    pub fn new(x: [f32; 4], y: [f32; 4], z: [f32; 4], w: [f32; 4]) -> Self {
        Self([Vec4(x), Vec4(y), Vec4(z), Vec4(w)])
    }

    pub fn identity() -> Self {
        Self::new(
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        let mut result = Mat4::default();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.0[i].0[j] += self.0[i].0[k] * rhs.0[k].0[j];
                }
            }
        }
        result
    }
}

impl MulAssign<Mat4> for Mat4 {
    fn mul_assign(&mut self, rhs: Mat4) {
        *self = *self * rhs;
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        let mut result = Vec4::default();
        for i in 0..4 {
            for j in 0..4 {
                result.0[i] += self.0[i].0[j] * rhs.0[j];
            }
        }
        result
    }
}
