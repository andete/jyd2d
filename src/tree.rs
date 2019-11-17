// (c) 2019 Joost Yervante Damad <joost@damad.be>

use svg::node::element::{Circle, Description, Group, Title};
use svg::node::element::Text as TextElement;
use svg::node::Text;

use crate::color::Color;
use crate::coordinate::Coordinate;
use crate::label::Label;

#[derive(Debug)]
pub struct Tree {
    pub name: String,
    pub species: String,
    pub trunk_diameter: f64,
    pub crown_diameter: f64,
    pub location: Coordinate,
    pub label_location: Coordinate,
}

impl Into<Group> for Tree {
    fn into(self) -> Group {
        let label: TextElement = Label::new(self.label_location, &self.name).into();
        let desc = Description::new().add(Text::new(format!("{}", self.species)));
        let title = Title::new().add(Text::new(format!("Boom {}", self.name)));
        let crown = Circle::new()
            .set("fill", Color::Green.to_string())
            .set("stroke", Color::DarkGreen.to_string())
            .set("r", self.crown_diameter)
            .set("id", format!("tree-crown-{}", self.name));
        let trunk = Circle::new()
            .set("fill", Color::Brown.to_string())
            .set("stroke", Color::Maroon.to_string())
            .set("r", self.trunk_diameter)
            .set("id", format!("tree-trunk-{}", self.name));
        Group::new().add(label).add(crown).add(trunk)
            .set("stroke-width", 0.2)
            .set("cx", self.location.x)
            .set("cy", -self.location.y).add(desc).add(title)
    }
}