use svg::Node;
use svg::node::element::Path;
use svg::node::element::path::Data;

use crate::color::Color;
use crate::coordinate::Coordinate;

#[derive(Debug, Clone, PartialEq)]
pub struct Area {
    pub corners: Vec<Coordinate>,
    pub color: Color,
    pub fill: Color,
}

impl Area {
    fn new(corners: Vec<Coordinate>) -> Area {
        Area { corners, color: Color::Black, fill: Color::None }
    }

    fn new_with_color(corners: Vec<Coordinate>, color: Color) -> Area {
        Area {
            corners,
            color,
            fill: Color::None,
        }
    }

    fn new_filled(corners: Vec<Coordinate>, color: Color, fill: Color) -> Area {
        Area { corners, color, fill }
    }
}

impl Into<Box<dyn Node>> for Area {
    fn into(self) -> Box<dyn Node> {
        let mut data = Data::new();
        assert!(self.corners.len() > 2);
        data = data.move_to((self.corners[0].x, self.corners[0].y));
        for corner in self.corners.iter().skip(1) {
            data = data.line_to((corner.x, corner.y));
        }

        Box::new(Path::new()
            .set("fill", self.fill.to_string())
            .set("stroke", self.color.to_string())
            .set("stroke-width", 0.25)
            .set("d", data))
    }
}