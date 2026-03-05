use crate::types::{Vec2, Vec3};
use std::ops::{Add, Mul, Neg, Sub};

// --- Vec2 ---

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn one() -> Self {
        Self { x: 1.0, y: 1.0 }
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            Self::zero()
        }
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }

    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn one() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            Self::zero()
        }
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
        }
    }

    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec2_length() {
        let v = Vec2::new(3.0, 4.0);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn vec2_normalize() {
        let v = Vec2::new(0.0, 5.0).normalize();
        assert!((v.length() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn vec2_dot() {
        let a = Vec2::new(1.0, 0.0);
        let b = Vec2::new(0.0, 1.0);
        assert_eq!(a.dot(b), 0.0);
    }

    #[test]
    fn vec3_cross() {
        let x = Vec3::new(1.0, 0.0, 0.0);
        let y = Vec3::new(0.0, 1.0, 0.0);
        let z = x.cross(y);
        assert_eq!(z, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn vec3_normalize() {
        let v = Vec3::new(1.0, 0.0, 0.0).normalize();
        assert!((v.length() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn vec3_lerp() {
        let a = Vec3::zero();
        let b = Vec3::one();
        let mid = a.lerp(b, 0.5);
        assert_eq!(mid, Vec3::new(0.5, 0.5, 0.5));
    }
}

impl Vec2 {
    pub fn rotate(self, angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        Self {
            x: self.x * c - self.y * s,
            y: self.x * s + self.y * c,
        }
    }

    pub fn point_side(self, a: Vec2, b: Vec2) -> f32 {
        -((self.x - a.x) * (b.y - a.y) - (self.y - a.y) * (b.x - a.x))
    }

    pub fn intersect_segs(self, p1: Vec2, q0: Vec2, q1: Vec2) -> Option<Vec2> {
        let d = (self.x - p1.x) * (q0.y - q1.y) - (self.y - p1.y) * (q0.x - q1.x);

        if d.abs() < 1e-6 {
            return None;
        }

        let t = ((self.x - q0.x) * (q0.y - q1.y) - (self.y - q0.y) * (q0.x - q1.x)) / d;
        let u = ((self.x - q0.x) * (self.y - p1.y) - (self.y - q0.y) * (self.x - p1.x)) / d;

        if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
            Some(Vec2 {
                x: self.x + t * (p1.x - self.x),
                y: self.y + t * (p1.y - self.y),
            })
        } else {
            None
        }
    }

    pub fn perp(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
}
