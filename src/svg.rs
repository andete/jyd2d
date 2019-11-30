// (c) 2019 Joost Yervante Damad <joost@damad.be>

use std::fs::File;
use std::io::Write;

use simple_xml_serialize::XMLElement;

pub struct Document {
    pub min_x: f64,
    pub min_y: f64,
    pub width: f64,
    pub height: f64,
    pub pixel_width: i64,
    pub pixel_height: i64,
    children: Vec<XMLElement>,
}

impl Document {
    pub fn new(min_x: f64, min_y: f64, width: f64, height: f64, pixel_width: i64, pixel_height: i64) -> Document {
        Document { min_x, min_y, width, height, pixel_width, pixel_height, children: vec![] }
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
        let children: Vec<XMLElement> = self.children.iter().map(|x| x.into()).collect();
        XMLElement::new("svg")
            .attr("width", self.width)
            .attr("height", self.height)
            .attr("viewBox", view_box)
            .attr("xmlns", "http://www.w3.org/2000/svg")
            .elements(children)
    }
}