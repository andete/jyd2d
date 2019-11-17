use std::ops;
use crate::vector2::Vector2;
use crate::vector3::Vector3;
use std::f64::consts::PI;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Matrix3 {
    pub m11: f64,
    pub m12: f64,
    pub m13: f64,
    pub m21: f64,
    pub m22: f64,
    pub m23: f64,
    pub m31: f64,
    pub m32: f64,
    pub m33: f64,
}

impl Matrix3 {
    pub fn builder() -> Matrix3Build {
        Matrix3Build { result: Matrix3::unit() }
    }

    pub fn zero() -> Matrix3 {
        Matrix3 {
            m11: 0.0,
            m12: 0.0,
            m13: 0.0,
            m21: 0.0,
            m22: 0.0,
            m23: 0.0,
            m31: 0.0,
            m32: 0.0,
            m33: 0.0,
        }
    }

    pub fn unit() -> Matrix3 {
        Matrix3 {
            m11: 1.0,
            m22: 1.0,
            m33: 1.0,
            ..Matrix3::zero()
        }
    }


    pub fn rotate(a: f64) -> Matrix3 {
        let ar = a * PI / 180.0;
        Matrix3 {
            m11: ar.cos(),
            m12: ar.sin(),
            m21: -ar.sin(),
            m22: ar.cos(),
            ..Matrix3::unit()
        }
    }

    pub fn translate(v: Vector2) -> Matrix3 {
        Matrix3 {
            m31: v.x,
            m32: v.y,
            ..Matrix3::unit()
        }
    }

    pub fn transpose(self) -> Matrix3 {
        Matrix3 {
            m11: self.m11,
            m12: self.m21,
            m13: self.m31,
            m21: self.m12,
            m22: self.m22,
            m23: self.m32,
            m31: self.m13,
            m32: self.m23,
            m33: self.m33,
        }
    }

    pub fn flip_x(flip_x: bool) -> Matrix3 {
        if !flip_x {
            Matrix3::unit()
        } else {
            Matrix3 {
                m11: -1.0,
                ..
                Matrix3::unit()
            }
        }
    }

    pub fn flip_y(flip_y: bool) -> Matrix3 {
        if !flip_y {
            Matrix3::unit()
        } else {
            Matrix3 {
                m22: -1.0,
                ..
                Matrix3::unit()
            }
        }
    }

    pub fn scale(sx: f64, sy: f64) -> Matrix3 {
        Matrix3 {
            m11: sx,
            m22: sy,
            ..Matrix3::unit()
        }
    }

    pub fn determinant(self) -> f64 {
        self.m11 * self.m22 * self.m33 + self.m12 * self.m23 * self.m31 + self.m13 * self.m21 * self.m32
            - self.m12 * self.m22 * self.m31 - self.m12 * self.m21 * self.m33 - self.m11 * self.m23 * self.m32
    }
}

impl ops::Mul<f64> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: f64) -> Matrix3 {
        Matrix3 {
            m11: self.m11 * rhs,
            m12: self.m12 * rhs,
            m13: self.m13 * rhs,
            m21: self.m21 * rhs,
            m22: self.m22 * rhs,
            m23: self.m23 * rhs,
            m31: self.m31 * rhs,
            m32: self.m32 * rhs,
            m33: self.m33 * rhs,
        }
    }
}

impl ops::Mul<Matrix3> for f64 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Matrix3 {
        Matrix3 {
            m11: rhs.m11 * self,
            m12: rhs.m12 * self,
            m13: rhs.m13 * self,
            m21: rhs.m21 * self,
            m22: rhs.m22 * self,
            m23: rhs.m23 * self,
            m31: rhs.m31 * self,
            m32: rhs.m32 * self,
            m33: rhs.m23 * self,
        }
    }
}

impl ops::Mul<Matrix3> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Matrix3 {
        Matrix3 {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21 + self.m13 * rhs.m31,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22 + self.m13 * rhs.m32,
            m13: self.m11 * rhs.m13 + self.m12 * rhs.m23 + self.m13 * rhs.m33,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21 + self.m23 * rhs.m31,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22 + self.m23 * rhs.m32,
            m23: self.m21 * rhs.m13 + self.m22 * rhs.m23 + self.m23 * rhs.m33,
            m31: self.m31 * rhs.m11 + self.m32 * rhs.m21 + self.m33 * rhs.m31,
            m32: self.m31 * rhs.m12 + self.m32 * rhs.m22 + self.m33 * rhs.m32,
            m33: self.m31 * rhs.m13 + self.m32 * rhs.m23 + self.m33 * rhs.m33,
        }
    }
}

impl ops::MulAssign<Matrix3> for Matrix3 {
    fn mul_assign(&mut self, rhs: Matrix3) {
        *self = *self * rhs;
    }
}

impl ops::Mul<Matrix3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Matrix3) -> Vector3 {
        Vector3 {
            x: self.x * rhs.m11 + self.y * rhs.m21 + self.z * rhs.m31,
            y: self.x * rhs.m12 + self.y * rhs.m22 + self.z * rhs.m32,
            z: self.x * rhs.m13 + self.y * rhs.m23 + self.z * rhs.m33,

        }
    }
}


pub struct Matrix3Build {
    result: Matrix3
}

impl Matrix3Build {
    pub fn rotate(mut self, a: f64) -> Matrix3Build {
        self.result *= Matrix3::rotate(a);
        self
    }
    pub fn translate(mut self, v: Vector2) -> Matrix3Build {
        self.result *= Matrix3::translate(v);
        self
    }

    pub fn flip_x(mut self, flip_x: bool) -> Matrix3Build {
        self.result *= Matrix3::flip_x(flip_x);
        self
    }

    pub fn flip_y(mut self, flip_y: bool) -> Matrix3Build {
        self.result *= Matrix3::flip_x(flip_y);
        self
    }

    pub fn scale(mut self, sx: f64, sy: f64) -> Matrix3Build {
        self.result *= Matrix3::scale(sx, sy);
        self
    }

    pub fn build(self) -> Matrix3 {
        self.result
    }
}
