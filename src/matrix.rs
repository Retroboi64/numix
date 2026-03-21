use crate::types::{Mat4x4, Vec4};
use std::ops::Mul;

impl Mat4x4<f32> {
    pub fn identity() -> Self {
        Self::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        Self::from([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotate(angle: f32, ax: f32, ay: f32, az: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        let t = 1.0 - c;
        Self::from([
            [
                t * ax * ax + c,
                t * ax * ay - s * az,
                t * ax * az + s * ay,
                0.0,
            ],
            [
                t * ax * ay + s * az,
                t * ay * ay + c,
                t * ay * az - s * ax,
                0.0,
            ],
            [
                t * ax * az - s * ay,
                t * ay * az + s * ax,
                t * az * az + c,
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn perspective(fovy_rad: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fovy_rad / 2.0).tan();
        let nf = 1.0 / (near - far);
        Self::from([
            [f / aspect, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (far + near) * nf, 2.0 * far * near * nf],
            [0.0, 0.0, -1.0, 0.0],
        ])
    }

    pub fn as_col_major(&self) -> [f32; 16] {
        let m = &self.mat;
        [
            m[0].x, m[1].x, m[2].x, m[3].x, m[0].y, m[1].y, m[2].y, m[3].y, m[0].z, m[1].z, m[2].z,
            m[3].z, m[0].w, m[1].w, m[2].w, m[3].w,
        ]
    }
}

impl Mul for Mat4x4<f32> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let col = |m: &Mat4x4<f32>, c: usize| Vec4 {
            x: m.mat[0].nth(c),
            y: m.mat[1].nth(c),
            z: m.mat[2].nth(c),
            w: m.mat[3].nth(c),
        };

        let dot4 = |r: Vec4<f32>, c: Vec4<f32>| r.x * c.x + r.y * c.y + r.z * c.z + r.w * c.w;

        Mat4x4 {
            mat: [
                Vec4 {
                    x: dot4(self.mat[0], col(&rhs, 0)),
                    y: dot4(self.mat[0], col(&rhs, 1)),
                    z: dot4(self.mat[0], col(&rhs, 2)),
                    w: dot4(self.mat[0], col(&rhs, 3)),
                },
                Vec4 {
                    x: dot4(self.mat[1], col(&rhs, 0)),
                    y: dot4(self.mat[1], col(&rhs, 1)),
                    z: dot4(self.mat[1], col(&rhs, 2)),
                    w: dot4(self.mat[1], col(&rhs, 3)),
                },
                Vec4 {
                    x: dot4(self.mat[2], col(&rhs, 0)),
                    y: dot4(self.mat[2], col(&rhs, 1)),
                    z: dot4(self.mat[2], col(&rhs, 2)),
                    w: dot4(self.mat[2], col(&rhs, 3)),
                },
                Vec4 {
                    x: dot4(self.mat[3], col(&rhs, 0)),
                    y: dot4(self.mat[3], col(&rhs, 1)),
                    z: dot4(self.mat[3], col(&rhs, 2)),
                    w: dot4(self.mat[3], col(&rhs, 3)),
                },
            ],
        }
    }
}

impl Vec4<f32> {
    fn nth(self, i: usize) -> f32 {
        match i {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            3 => self.w,
            _ => panic!("Vec4 index out of range"),
        }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_mul_is_identity() {
        let i = Mat4x4::identity();
        let result = i * i;
        assert_eq!(result, i);
    }

    #[test]
    fn col_major_identity_is_flat_diagonal() {
        let cols = Mat4x4::<f32>::identity().as_col_major();
        #[rustfmt::skip]
        let expected = [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];
        assert_eq!(cols, expected);
    }

    #[test]
    fn translate_moves_origin() {
        let m = Mat4x4::translate(1.0, 2.0, 3.0);
        assert_eq!(m.mat[0].w, 1.0);
        assert_eq!(m.mat[1].w, 2.0);
        assert_eq!(m.mat[2].w, 3.0);
    }

    #[test]
    fn rotate_y_90_maps_x_to_neg_z() {
        use std::f32::consts::FRAC_PI_2;
        let m = Mat4x4::rotate(FRAC_PI_2, 0.0, 1.0, 0.0);
        // Row-major Y rotation: row0=[cos,0,sin,0], row2=[-sin,0,cos,0]
        // At 90°: cos≈0, sin≈1
        assert!((m.mat[0].x).abs() < 1e-6); // cos ≈ 0
        assert!((m.mat[0].z - 1.0).abs() < 1e-6); // sin ≈ 1
        assert!((m.mat[2].x - -1.0).abs() < 1e-6); // -sin ≈ -1
        assert!((m.mat[2].z).abs() < 1e-6); // cos ≈ 0
    }

    #[test]
    fn perspective_w_row_is_neg_z() {
        // The -1 that feeds -Z into W must sit at row3, col2 in row-major.
        let p = Mat4x4::perspective(45.0_f32.to_radians(), 1.0, 0.1, 100.0);
        assert_eq!(p.mat[3].z, -1.0);
        assert_eq!(p.mat[3].x, 0.0);
        assert_eq!(p.mat[3].w, 0.0);
    }
}
