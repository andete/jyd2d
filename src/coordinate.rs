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

    pub fn flip_x_if(self, v: bool) -> Coordinate {
        Coordinate { fx: v, ..self }
    }

    pub fn flip_y(self) -> Coordinate {
        Coordinate { fy: !self.fy, ..self }
    }

    pub fn flip_y_if(self, v: bool) -> Coordinate {
        Coordinate { fy: v, ..self }
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

    pub fn matrix(&self) -> Matrix3 {
        Matrix3::builder()
            .scale(self.sx, self.sy)
            .flip_x(self.fx)
            //.flip_x(world.fx)
            .flip_y(self.fy)
            //.flip_y(world.fy)
            .rotate(self.r)
            //.rotate(world.r)
            //.translate(world.into())
            .build()
    }

    pub fn reference_to_world(self, world: &Coordinate) -> Coordinate {
        let matrix = self.matrix();
        let v1 = Vector3::new(self.x, self.y, 1.0);
        let res = v1 * matrix;
        Coordinate::new(res.x, res.y)
            .translate(-world.x, -world.y) // ??
            .rotate(-world.r) // ??
            .flip_x_if(world.fx)
            .flip_y_if(world.fy)
            .scale_x(world.sx)
            .scale_y(world.sy)
    }

    pub fn fix(self) -> Coordinate {
        self.reference_to_world(&Coordinate::new(0.0, 0.0))
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

#[derive(Debug, Clone, PartialEq)]
pub struct Coordinates(pub Vec<Coordinate>);

impl Coordinates {
    pub fn axis_scale(&self) -> f64 {
        let mut sort_x = self.0.clone();
        sort_x.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        let min_x = sort_x.first().unwrap().x;
        let max_x = sort_x.last().unwrap().x;
        let mut sort_y = self.0.clone();
        sort_y.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
        let min_y = sort_y.first().unwrap().y;
        let max_y = sort_y.last().unwrap().y;
        let dx = max_x - min_x;
        let dy = max_y - min_y;
        let d = if dx < dy {
            dx
        } else {
            dy
        };
        d / 10.0
    }
}

impl From<Vec<(f64, f64)>> for Coordinates {
    fn from(v: Vec<(f64, f64)>) -> Coordinates {
        Coordinates(v.into_iter().map(|x| x.into()).collect())
    }
}

impl AsRef<Vec<Coordinate>> for Coordinates {
    fn as_ref(&self) -> &Vec<Coordinate> {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use crate::Coordinate;

    #[test]
    fn flip_x() {
        let c = Coordinate::new(10.0, 6.0);
        let d = Coordinate::new(0.0, 0.0).flip_x();
        let e = c.reference_to_world(&d);
        assert_eq!(Coordinate::new(-10.0, 6.0), e);
    }

    #[test]
    fn flip_y() {
        let c = Coordinate::new(10.0, 6.0);
        let d = Coordinate::new(0.0, 0.0).flip_y();
        let e = c.reference_to_world(&d);
        assert_eq!(Coordinate::new(10.0, -6.0), e);
    }
}