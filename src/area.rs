use crate::coordinate::Coordinate;
use svg::node::element::Element;
use svg::node::element::path::Data;
use svg::node::element::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct Area {
    pub corners: Vec<Coordinate>,
}

impl Into<Element> for Area {
    fn into(self) -> Element {

        let data = Data::new();
        assert!(self.corners.len() > 2);
        data.move_to((corners[0].x, corners[0].y));
        for corner in self.corners[1..] {
            data.line_to((corner.x, corner.y));
        }
        data.close();

        Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 0.25)
            .set("d", data)
    }
}