#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Mat4x4<T> {
    pub mat: [Vec4<T>; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Mat3x4<T> {
    pub mat: [Vec3<T>; 4],
}
