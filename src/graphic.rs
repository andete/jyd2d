// (c) 2019 Joost Yervante Damad <joost@damad.be>

use std::fmt::Write as FmtWrite;

use simple_xml_serialize::XMLElement;

use crate::color::Color;
use crate::Coordinate;
use crate::coordinate::Coordinates;

#[derive(Debug, Clone, PartialEq)]
pub struct Area {
    pub origin: Coordinate,
    pub corners: Coordinates,
    pub color: Color,
    pub fill: Color,
}

impl Area {
    pub fn new(origin: Coordinate, corners: Coordinates) -> Area {
        Area { origin, corners, color: Color::Black, fill: Color::None }
    }

    pub fn color(self, color: Color) -> Self {
        Area { color, ..self }
    }

    pub fn fill(self, fill: Color) -> Self {
        Area { fill, ..self }
    }
}

impl Into<XMLElement> for Area {
    fn into(self) -> XMLElement {
        let mut data = String::new();
        let corners = self.corners.as_ref();
        println!("corners: {:?}", corners);
        let corners: Vec<Coordinate> = corners.iter().map(|c| c.reference_to_world(&self.origin)).collect();
        println!("corners2: {:?}", corners);
        write!(&mut data, "M{},{} ", corners[0].x, corners[0].y).unwrap();
        corners.iter().skip(1).for_each(|c| {
            write!(&mut data, "L{},{} ", c.x, c.y).unwrap();
        });
        write!(&mut data, "z").unwrap();
        XMLElement::new("path")
            .attr("d", data)
            .attr("fill", self.fill)
            .attr("stroke", self.color)
            .attr("stroke-width", 0.25)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
    pub color: Color,
    pub fill: Color,
}

impl Circle {
    pub fn new(cx: f64, cy: f64, r: f64, color: Color, fill: Color) -> Circle {
        Circle { cx, cy, r, color, fill }
    }
}

impl Into<XMLElement> for Circle {
    fn into(self) -> XMLElement {
        XMLElement::new("circle")
            .attr("r", self.r)
            .attr("cx", self.cx)
            .attr("cy", self.cy)
            .attr("fill", self.fill)
            .attr("stroke", self.color)
            .attr("stroke-width", 0.25)
    }
}