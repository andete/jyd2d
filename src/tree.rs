// (c) 2019 Joost Yervante Damad <joost@damad.be>

use simple_xml_serialize::XMLElement;

use crate::Circle;
use crate::color::Color;
use crate::coordinate::Coordinate;
use crate::text::{Description, Label, Title};

#[derive(Debug)]
pub struct Tree {
    pub name: String,
    pub species: String,
    pub trunk_diameter: f64,
    pub crown_diameter: Option<f64>,
    pub location: Coordinate,
    pub label_location: Coordinate,
}

impl Into<XMLElement> for Tree {
    fn into(self) -> XMLElement {
        XMLElement::new("g")
            .attr("stroke-width", 0.2)
            .attr("id", format!("tree-{}", self.name))
            .element(Label::new(self.label_location, &self.name))
            .element(Title(format!("Tree {}", self.name)))
            .element(Description(format!("Tree {}", self.name)))
            .element_opt(self.crown_diameter.map(|d| {
                Circle::new(self.location.x, self.location.y, d / 2.0,
                            Color::Green, Color::DarkGreen)
            }))
            .element(Circle::new(self.location.x, self.location.y, self.trunk_diameter / 2.0,
                                 Color::Brown, Color::Maroon))
    }
}