use crate::types::{Vec2, Vec2i, Vec3, Vec3i};
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl Vec2 {
    #[inline] pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
    #[inline] pub fn zero() -> Self { Self { x: 0.0, y: 0.0 } }
    #[inline] pub fn one() -> Self { Self { x: 1.0, y: 1.0 } }
    #[inline] pub fn splat(v: f32) -> Self { Self { x: v, y: v } }

    #[inline]
    pub fn from_angle(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        Self { x: c, y: s }
    }


    #[inline] pub fn dot(self, other: Self) -> f32 { self.x * other.x + self.y * other.y }
    #[inline] pub fn cross(self, other: Self) -> f32 { self.x * other.y - self.y * other.x }
    #[inline] pub fn length_squared(self) -> f32 { self.dot(self) }
    #[inline] pub fn length(self) -> f32 { self.length_squared().sqrt() }

    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > 1e-10 { self * (1.0 / len) } else { Self::zero() }
    }

    pub fn clamp_length(self, max: f32) -> Self {
        let len = self.length();
        if len > max && len > 1e-10 { self * (max / len) } else { self }
    }

    #[inline] pub fn distance(self, other: Self) -> f32 { (self - other).length() }
    #[inline] pub fn distance_squared(self, other: Self) -> f32 { (self - other).length_squared() }
    #[inline] pub fn angle(self) -> f32 { self.y.atan2(self.x) }

    #[inline]
    pub fn angle_to(self, other: Self) -> f32 {
        other.cross(self).atan2(self.dot(other))
    }

    pub fn rotate(self, angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        Self {
            x: self.x * c - self.y * s,
            y: self.x * s + self.y * c,
        }
    }

    #[inline] pub fn perp(self) -> Self { Self { x: -self.y, y: self.x } }

    #[inline]
    pub fn reflect(self, normal: Self) -> Self {
        self - normal * (2.0 * self.dot(normal))
    }

    #[inline]
    pub fn project_onto(self, onto: Self) -> Self {
        let d = onto.length_squared();
        if d < 1e-10 { return Self::zero(); }
        onto * (self.dot(onto) / d)
    }

    #[inline]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }

    #[inline]
    pub fn point_side(self, a: Vec2, b: Vec2) -> f32 {
        -((self.x - a.x) * (b.y - a.y) - (self.y - a.y) * (b.x - a.x))
    }

    pub fn intersect_segs(self, p1: Vec2, q0: Vec2, q1: Vec2) -> Option<Vec2> {
        let dp = p1 - self;
        let dq = q1 - q0;

        let d = dp.cross(dq);
        if d.abs() < 1e-8 {
            return None;
        }

        let diff = q0 - self;
        let t = diff.cross(dq) / d;
        let u = diff.cross(dp) / d;

        if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
            Some(self + dp * t)
        } else {
            None
        }
    }

    #[inline] pub fn abs(self)  -> Self { Self { x: self.x.abs(),   y: self.y.abs()   } }
    #[inline] pub fn floor(self)-> Self { Self { x: self.x.floor(), y: self.y.floor() } }
    #[inline] pub fn ceil(self) -> Self { Self { x: self.x.ceil(),  y: self.y.ceil()  } }
    #[inline] pub fn round(self)-> Self { Self { x: self.x.round(), y: self.y.round() } }

    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self { x: self.x.min(other.x), y: self.y.min(other.y) }
    }

    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self { x: self.x.max(other.x), y: self.y.max(other.y) }
    }

    #[inline]
    pub fn clamp(self, lo: Self, hi: Self) -> Self {
        self.max(lo).min(hi)
    }

    #[inline] pub fn min_component(self) -> f32 { self.x.min(self.y) }
    #[inline] pub fn max_component(self) -> f32 { self.x.max(self.y) }

    #[inline]
    pub fn approx_eq(self, other: Self, eps: f32) -> bool {
        (self.x - other.x).abs() < eps && (self.y - other.y).abs() < eps
    }
}

