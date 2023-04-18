use std::ops::{Add, Index, IndexMut, Mul, MulAssign, Sub};

macro_rules! impl_vec {
    ($name:ident, $($field:ident),+) => {
        #[repr(C)]
        #[derive(Copy, Clone, Default, Debug)]
        pub struct $name {
            $(
                pub $field: f32,
            )+
        }

        impl $name {
            pub fn new($($field: f32,)+) -> Self {
                Self { $($field),+ }
            }

            pub fn zero() -> Self {
                Default::default()
            }

            pub fn as_array(&self) -> &[f32; std::mem::size_of::<Self>() / std::mem::size_of::<f32>()] {
                unsafe { std::mem::transmute(self) }
            }

            pub fn as_array_mut(&mut self) -> &mut [f32; std::mem::size_of::<Self>() / std::mem::size_of::<f32>()] {
                unsafe { std::mem::transmute(self) }
            }

            pub fn from_array(array: [f32; std::mem::size_of::<Self>() / std::mem::size_of::<f32>()]) -> Self {
                unsafe { std::mem::transmute(array) }
            }

            pub fn dot(&self, rhs: Self) -> f32 {
                0.0 $(+ self.$field * rhs.$field)+
            }

            pub fn normalized(&self) -> Self {
                let inv_len = 1.0 / self.dot(*self).sqrt();
                Self::new($(self.$field * inv_len),+)
            }
        }

        impl Index<usize> for $name {
            type Output = f32;

            fn index(&self, index: usize) -> &Self::Output {
                &self.as_array()[index]
            }
        }

        impl IndexMut<usize> for $name {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.as_array_mut()[index]
            }
        }

        impl Add<$name> for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self::new($(self.$field + rhs.$field),+)
            }
        }

        impl Sub<$name> for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self::new($(self.$field - rhs.$field),+)
            }
        }

        impl Mul<f32> for $name {
            type Output = Self;

            fn mul(self, rhs: f32) -> Self::Output {
                Self::new($(self.$field * rhs),+)
            }
        }

        impl Mul<$name> for f32 {
            type Output = $name;

            fn mul(self, rhs: $name) -> Self::Output {
                rhs * self
            }
        }
    };
}

impl_vec!(Vec2, x, y);
impl_vec!(Vec3, x, y, z);
impl_vec!(Vec4, x, y, z, w);

impl Vec2 {
    pub fn perp(&self) -> Self {
        Self::new(-self.y, self.x)
    }
}

impl Vec3 {
    pub fn cross(&self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl Vec4 {
    pub fn xy(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn xyz(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
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
                result[i][j] = self.row(i).dot(rhs.col(j));
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
            result[i] += self[i].dot(rhs);
        }
        result
    }
}
