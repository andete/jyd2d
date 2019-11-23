// (c) 2019 Joost Yervante Damad <joost@damad.be>

use crate::matrix3::Matrix3;
use crate::vector3::Vector3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    // Euler angle
    pub r: f64,
    // flipped x axis
    pub fx: bool,
    // flipped y axis
    pub fy: bool,
    // scale x
    pub sx: f64,
    // scale y
    pub sy: f64,
}

impl Coordinate {
    pub fn new(x: f64, y: f64) -> Coordinate {
        Coordinate { x, y, r: 0.0, fx: false, fy: false, sx: 1.0, sy: 1.0 }
    }

    pub fn flip_x(self) -> Coordinate {
        Coordinate { fx: !self.fx, ..self }
    }

    pub fn flip_y(self) -> Coordinate {
        Coordinate { fy: !self.fy, ..self }
    }

    pub fn rotate(self, r: f64) -> Coordinate {
        Coordinate { r: self.r + r, ..self }
    }

    pub fn scale_x(self, sx: f64) -> Coordinate {
        Coordinate { sx, ..self }
    }

    pub fn scale_y(self, sy: f64) -> Coordinate {
        Coordinate { sy, ..self }
    }

    pub fn translate(self, x: f64, y: f64) -> Coordinate {
        Coordinate {
            x: self.x + x,
            y: self.y + y,
            ..self
        }
    }

    pub fn reference_to_world(self, reference: &Coordinate) -> Coordinate {
        let matrix = Matrix3::builder()
            .scale(reference.sx, reference.sy)
            .flip_x(self.fx)
            .flip_x(reference.fx)
            .flip_y(self.fy)
            .flip_y(reference.fy)
            .rotate(self.r)
            .rotate(reference.r)
            .translate(reference.into())
            .build();
        let v1 = Vector3::new(self.x, self.y, 1.0);
        let res = v1 * matrix;
        Coordinate::new(res.x, res.y)
    }

    pub fn distance(self, other: Coordinate) -> f64 {
        (other.x - self.x) * (other.x - self.x) + (other.y - self.y) * (other.y - self.y).sqrt()
    }
}

impl From<(f64, f64)> for Coordinate {
    fn from(t: (f64, f64)) -> Coordinate {
        Coordinate::new(t.0, t.1)
    }
}