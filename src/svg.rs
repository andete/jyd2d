// (c) 2019 Joost Yervante Damad <joost@damad.be>

use std::fs::File;
use std::io::Write;

use simple_xml_serialize::XMLElement;

use crate::Coordinate;
use crate::graphic::World;

pub struct Document {
    pub min_x: f64,
    pub min_y: f64,
    pub width: f64,
    pub height: f64,
    pub pixels_per_unit: i64,
    pub origin: Coordinate,
    children: Vec<XMLElement>,
}

impl Document {
    pub fn new(min_x: f64, min_y: f64, width: f64, height: f64, pixels_per_unit: i64) -> Document {
        let origin = Coordinate::new(min_x, min_y + height).flip_y();
        Document { min_x, min_y, width, height, pixels_per_unit, origin, children: vec![] }
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        let mut out = File::create(filename)?;
        let xml: XMLElement = self.into();
        out.write(xml.to_string_pretty("\n", "  ").as_bytes())?;
        Ok(())
    }

    pub fn add<X: Into<XMLElement>>(&mut self, x: X) {
        self.children.push(x.into())
    }
}

impl Into<XMLElement> for &Document {
    fn into(self) -> XMLElement {
        let view_box = format!("{} {} {} {}", self.min_x, self.min_y, self.width, self.height);
        let pixels_per_unit = self.pixels_per_unit as f64;
        let pixel_width = (self.width * pixels_per_unit).ceil() as i64;
        let pixel_height = (self.height * pixels_per_unit).ceil() as i64;
        let mut world = World::new(self.origin);
        for child in &self.children {
            world.add(child)
        }
        XMLElement::new("svg")
            .attr("width", pixel_width)
            .attr("height", pixel_height)
            .attr("viewBox", view_box)
            .attr("xmlns", "http://www.w3.org/2000/svg")
            .attr("stroke-width", 0.25)
            .element(world)
    }
}