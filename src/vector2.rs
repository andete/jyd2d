use std::ops;
use crate::coordinate::Coordinate;


#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }


    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(self) -> Vector2 {
        let l = self.length();
        Vector2 { x: self.x / l, y: self.y / l }
    }

    pub fn distance(self, rhs: Vector2) -> f64 {
        (rhs - self).length()
    }

    pub fn dot_product(self, rhs: Vector2) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }


    pub fn perp_dot_product(self, rhs: Vector2) -> f64 {
        -self.y * rhs.x + self.x * rhs.y
    }
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl ops::Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Vector2 {
        Vector2 { x: -self.x, y: -self.y }
    }
}

impl ops::Mul<f64> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f64) -> Vector2 {
        Vector2 { x: self.x * rhs, y: self.y * rhs }
    }
}

impl ops::Mul<Vector2> for f64 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2 { x: rhs.x * self, y: rhs.y * self }
    }
}

impl ops::Div<f64> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f64) -> Vector2 {
        Vector2 { x: self.x / rhs, y: self.y / rhs }
    }
}

impl From<&Coordinate> for Vector2 {
    fn from(c:&Coordinate) -> Vector2 {
        Vector2 { x: c.x, y: c.y }
    }
}

#[cfg(test)]
mod test {
    use crate::vector2::*;

    #[test]
    fn neg() {
        let v = Vector2::new(3.0, 7.0);
        assert_eq!(Vector2::new(-3.0, -7.0), -v);
    }

    #[test]
    fn length() {
        let v = Vector2::new(-12.0, 5.0);
        assert_eq!(13.0, v.length());
    }

    #[test]
    fn mul() {
        let v = Vector2::new(4.0, 0.0);
        assert_eq!(Vector2::new(12.0, 0.0), v * 3.0);
    }

    #[test]
    fn div() {
        let v = Vector2::new(4.0, 5.0);
        assert_eq!(Vector2::new(2.0, 2.5), v / 2.0);
    }

    #[test]
    fn normalize() {
        let v = Vector2::new(12.0, 5.0);
        assert_eq!(Vector2::new(12.0 / 13.0, 5.0 / 13.0), v.normalize());
    }

    #[test]
    fn add() {
        let v1 = Vector2::new(7.0, -2.0);
        let v2 = Vector2::new(6.0, 6.0);
        assert_eq!(Vector2::new(13.0, 4.0), v1 + v2);
    }

    #[test]
    fn sub() {
        let v1 = Vector2::new(3.0, 10.0);
        let v2 = Vector2::new(8.0, -7.0);
        assert_eq!(Vector2::new(-5.0, 17.0), v1 - v2);
    }

    #[test]
    fn sub2() {
        let v1 = Vector2::new(3.0, 10.0);
        let v2 = Vector2::new(8.0, -7.0);
        assert_eq!(Vector2::new(-29.0, 38.0), v1 - 4.0 * v2);
    }

    #[test]
    fn distance() {
        let v1 = Vector2::new(10.0, -14.0);
        let v2 = Vector2::new(6.0, 30.0);
        assert_eq!(((10.0f64 - 6.0) * (10.0 - 6.0) + (-14.0 - 30.0) * (-14.0 - 30.0)).sqrt(), v1.distance(v2));
    }

    #[test]
    fn dot_product() {
        let v1 = Vector2::new(2.0, 6.0);
        let v2 = Vector2::new(-3.0, 8.0);
        assert_eq!(-6.0 + 48.0, v1.dot_product(v2));
    }
}

