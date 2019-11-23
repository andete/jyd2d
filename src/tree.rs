// (c) 2019 Joost Yervante Damad <joost@damad.be>

use crate::color::Color;
use crate::coordinate::Coordinate;
use crate::label::Label;

#[derive(Debug)]
pub struct Tree {
    pub name: String,
    pub species: String,
    pub trunk_diameter: f64,
    pub crown_diameter: Option<f64>,
    pub location: Coordinate,
    pub label_location: Coordinate,
}

//impl Into<Group> for Tree {
//    fn into(self) -> Group {
//        let label: TextElement = Label::new(self.label_location, &self.name).into();
//        let desc = Description::new().add(Text::new(format!("{}", self.species)));
//        let title = Title::new().add(Text::new(format!("Tree {}", self.name)));
//        let crown = self.crown_diameter.map(|d| Circle::new()
//            .set("fill", Color::Green.to_string())
//            .set("stroke", Color::DarkGreen.to_string())
//            .set("r", d/2.0));
//        let trunk = Circle::new()
//            .set("fill", Color::Brown.to_string())
//            .set("stroke", Color::Maroon.to_string())
//            .set("r", self.trunk_diameter/2.0);
//        let group = Group::new().add(label);
//        let group = match crown {
//            Some(c) => group.add(c),
//            None => group,
//        };
//        group.add(trunk)
//            .set("stroke-width", 0.2)
//            .set("cx", self.location.x)
//            .set("cy", -self.location.y)
//            .set("id", format!("tree-{}", self.name))
//            .add(desc)
//            .add(title)
//    }
//}