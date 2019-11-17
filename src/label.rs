// (c) 2019 Joost Yervante Damad <joost@damad.be>

use svg::node::element::Text as TextElement;
use svg::node::Text;

use crate::coordinate::Coordinate;
use crate::svg_element::SVGElement;

struct Label {
    location: Coordinate,
    text: String,
}

impl Into<SVGElement> for Label {
    fn into(self) -> SVGElement {
        SVGElement::Text(
            TextElement::new()
                .set("text-anchor", "middle")
                .set("x", self.location.x)
                .set("y", self.location.y)
                .set("font-size", 1.0)
                .add(Text::new(self.text.clone())))
    }
}

