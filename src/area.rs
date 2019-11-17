use crate::coordinate::Coordinate;
use svg::node::element::path::Data;
use svg::node::element::Path;
use crate::svg_element::SVGElement;

#[derive(Debug, Clone, PartialEq)]
pub struct Area {
    pub corners: Vec<Coordinate>,
}

impl Into<SVGElement> for Area {
    fn into(self) -> SVGElement {

        let mut data = Data::new();
        assert!(self.corners.len() > 2);
        data = data.move_to((self.corners[0].x, self.corners[0].y));
        for corner in self.corners.iter().skip(1) {
            data = data.line_to((corner.x, corner.y));
        }
        data = data.close();

        SVGElement::Path(Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 0.25)
            .set("d", data))
    }
}