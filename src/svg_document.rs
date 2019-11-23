// (c) 2019 Joost Yervante Damad <joost@damad.be>

use std::fs::File;
use std::io::Write;

use crate::Area;

pub trait WriteToSvg {
    fn write<T: Write>(&self, indent: i16, out: &mut T) -> std::io::Result<()>;

    fn indent<T: Write>(&self, mut out: &mut T, indent: i16) -> std::io::Result<()> {
        for _ in 0..indent {
            write!(&mut out, " ")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Element {
    Area(Area)
}

impl Into<Element> for Area {
    fn into(self: Area) -> Element {
        Element::Area(self)
    }
}

pub struct Document {
    pub min_x: f64,
    pub min_y: f64,
    pub width: f64,
    pub height: f64,
    pub pixel_width: i64,
    pub pixel_height: i64,
    elements: Vec<Element>,
}

impl WriteToSvg for Element {
    fn write<T: Write>(&self, indent: i16, mut out: &mut T) -> std::io::Result<()> {
        match self {
            Element::Area(area) => area.write(indent, &mut out)
        }
    }
}


impl Document {
    pub fn new(min_x: f64, min_y: f64, width: f64, height: f64, pixel_width: i64, pixel_height: i64) -> Document {
        Document { min_x, min_y, width, height, pixel_width, pixel_height, elements: vec![] }
    }

    pub fn add<T: Into<Element>>(&mut self, element: T) {
        self.elements.push(element.into())
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        let mut out = File::create(filename)?;
        self.write(0, &mut out)
    }
}

impl WriteToSvg for Document {
    fn write<T: Write>(&self, indent: i16, mut out: &mut T) -> std::io::Result<()> {
        let view_box = format!("{} {} {} {}", self.min_x, self.min_y, self.width, self.height);
        self.indent(out, indent)?;
        write!(out, "<svg width=\"{}\" height=\"{}\" viewBox=\"{}\"  xmlns=\"http://www.w3.org/2000/svg\">\n",
               self.width,
               self.height,
               view_box)?;
        for x in &self.elements {
            x.write(indent + 1, &mut out)?;
        }
        self.indent(out, indent)?;
        write!(out, "</svg>\n")
    }
}