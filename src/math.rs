use std::ops::{Index, IndexMut, Mul, MulAssign};

#[derive(Copy, Clone, Default, Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn from_array(array: [f32; 4]) -> Self {
        Self {
            x: array[0],
            y: array[1],
            z: array[2],
            w: array[3],
        }
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("invalid index"),
        }
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("invalid index"),
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Mat4 {
    rows: [Vec4; 4],
}

impl Mat4 {
    pub fn new(x: [f32; 4], y: [f32; 4], z: [f32; 4], w: [f32; 4]) -> Self {
        Self {
            rows: [
                Vec4::from_array(x),
                Vec4::from_array(y),
                Vec4::from_array(z),
                Vec4::from_array(w),
            ],
        }
    }

    pub fn identity() -> Self {
        Self::new(
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    pub fn row(&self, row: usize) -> Vec4 {
        self.rows[row]
    }

    pub fn col(&self, col: usize) -> Vec4 {
        Vec4::new(
            self.rows[0][col],
            self.rows[1][col],
            self.rows[2][col],
            self.rows[3][col],
        )
    }
}

impl Index<usize> for Mat4 {
    type Output = Vec4;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        let mut result = Mat4::default();
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self.row(i).dot(&rhs.col(j));
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

#[allow(clippy::suspicious_arithmetic_impl)]
impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        let mut result = Vec4::default();
        for i in 0..4 {
            result[i] += self[i].dot(&rhs);
        }
        result
    }
}