impl Add  for Vec2 { type Output = Self; fn add(self, o: Self) -> Self { Self { x: self.x + o.x, y: self.y + o.y } } }
impl Sub  for Vec2 { type Output = Self; fn sub(self, o: Self) -> Self { Self { x: self.x - o.x, y: self.y - o.y } } }
impl Neg  for Vec2 { type Output = Self; fn neg(self)           -> Self { Self { x: -self.x, y: -self.y } } }
impl Mul<f32> for Vec2 { type Output = Self; fn mul(self, s: f32) -> Self { Self { x: self.x * s, y: self.y * s } } }
impl Mul<Vec2> for f32 { type Output = Vec2; fn mul(self, v: Vec2) -> Vec2 { Vec2 { x: self * v.x, y: self * v.y } } }
impl Div<f32> for Vec2 { type Output = Self; fn div(self, s: f32) -> Self { self * (1.0 / s) } }

impl AddAssign for Vec2 { fn add_assign(&mut self, o: Self) { self.x += o.x; self.y += o.y; } }
impl SubAssign for Vec2 { fn sub_assign(&mut self, o: Self) { self.x -= o.x; self.y -= o.y; } }
impl MulAssign<f32> for Vec2 { fn mul_assign(&mut self, s: f32) { self.x *= s; self.y *= s; } }
impl DivAssign<f32> for Vec2 { fn div_assign(&mut self, s: f32) { *self *= 1.0 / s; } }

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.4}, {:.4})", self.x, self.y)
    }
}

impl Vec3 {
    #[inline] pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
    #[inline] pub fn zero() -> Self { Self { x: 0.0, y: 0.0, z: 0.0 } }
    #[inline] pub fn one()  -> Self { Self { x: 1.0, y: 1.0, z: 1.0 } }
    #[inline] pub fn splat(v: f32) -> Self { Self { x: v, y: v, z: v } }

    #[inline] pub fn from_xy(v: Vec2, z: f32) -> Self { Self { x: v.x, y: v.y, z } }

    #[inline] pub fn xy(self) -> Vec2 { Vec2 { x: self.x, y: self.y } }

    #[inline]
    pub fn dot(self, o: Self) -> f32 { self.x * o.x + self.y * o.y + self.z * o.z }

    #[inline]
    pub fn cross(self, o: Self) -> Self {
        Self {
            x: self.y * o.z - self.z * o.y,
            y: self.z * o.x - self.x * o.z,
            z: self.x * o.y - self.y * o.x,
        }
    }

    #[inline] pub fn length_squared(self) -> f32 { self.dot(self) }
    #[inline] pub fn length(self)          -> f32 { self.length_squared().sqrt() }

    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > 1e-10 { self * (1.0 / len) } else { Self::zero() }
    }

    pub fn clamp_length(self, max: f32) -> Self {
        let len = self.length();
        if len > max && len > 1e-10 { self * (max / len) } else { self }
    }

    #[inline] pub fn distance(self, o: Self)         -> f32 { (self - o).length() }
    #[inline] pub fn distance_squared(self, o: Self) -> f32 { (self - o).length_squared() }

    #[inline]
    pub fn reflect(self, normal: Self) -> Self {
        self - normal * (2.0 * self.dot(normal))
    }

    #[inline]
    pub fn project_onto(self, onto: Self) -> Self {
        let d = onto.length_squared();
        if d < 1e-10 { return Self::zero(); }
        onto * (self.dot(onto) / d)
    }

    #[inline]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
        }
    }

    #[inline] pub fn abs(self)  -> Self { Self { x: self.x.abs(),   y: self.y.abs(),   z: self.z.abs()   } }
    #[inline] pub fn floor(self)-> Self { Self { x: self.x.floor(), y: self.y.floor(), z: self.z.floor() } }
    #[inline] pub fn ceil(self) -> Self { Self { x: self.x.ceil(),  y: self.y.ceil(),  z: self.z.ceil()  } }
    #[inline] pub fn round(self)-> Self { Self { x: self.x.round(), y: self.y.round(), z: self.z.round() } }

    #[inline]
    pub fn min(self, o: Self) -> Self {
        Self { x: self.x.min(o.x), y: self.y.min(o.y), z: self.z.min(o.z) }
    }

    #[inline]
    pub fn max(self, o: Self) -> Self {
        Self { x: self.x.max(o.x), y: self.y.max(o.y), z: self.z.max(o.z) }
    }

    #[inline]
    pub fn clamp(self, lo: Self, hi: Self) -> Self { self.max(lo).min(hi) }

    #[inline] pub fn min_component(self) -> f32 { self.x.min(self.y).min(self.z) }
    #[inline] pub fn max_component(self) -> f32 { self.x.max(self.y).max(self.z) }

    #[inline]
    pub fn approx_eq(self, other: Self, eps: f32) -> bool {
        (self.x - other.x).abs() < eps
            && (self.y - other.y).abs() < eps
            && (self.z - other.z).abs() < eps
    }
}

