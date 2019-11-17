// (c) 2019 Joost Yervante Damad <joost@damad.be>

use svg::node::element::Text as TextElement;
use svg::node::Text;

use crate::coordinate::Coordinate;

pub struct Label {
    pub location: Coordinate,
    pub text: String,
}

impl Label {
    pub fn new(location: Coordinate, text: &str) -> Label {
        Label {
            location,
            text: text.to_string(),
        }
    }
}

impl Into<TextElement> for Label {
    fn into(self) -> TextElement {
        TextElement::new()
            .set("text-anchor", "middle")
            .set("x", self.location.x)
            .set("y", self.location.y)
            .set("font-size", 1.0)
            .add(Text::new(self.text.clone()))
    }
}

