use std::fmt::Write as FmtWrite;
use std::io::{Error, Write};

use crate::color::Color;
use crate::coordinate::{Coordinate, Coordinates};
use crate::svg_document::WriteToSvg;

#[derive(Debug, Clone, PartialEq)]
pub struct Area {
    pub corners: Coordinates,
    pub color: Color,
    pub fill: Color,
}

impl Area {
    pub fn new(corners: Coordinates) -> Area {
        Area { corners, color: Color::Black, fill: Color::None }
    }

    pub fn new_with_color(corners: Coordinates, color: Color) -> Area {
        Area {
            corners,
            color,
            fill: Color::None,
        }
    }

    pub fn new_filled(corners: Coordinates, color: Color, fill: Color) -> Area {
        Area { corners, color, fill }
    }
}

impl WriteToSvg for Area {
    fn write<T: Write>(&self, indent: i16, mut out: &mut T) -> Result<(), Error> {
        let mut data = String::new();
        let corners = self.corners.as_ref();
        write!(&mut data, "M{},{} ", corners[0].x, corners[0].y).unwrap();
        corners.iter().skip(1).for_each(|c| {
            write!(&mut data, "L{},{} ", c.x, c.y).unwrap();
        });
        write!(&mut data, "z").unwrap();
        self.indent(out, indent)?;
        write!(&mut out, "<path d=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"0.25\" />\n",
               data, self.fill.to_string(), self.color.to_string())
    }
}