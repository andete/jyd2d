// (c) 2019 Joost Yervante Damad <joost@damad.be>

use std::fs::File;
use std::io;
use std::io::Write;

use crate::{Area, WriteToSvg};

#[derive(Debug)]
pub enum Element {
    Area(Area)
}

impl Into<Element> for Area {
    fn into(self: Area) -> Element {
        Element::Area(self)
    }
}


impl WriteToSvg for Element {
    fn write<T: Write>(&self, indent: i16, mut out: &mut T) -> std::io::Result<()> {
        match self {
            Element::Area(area) => area.write(indent, &mut out)
        }
    }
}


