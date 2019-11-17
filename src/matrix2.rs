use std::ops;
use crate::vector2::Vector2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix2 {
    pub m11: f64,
    pub m12: f64,
    pub m21: f64,
    pub m22: f64,
}

impl Matrix2 {
    pub fn new(m11: f64, m12: f64, m21: f64, m22: f64) -> Matrix2 {
        Matrix2 { m11, m12, m21, m22 }
    }

    pub fn builder() -> Matrix2Build {
        Matrix2Build { result: Matrix2::unit() }
    }

    pub fn unit() -> Matrix2 {
        Matrix2 {
            m11: 1.0,
            m12: 0.0,
            m21: 0.0,
            m22: 1.0,
        }
    }

    pub fn transpose(self) -> Matrix2 {
        Matrix2 {
            m11: self.m11,
            m12: self.m21,
            m21: self.m12,
            m22: self.m22,
        }
    }

    pub fn rotate(a: f64) -> Matrix2 {
        Matrix2 {
            m11: a.cos(),
            m12: a.sin(),
            m21: -a.sin(),
            m22: a.cos(),
        }
    }

    pub fn scale(kx: f64, ky: f64) -> Matrix2 {
        Matrix2 {
            m11: kx,
            m12: 0.0,
            m21: 0.0,
            m22: ky,
        }
    }

    pub fn scale_n(n: Vector2, k: f64) -> Matrix2 {
        Matrix2 {
            m11: 1.0 + (k - 1.0) * n.x * n.x,
            m12: (k - 1.0) * n.x * n.y,
            m21: (k - 1.0) * n.x * n.y,
            m22: 1.0 + (k - 1.0) * n.y * n.y,
        }
    }

    pub fn project_x() -> Matrix2 {
        Matrix2 {
            m11: 1.0,
            m12: 0.0,
            m21: 0.0,
            m22: 0.0,
        }
    }

    pub fn project_y() -> Matrix2 {
        Matrix2 {
            m11: 0.0,
            m12: 0.0,
            m21: 0.0,
            m22: 1.0,
        }
    }

    pub fn assert_approx_eq(self, rhs: Matrix2) {
        use assert_approx_eq::assert_approx_eq;
        assert_approx_eq!(self.m11, rhs.m11);
        assert_approx_eq!(self.m12, rhs.m12);
        assert_approx_eq!(self.m21, rhs.m21);
        assert_approx_eq!(self.m22, rhs.m22);
    }

    pub fn project_n(n: Vector2) -> Matrix2 {
        Matrix2::scale_n(n, 0.0)
    }

    pub fn reflect_n(n: Vector2) -> Matrix2 {
        Matrix2::scale_n(n, -1.0)
    }

    pub fn shear_x(s: f64) -> Matrix2 {
        Matrix2 {
            m11: 1.0,
            m12: 0.0,
            m21: s,
            m22: 1.0,
        }
    }

    pub fn shear_y(s: f64) -> Matrix2 {
        Matrix2 {
            m11: 1.0,
            m12: s,
            m21: 0.0,
            m22: 1.0,
        }
    }

    pub fn determinant(self) -> f64 {
        self.m11 * self.m22 - self.m12 * self.m21
    }

    pub fn adjoint(self) -> Matrix2 {
        Matrix2 {
            m11: self.m22,
            m12: -self.m12,
            m21: -self.m21,
            m22: self.m11,
        }
    }

    pub fn inverse(self) -> Matrix2 {
        self.adjoint() / self.determinant()
    }
}

impl ops::Add<Matrix2> for Matrix2 {
    type Output = Matrix2;

    fn add(self, rhs: Matrix2) -> Matrix2 {
        Matrix2 { m11: self.m11 + rhs.m11, m12: self.m12 + rhs.m12, m21: self.m21 + rhs.m21, m22: self.m22 + rhs.m22 }
    }
}

impl ops::Sub<Matrix2> for Matrix2 {
    type Output = Matrix2;

    fn sub(self, rhs: Matrix2) -> Matrix2 {
        self + (-rhs)
    }
}