impl Add  for Vec3 { type Output = Self; fn add(self, o: Self) -> Self { Self { x: self.x + o.x, y: self.y + o.y, z: self.z + o.z } } }
impl Sub  for Vec3 { type Output = Self; fn sub(self, o: Self) -> Self { Self { x: self.x - o.x, y: self.y - o.y, z: self.z - o.z } } }
impl Neg  for Vec3 { type Output = Self; fn neg(self)           -> Self { Self { x: -self.x, y: -self.y, z: -self.z } } }
impl Mul<f32> for Vec3 { type Output = Self; fn mul(self, s: f32) -> Self { Self { x: self.x * s, y: self.y * s, z: self.z * s } } }
impl Mul<Vec3> for f32 { type Output = Vec3; fn mul(self, v: Vec3) -> Vec3 { Vec3 { x: self * v.x, y: self * v.y, z: self * v.z } } }
impl Div<f32> for Vec3 { type Output = Self; fn div(self, s: f32) -> Self { self * (1.0 / s) } }

impl AddAssign for Vec3 { fn add_assign(&mut self, o: Self) { self.x += o.x; self.y += o.y; self.z += o.z; } }
impl SubAssign for Vec3 { fn sub_assign(&mut self, o: Self) { self.x -= o.x; self.y -= o.y; self.z -= o.z; } }
impl MulAssign<f32> for Vec3 { fn mul_assign(&mut self, s: f32) { self.x *= s; self.y *= s; self.z *= s; } }
impl DivAssign<f32> for Vec3 { fn div_assign(&mut self, s: f32) { *self *= 1.0 / s; } }

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.4}, {:.4}, {:.4})", self.x, self.y, self.z)
    }
}

impl Vec2i {
    #[inline] pub fn new(x: i32, y: i32) -> Self { Self { x, y } }
    #[inline] pub fn zero() -> Self { Self { x: 0, y: 0 } }
    #[inline] pub fn splat(v: i32) -> Self { Self { x: v, y: v } }

    #[inline] pub fn dot(self, o: Self) -> i32 { self.x * o.x + self.y * o.y }
    #[inline] pub fn cross(self, o: Self) -> i32 { self.x * o.y - self.y * o.x }

    #[inline] pub fn length_squared(self) -> i32 { self.dot(self) }
    #[inline] pub fn length(self) -> f32 { (self.length_squared() as f32).sqrt() }

    #[inline] pub fn abs(self) -> Self { Self { x: self.x.abs(), y: self.y.abs() } }
    #[inline] pub fn min(self, o: Self) -> Self { Self { x: self.x.min(o.x), y: self.y.min(o.y) } }
    #[inline] pub fn max(self, o: Self) -> Self { Self { x: self.x.max(o.x), y: self.y.max(o.y) } }

    #[inline] pub fn as_vec2(self) -> Vec2 { Vec2 { x: self.x as f32, y: self.y as f32 } }
}

impl Add  for Vec2i { type Output = Self; fn add(self, o: Self) -> Self { Self { x: self.x + o.x, y: self.y + o.y } } }
impl Sub  for Vec2i { type Output = Self; fn sub(self, o: Self) -> Self { Self { x: self.x - o.x, y: self.y - o.y } } }
impl Neg  for Vec2i { type Output = Self; fn neg(self)           -> Self { Self { x: -self.x, y: -self.y } } }
impl Mul<i32> for Vec2i { type Output = Self; fn mul(self, s: i32) -> Self { Self { x: self.x * s, y: self.y * s } } }

impl AddAssign for Vec2i { fn add_assign(&mut self, o: Self) { self.x += o.x; self.y += o.y; } }
impl SubAssign for Vec2i { fn sub_assign(&mut self, o: Self) { self.x -= o.x; self.y -= o.y; } }

impl fmt::Display for Vec2i {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Vec3i {
    #[inline] pub fn new(x: i32, y: i32, z: i32) -> Self { Self { x, y, z } }
    #[inline] pub fn zero() -> Self { Self { x: 0, y: 0, z: 0 } }
    #[inline] pub fn splat(v: i32) -> Self { Self { x: v, y: v, z: v } }

    #[inline] pub fn dot(self, o: Self) -> i32 { self.x * o.x + self.y * o.y + self.z * o.z }

