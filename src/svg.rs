// (c) 2019 Joost Yervante Damad <joost@damad.be>

use std::fs::File;
use std::io;

use crate::element::Element;

pub trait WriteToSvg {
    fn write<T: io::Write>(&self, indent: i16, out: &mut T) -> io::Result<()>;

    fn indent<T: io::Write>(&self, mut out: &mut T, indent: i16) -> io::Result<()> {
        for _ in 0..indent {
            write!(&mut out, "  ")?;
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Children(Vec<Element>);

impl Children {
    pub fn add<T: Into<Element>>(&mut self, element: T) {
        self.0.push(element.into())
    }
}

pub struct Document {
    pub min_x: f64,
    pub min_y: f64,
    pub width: f64,
    pub height: f64,
    pub pixel_width: i64,
    pub pixel_height: i64,
    pub children: Children,
}

impl Document {
    pub fn new(min_x: f64, min_y: f64, width: f64, height: f64, pixel_width: i64, pixel_height: i64) -> Document {
        Document { min_x, min_y, width, height, pixel_width, pixel_height, children: Children::default() }
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        let mut out = File::create(filename)?;
        self.write(0, &mut out)
    }
}

impl WriteToSvg for Document {
    fn write<T: io::Write>(&self, indent: i16, mut out: &mut T) -> io::Result<()> {
        let view_box = format!("{} {} {} {}", self.min_x, self.min_y, self.width, self.height);
        self.indent(out, indent)?;
        write!(out, "<svg width=\"{}\" height=\"{}\" viewBox=\"{}\"  xmlns=\"http://www.w3.org/2000/svg\">\n",
               self.width,
               self.height,
               view_box)?;
        self.children.write(indent + 1, &mut out)?;
        self.indent(out, indent)?;
        write!(out, "</svg>\n")
    }
}

impl WriteToSvg for Children {
    fn write<T: io::Write>(&self, indent: i16, mut out: &mut T) -> io::Result<()> {
        for x in &self.0 {
            x.write(indent, &mut out)?;
        }
        Ok(())
    }
}