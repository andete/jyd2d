// (c) 2019 Joost Yervante Damad <joost@damad.be>

use std::io::{Error, Write};
use std::io;

use crate::coordinate::Coordinate;
use crate::WriteToSvg;

#[derive(Debug)]
pub struct Label {
    pub location: Coordinate,
    pub text: String,
    pub size: f64,
}

impl Label {
    pub fn new(location: Coordinate, text: &str, size: f64) -> Label {
        Label {
            location,
            text: text.to_string(),
            size,
        }
    }
}

impl WriteToSvg for Label {
    fn write<T: io::Write>(&self, indent: i16, mut out: &mut T) -> Result<(), io::Error> {
        self.indent(&mut out, indent)?;
        write!(&mut out, "<text text-anchor=\"middle\" x=\"{}\" y=\"{}\" font-size=\"{}\">{}</text>\n",
               self.text, self.location.x, self.location.y, self.size)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Title(pub String);

impl WriteToSvg for Title {
    fn write<T: io::Write>(&self, indent: i16, mut out: &mut T) -> Result<(), io::Error> {
        self.indent(&mut out, indent)?;
        write!(&mut out, "<title>{}</title>", self.0)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Description(pub String);

impl WriteToSvg for Description {
    fn write<T: io::Write>(&self, indent: i16, mut out: &mut T) -> Result<(), io::Error> {
        self.indent(&mut out, indent)?;
        write!(&mut out, "<desc>{}</desc>", self.0)?;
        Ok(())
    }
}