    #[inline]
    pub fn cross(self, o: Self) -> Self {
        Self {
            x: self.y * o.z - self.z * o.y,
            y: self.z * o.x - self.x * o.z,
            z: self.x * o.y - self.y * o.x,
        }
    }

    #[inline] pub fn length_squared(self) -> i32 { self.dot(self) }
    #[inline] pub fn length(self) -> f32 { (self.length_squared() as f32).sqrt() }

    #[inline] pub fn abs(self) -> Self { Self { x: self.x.abs(), y: self.y.abs(), z: self.z.abs() } }
    #[inline] pub fn min(self, o: Self) -> Self { Self { x: self.x.min(o.x), y: self.y.min(o.y), z: self.z.min(o.z) } }
    #[inline] pub fn max(self, o: Self) -> Self { Self { x: self.x.max(o.x), y: self.y.max(o.y), z: self.z.max(o.z) } }

    #[inline] pub fn xy(self) -> Vec2i { Vec2i { x: self.x, y: self.y } }
    #[inline] pub fn as_vec3(self) -> Vec3 { Vec3 { x: self.x as f32, y: self.y as f32, z: self.z as f32 } }
}

impl Add  for Vec3i { type Output = Self; fn add(self, o: Self) -> Self { Self { x: self.x + o.x, y: self.y + o.y, z: self.z + o.z } } }
impl Sub  for Vec3i { type Output = Self; fn sub(self, o: Self) -> Self { Self { x: self.x - o.x, y: self.y - o.y, z: self.z - o.z } } }
impl Neg  for Vec3i { type Output = Self; fn neg(self)           -> Self { Self { x: -self.x, y: -self.y, z: -self.z } } }
impl Mul<i32> for Vec3i { type Output = Self; fn mul(self, s: i32) -> Self { Self { x: self.x * s, y: self.y * s, z: self.z * s } } }

impl AddAssign for Vec3i { fn add_assign(&mut self, o: Self) { self.x += o.x; self.y += o.y; self.z += o.z; } }
impl SubAssign for Vec3i { fn sub_assign(&mut self, o: Self) { self.x -= o.x; self.y -= o.y; self.z -= o.z; } }

impl fmt::Display for Vec3i {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<Vec2i> for Vec2  { fn from(v: Vec2i) -> Vec2  { v.as_vec2() } }
impl From<Vec3i> for Vec3  { fn from(v: Vec3i) -> Vec3  { v.as_vec3() } }

impl From<Vec2> for Vec2i {
    fn from(v: Vec2) -> Vec2i { Vec2i { x: v.x as i32, y: v.y as i32 } }
}
impl From<Vec3> for Vec3i {
    fn from(v: Vec3) -> Vec3i { Vec3i { x: v.x as i32, y: v.y as i32, z: v.z as i32 } }
}

impl From<(f32, f32)>       for Vec2  { fn from((x, y): (f32, f32))       -> Self { Self { x, y } } }
impl From<(i32, i32)>       for Vec2i { fn from((x, y): (i32, i32))       -> Self { Self { x, y } } }
impl From<(f32, f32, f32)>  for Vec3  { fn from((x, y, z): (f32, f32, f32)) -> Self { Self { x, y, z } } }
impl From<(i32, i32, i32)>  for Vec3i { fn from((x, y, z): (i32, i32, i32)) -> Self { Self { x, y, z } } }

impl From<Vec2>  for (f32, f32)       { fn from(v: Vec2)  -> Self { (v.x, v.y) } }
impl From<Vec2i> for (i32, i32)       { fn from(v: Vec2i) -> Self { (v.x, v.y) } }
impl From<Vec3>  for (f32, f32, f32)  { fn from(v: Vec3)  -> Self { (v.x, v.y, v.z) } }
impl From<Vec3i> for (i32, i32, i32)  { fn from(v: Vec3i) -> Self { (v.x, v.y, v.z) } }

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::{FRAC_PI_2};

    #[test] fn vec2_length()        { assert_eq!(Vec2::new(3.0, 4.0).length(), 5.0); }
    #[test] fn vec2_normalize()     { assert!((Vec2::new(0.0, 5.0).normalize().length() - 1.0).abs() < 1e-6); }
    #[test] fn vec2_zero_normalize(){ assert_eq!(Vec2::zero().normalize(), Vec2::zero()); }
    #[test] fn vec2_dot_perp()      { assert_eq!(Vec2::new(1.0, 0.0).dot(Vec2::new(0.0, 1.0)), 0.0); }
    #[test] fn vec2_cross()         { assert_eq!(Vec2::new(1.0, 0.0).cross(Vec2::new(0.0, 1.0)), 1.0); }

