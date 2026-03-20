use crate::types::{Mat3x4, Mat4x4, Vec2, Vec3, Vec4};
use std::fmt;

#[allow(unused_imports)]
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Scalar:
    Copy
    + Default
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Neg<Output = Self>
{
}

impl<T> Scalar for T where
    T: Copy
        + Default
        + PartialEq
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
{
}

// Vec2<T>
impl<T: Scalar> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn scale(self, scalar: T) -> Self {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn length_sq(self) -> T {
        self.dot(self)
    }
}

impl<T: Scalar> Add for Vec2<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Scalar> Sub for Vec2<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Scalar> Neg for Vec2<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Scalar> Mul<T> for Vec2<T> {
    type Output = Self;
    fn mul(self, scalar: T) -> Self {
        self.scale(scalar)
    }
}

impl<T: Scalar + AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Scalar + SubAssign> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T: Scalar + fmt::Display> fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Vec3<T>
impl<T: Scalar> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn scale(self, scalar: T) -> Self {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn length_sq(self) -> T {
        self.dot(self)
    }

    pub fn cross(self, other: Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl<T: Scalar> Add for Vec3<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Scalar> Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Scalar> Neg for Vec3<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Scalar> Mul<T> for Vec3<T> {
    type Output = Self;
    fn mul(self, scalar: T) -> Self {
        self.scale(scalar)
    }
}

impl<T: Scalar + AddAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: Scalar + SubAssign> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: Scalar + fmt::Display> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// Vec4<T>
impl<T: Scalar> Vec4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn scale(self, scalar: T) -> Self {
        Vec4 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }

    pub fn length_sq(self) -> T {
        self.dot(self)
    }

    pub fn xyz(self) -> Vec3<T> {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl<T: Scalar> Add for Vec4<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<T: Scalar> Sub for Vec4<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<T: Scalar> Neg for Vec4<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Vec4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl<T: Scalar> Mul<T> for Vec4<T> {
    type Output = Self;
    fn mul(self, scalar: T) -> Self {
        self.scale(scalar)
    }
}

impl<T: Scalar + fmt::Display> fmt::Display for Vec4<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

// Mat4x4<T> — basic row access and Vec4 multiply
impl<T: Scalar> Mat4x4<T> {
    pub fn mul_vec4(self, v: Vec4<T>) -> Vec4<T> {
        Vec4 {
            x: self.mat[0].dot(v),
            y: self.mat[1].dot(v),
            z: self.mat[2].dot(v),
            w: self.mat[3].dot(v),
        }
    }
}

// Mat3x4<T> — 4 columns of Vec3 rows (e.g. a 3×4 transform matrix)
impl<T: Scalar> Mat3x4<T> {
    pub fn mul_vec4(self, v: Vec4<T>) -> Vec3<T> {
        Vec3 {
            x: self.mat[0].dot(v.xyz()) + self.mat[3].x * v.w,
            y: self.mat[1].dot(v.xyz()) + self.mat[3].y * v.w,
            z: self.mat[2].dot(v.xyz()) + self.mat[3].z * v.w,
        }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec2_add_operator() {
        let a = Vec2::new(1, 2);
        let b = Vec2::new(3, 4);
        assert_eq!(a + b, Vec2::new(4, 6));
    }

    #[test]
    fn vec2_sub_operator() {
        let a = Vec2::new(5, 7);
        let b = Vec2::new(2, 3);
        assert_eq!(a - b, Vec2::new(3, 4));
    }

    #[test]
    fn vec2_neg() {
        let v = Vec2::new(1, -2);
        assert_eq!(-v, Vec2::new(-1, 2));
    }

    #[test]
    fn vec2_scale() {
        let v = Vec2::new(2, 3);
        assert_eq!(v * 4, Vec2::new(8, 12));
    }

    #[test]
    fn vec2_dot() {
        let a = Vec2::new(1, 2);
        let b = Vec2::new(3, 4);
        assert_eq!(a.dot(b), 11); // 1*3 + 2*4
    }

    #[test]
    fn vec2_length_sq() {
        let v = Vec2::new(3, 4);
        assert_eq!(v.length_sq(), 25); // 9 + 16
    }

    #[test]
    fn vec2_add_assign() {
        let mut v = Vec2::new(1, 2);
        v += Vec2::new(3, 4);
        assert_eq!(v, Vec2::new(4, 6));
    }

    #[test]
    fn vec3_add_operator() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(4, 5, 6);
        assert_eq!(a + b, Vec3::new(5, 7, 9));
    }

    #[test]
    fn vec3_dot_includes_z() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(4, 5, 6);
        assert_eq!(a.dot(b), 32); // 1*4 + 2*5 + 3*6 = 4+10+18
    }

    #[test]
    fn vec3_cross_product() {
        let a = Vec3::new(1, 0, 0);
        let b = Vec3::new(0, 1, 0);
        assert_eq!(a.cross(b), Vec3::new(0, 0, 1)); // right-hand rule
    }

    #[test]
    fn vec3_cross_anticommutative() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(4, 5, 6);
        assert_eq!(a.cross(b), -b.cross(a));
    }

    #[test]
    fn vec3_length_sq() {
        let v = Vec3::new(1, 2, 2);
        assert_eq!(v.length_sq(), 9); // 1+4+4
    }

    #[test]
    fn vec4_dot() {
        let a = Vec4::new(1, 2, 3, 4);
        let b = Vec4::new(1, 0, 0, 0);
        assert_eq!(a.dot(b), 1);
    }

    #[test]
    fn vec4_xyz_drops_w() {
        let v = Vec4::new(1, 2, 3, 99);
        assert_eq!(v.xyz(), Vec3::new(1, 2, 3));
    }

    #[test]
    fn vec4_add_sub_roundtrip() {
        let a = Vec4::new(1, 2, 3, 4);
        let b = Vec4::new(5, 6, 7, 8);
        assert_eq!((a + b) - b, a);
    }
}
