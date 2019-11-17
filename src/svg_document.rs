// (c) 2019 Joost Yervante Damad <joost@damad.be>

use svg::{Node, Document};

#[derive(Debug)]
pub struct SVGDocument {
    pub min_x: f64,
    pub min_y: f64,
    pub width: f64,
    pub height: f64,
    pub pixel_width: f64,
    pub pixel_height: f64,
    document: Document,
}

impl SVGDocument {
    pub fn new(min_x: f64, min_y: f64, width: f64, height: f64, pixel_width: f64, pixel_height: f64) -> SVGDocument {
        let document = Document::new()
            .set("viewBox", (min_x, min_y, width, height))
            .set("width", pixel_width)
            .set("height", pixel_height)
            .set("preserveAspectRatio", "xMinYMax");
        SVGDocument { min_x, min_y, width, height, pixel_width, pixel_height, document }
    }

    pub fn add<T:Node>(mut self, element: T) {
        self.document = self.document.add(element);
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        svg::save(filename, &self.document)
    }
}