    #[test]
    fn vec2_from_angle() {
        let v = Vec2::from_angle(0.0);
        assert!(v.approx_eq(Vec2::new(1.0, 0.0), 1e-6));
        let v2 = Vec2::from_angle(FRAC_PI_2);
        assert!(v2.approx_eq(Vec2::new(0.0, 1.0), 1e-6));
    }

    #[test]
    fn vec2_rotate() {
        let v = Vec2::new(1.0, 0.0).rotate(FRAC_PI_2);
        assert!(v.approx_eq(Vec2::new(0.0, 1.0), 1e-6));
    }

    #[test]
    fn vec2_reflect() {
        let v = Vec2::new(1.0, -1.0);
        let n = Vec2::new(0.0, 1.0);
        assert!(v.reflect(n).approx_eq(Vec2::new(1.0, 1.0), 1e-6));
    }

    #[test]
    fn vec2_clamp_length() {
        let v = Vec2::new(3.0, 4.0); // length = 5
        let c = v.clamp_length(2.5);
        assert!((c.length() - 2.5).abs() < 1e-5);
        assert_eq!(v.clamp_length(10.0), v);
    }

    #[test]
    fn vec2_project() {
        let a = Vec2::new(3.0, 0.0);
        let b = Vec2::new(1.0, 1.0).normalize();
        let p = a.project_onto(b);
        assert!(p.approx_eq(Vec2::new(1.5, 1.5), 1e-5));
    }

    #[test]
    fn vec2_perp_dot_zero() {
        let v = Vec2::new(2.0, 3.0);
        assert_eq!(v.dot(v.perp()), 0.0);
    }

    #[test]
    fn vec2_lerp() {
        let a = Vec2::zero();
        let b = Vec2::one();
        assert_eq!(a.lerp(b, 0.5), Vec2::new(0.5, 0.5));
    }

    #[test]
    fn vec2_ops() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        assert_eq!(a + b, Vec2::new(4.0, 6.0));
        assert_eq!(b - a, Vec2::new(2.0, 2.0));
        assert_eq!(a * 2.0, Vec2::new(2.0, 4.0));
        assert_eq!(2.0 * a, Vec2::new(2.0, 4.0));
        assert_eq!(b / 2.0, Vec2::new(1.5, 2.0));
        assert_eq!(-a, Vec2::new(-1.0, -2.0));
    }

    #[test]
    fn vec2_assign_ops() {
        let mut v = Vec2::new(1.0, 2.0);
        v += Vec2::new(1.0, 1.0);
        assert_eq!(v, Vec2::new(2.0, 3.0));
        v -= Vec2::new(0.5, 0.5);
        assert!(v.approx_eq(Vec2::new(1.5, 2.5), 1e-6));
        v *= 2.0;
        assert!(v.approx_eq(Vec2::new(3.0, 5.0), 1e-6));
        v /= 2.0;
        assert!(v.approx_eq(Vec2::new(1.5, 2.5), 1e-6));
    }

    #[test]
    fn vec2_intersect_segs_basic() {
        // Two segments that cross at (0.5, 0.5)
        let p0 = Vec2::new(0.0, 0.0);
        let p1 = Vec2::new(1.0, 1.0);
        let q0 = Vec2::new(0.0, 1.0);
        let q1 = Vec2::new(1.0, 0.0);
        let hit = p0.intersect_segs(p1, q0, q1).expect("should intersect");
        assert!(hit.approx_eq(Vec2::new(0.5, 0.5), 1e-5));
    }

    #[test]
    fn vec2_intersect_segs_parallel() {
        let p0 = Vec2::new(0.0, 0.0);
        let p1 = Vec2::new(1.0, 0.0);
        let q0 = Vec2::new(0.0, 1.0);
        let q1 = Vec2::new(1.0, 1.0);
        assert!(p0.intersect_segs(p1, q0, q1).is_none());
    }

