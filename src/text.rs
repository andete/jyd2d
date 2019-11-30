// (c) 2019 Joost Yervante Damad <joost@damad.be>



use simple_xml_serialize::XMLElement;

use crate::coordinate::Coordinate;

#[derive(Debug)]
pub struct Label {
    pub location: Coordinate,
    pub text: String,
    pub size: f64,
}

impl Label {
    pub fn new(location: Coordinate, text: &str, size: f64) -> Label {
        Label {
            location,
            text: text.to_string(),
            size,
        }
    }
}

impl Into<XMLElement> for Label {
    fn into(self) -> XMLElement {
        XMLElement::new("text")
            .attr("text-anchor", "middle")
            .attr("x", self.location.x)
            .attr("y", self.location.y)
            .attr("font-size", self.size)
            .text(self.text)
    }
}

#[derive(Debug)]
pub struct Title(pub String);

impl Into<XMLElement> for Title {
    fn into(self) -> XMLElement {
        XMLElement::new("title")
            .text(self.0)
    }
}

#[derive(Debug)]
pub struct Description(pub String);

impl Into<XMLElement> for Description {
    fn into(self) -> XMLElement {
        XMLElement::new("desc")
            .text(self.0)
    }
}