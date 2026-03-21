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
    pub mat: [Vec4<T>; 3],
}

impl<T: Copy + Default> From<[[T; 4]; 4]> for Mat4x4<T> {
    fn from(a: [[T; 4]; 4]) -> Self {
        Self {
            mat: a.map(|r| Vec4 {
                x: r[0],
                y: r[1],
                z: r[2],
                w: r[3],
            }),
        }
    }
}

impl<T: Copy + Default> From<[[T; 4]; 3]> for Mat3x4<T> {
    fn from(a: [[T; 4]; 3]) -> Self {
        Self {
            mat: a.map(|r| Vec4 {
                x: r[0],
                y: r[1],
                z: r[2],
                w: r[3],
            }),
        }
    }
}