    #[test]
    fn vec2_intersect_segs_no_overlap() {
        let p0 = Vec2::new(0.0, 0.0);
        let p1 = Vec2::new(1.0, 0.0);
        let q0 = Vec2::new(2.0, -1.0);
        let q1 = Vec2::new(2.0,  1.0);
        assert!(p0.intersect_segs(p1, q0, q1).is_none());
    }

    #[test]
    fn vec2_component_ops() {
        let v = Vec2::new(-2.5, 3.7);
        assert_eq!(v.abs(), Vec2::new(2.5, 3.7));
        assert_eq!(v.floor(), Vec2::new(-3.0, 3.0));
        assert_eq!(v.ceil(),  Vec2::new(-2.0, 4.0));
        assert_eq!(Vec2::new(1.0, 3.0).min(Vec2::new(2.0, 2.0)), Vec2::new(1.0, 2.0));
        assert_eq!(Vec2::new(1.0, 3.0).max(Vec2::new(2.0, 2.0)), Vec2::new(2.0, 3.0));
    }

    #[test]
    fn vec2_display() {
        // Just check it doesn't panic
        let _ = format!("{}", Vec2::new(1.0, 2.0));
    }

    #[test]
    fn vec3_cross_unit() {
        let x = Vec3::new(1.0, 0.0, 0.0);
        let y = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(x.cross(y), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn vec3_normalize() {
        let v = Vec3::new(1.0, 0.0, 0.0).normalize();
        assert!((v.length() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn vec3_zero_normalize() { assert_eq!(Vec3::zero().normalize(), Vec3::zero()); }

    #[test]
    fn vec3_lerp() {
        let a = Vec3::zero();
        let b = Vec3::one();
        assert_eq!(a.lerp(b, 0.5), Vec3::new(0.5, 0.5, 0.5));
    }

    #[test]
    fn vec3_reflect() {
        let v = Vec3::new(1.0, -1.0, 0.0);
        let n = Vec3::new(0.0,  1.0, 0.0);
        assert!(v.reflect(n).approx_eq(Vec3::new(1.0, 1.0, 0.0), 1e-6));
    }

    #[test]
    fn vec3_from_xy_roundtrip() {
        let v2 = Vec2::new(3.0, 4.0);
        let v3 = Vec3::from_xy(v2, 5.0);
        assert_eq!(v3.xy(), v2);
        assert_eq!(v3.z, 5.0);
    }

    #[test]
    fn vec3_ops() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(a + b, Vec3::new(5.0, 7.0, 9.0));
        assert_eq!(b - a, Vec3::new(3.0, 3.0, 3.0));
        assert_eq!(a * 2.0, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(2.0 * a, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(b / 2.0, Vec3::new(2.0, 2.5, 3.0));
    }

    #[test]
    fn vec3_component_ops() {
        let v = Vec3::new(-1.5, 2.7, -0.1);
        assert_eq!(v.abs(), Vec3::new(1.5, 2.7, 0.1));
        assert_eq!(v.floor(), Vec3::new(-2.0, 2.0, -1.0));
    }

    #[test]
    fn vec2i_ops() {
        let a = Vec2i::new(1, 2);
        let b = Vec2i::new(3, 4);
        assert_eq!(a + b, Vec2i::new(4, 6));
        assert_eq!(b - a, Vec2i::new(2, 2));
        assert_eq!(a * 3,  Vec2i::new(3, 6));
        assert_eq!(-a,     Vec2i::new(-1, -2));
    }

    #[test]
    fn vec3i_cross() {
        let x = Vec3i::new(1, 0, 0);
        let y = Vec3i::new(0, 1, 0);
        assert_eq!(x.cross(y), Vec3i::new(0, 0, 1));
    }

    #[test]
    fn conversions() {
        let vi = Vec2i::new(3, 4);
        let vf: Vec2 = vi.into();
        assert_eq!(vf, Vec2::new(3.0, 4.0));

        let back: Vec2i = vf.into();
        assert_eq!(back, vi);

        let t: (f32, f32) = Vec2::new(1.0, 2.0).into();
        assert_eq!(t, (1.0, 2.0));
        let v: Vec2 = (5.0_f32, 6.0_f32).into();
        assert_eq!(v, Vec2::new(5.0, 6.0));
    }

    #[test]
    fn vec3_v3i_conversion() {
        let vi = Vec3i::new(1, 2, 3);
        let vf: Vec3 = vi.into();
        assert!(vf.approx_eq(Vec3::new(1.0, 2.0, 3.0), 1e-9));
    }
}
