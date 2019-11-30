// (c) 2019 Joost Yervante Damad <joost@damad.be>



use simple_xml_serialize::XMLElement;

use crate::coordinate::Coordinate;

#[derive(Debug)]
pub struct Label {
    pub location: Coordinate,
    pub text: String,
    pub size: Option<f64>,
}

impl Label {
    pub fn new(location: Coordinate, text: &str) -> Label {
        Label {
            location,
            text: text.to_string(),
            size: None,
        }
    }
    pub fn size(mut self, size: f64) -> Self {
        self.size = Some(size);
        self
    }
}

impl Into<XMLElement> for Label {
    fn into(self) -> XMLElement {
        XMLElement::new("text")
            .attr("text-anchor", "middle")
            .attr("x", self.location.x)
            .attr("y", self.location.y)
            .attr_opt("font-size", self.size)
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