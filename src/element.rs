// (c) 2019 Joost Yervante Damad <joost@damad.be>

use std::io;

use crate::{Area, WriteToSvg};
use crate::graphic::Circle;

#[derive(Debug)]
pub enum Element {
    Area(Area),
    Circle(Circle),
}

impl Into<Element> for Area {
    fn into(self) -> Element {
        Element::Area(self)
    }
}

impl Into<Element> for Circle {
    fn into(self) -> Element {
        Element::Circle(self)
    }
}


impl WriteToSvg for Element {
    fn write<T: io::Write>(&self, indent: i16, mut out: &mut T) -> io::Result<()> {
        match self {
            Element::Area(area) => area.write(indent, &mut out),
            Element::Circle(circle) => circle.write(indent, &mut out),
        }
    }
}