impl ops::Mul<f64> for Matrix2 {
    type Output = Matrix2;

    fn mul(self, rhs: f64) -> Matrix2 {
        Matrix2 { m11: self.m11 * rhs, m12: self.m12 * rhs, m21: self.m21 * rhs, m22: self.m22 * rhs }
    }
}

impl ops::Neg for Matrix2 {
    type Output = Matrix2;

    fn neg(self) -> Matrix2 {
        Matrix2 { m11: -self.m11, m12: -self.m12, m21: -self.m21, m22: -self.m22 }
    }
}

impl ops::Div<f64> for Matrix2 {
    type Output = Matrix2;

    fn div(self, rhs: f64) -> Matrix2 {
        Matrix2 { m11: self.m11 / rhs, m12: self.m12 / rhs, m21: self.m21 / rhs, m22: self.m22 / rhs }
    }
}

impl ops::Mul<Matrix2> for f64 {
    type Output = Matrix2;

    fn mul(self, rhs: Matrix2) -> Matrix2 {
        Matrix2 { m11: rhs.m11 * self, m12: rhs.m12 * self, m21: rhs.m21 * self, m22: rhs.m22 * self }
    }
}

impl ops::Mul<Vector2> for Matrix2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.m11 * rhs.x + self.m12 * rhs.y,
            y: self.m21 * rhs.x + self.m22 * rhs.y,

        }
    }
}

impl ops::Mul<Matrix2> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Matrix2) -> Vector2 {
        Vector2 {
            x: self.x * rhs.m11 + self.y * rhs.m21,
            y: self.x * rhs.m12 + self.y * rhs.m22,

        }
    }
}


impl ops::Mul<Matrix2> for Matrix2 {
    type Output = Matrix2;

    fn mul(self, rhs: Matrix2) -> Matrix2 {
        Matrix2 {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21,
            m12: self.m11 * rhs.m21 + self.m12 * rhs.m22,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22,
        }
    }
}

impl ops::MulAssign<Matrix2> for Matrix2 {
    fn mul_assign(&mut self, rhs: Matrix2) {
        *self = *self * rhs;
    }
}

pub struct Matrix2Build {
    result: Matrix2
}

impl Matrix2Build {
    pub fn rotate(mut self, a: f64) -> Matrix2Build {
        self.result *= Matrix2::rotate(a);
        self
    }

    pub fn scale(mut self, kx: f64, ky: f64) -> Matrix2Build {
        self.result *= Matrix2::scale(kx, ky);
        self
    }

    pub fn scale_n(mut self, n: Vector2, k: f64) -> Matrix2Build {
        self.result *= Matrix2::scale_n(n, k);
        self
    }

    pub fn project_x(mut self) -> Matrix2Build {
        self.result *= Matrix2::project_x();
        self
    }

    pub fn project_y(mut self) -> Matrix2Build {
        self.result *= Matrix2::project_y();
        self
    }

    pub fn project_n(mut self, n: Vector2) -> Matrix2Build {
        self.result *= Matrix2::project_n(n);
        self
    }

    pub fn reflect_n(mut self, n: Vector2) -> Matrix2Build {
        self.result *= Matrix2::reflect_n(n);
        self
    }

    pub fn shear_x(mut self, s: f64) -> Matrix2Build {
        self.result *= Matrix2::shear_x(s);
        self
    }

    pub fn shear_y(mut self, s: f64) -> Matrix2Build {
        self.result *= Matrix2::shear_y(s);
        self
    }

    pub fn build(self) -> Matrix2 {
        self.result
    }
}


#[cfg(test)]
mod test {
    use crate::matrix2::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn identity() {
        let m = Matrix2::unit();
        assert_eq!(m.inverse(), m);
    }

    #[test]
    fn inverse() {
        let m = Matrix2::new(4.0, 3.0, -1.0, 4.0);
        m.inverse().inverse().assert_approx_eq(m);
        assert_approx_eq!(m.inverse().determinant(), 1.0/m.determinant());
    }
}