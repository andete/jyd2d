// (c) 2019 Joost Yervante Damad <joost@damad.be>

use std::fmt::Write as FmtWrite;

use simple_xml_serialize::XMLElement;

use crate::color::Color;
use crate::Coordinate;
use crate::coordinate::Coordinates;

#[derive(Debug, Clone, PartialEq)]
pub struct Area {
    pub corners: Coordinates,
    pub color: Color,
    pub fill: Color,
    pub world: Option<World>,
}

impl Area {
    pub fn new<T: Into<Coordinates>>(corners: T) -> Area {
        Area { corners: corners.into(), color: Color::Black, fill: Color::None, world: None }
    }

    pub fn color(self, color: Color) -> Self {
        Area { color, ..self }
    }

    pub fn fill(self, fill: Color) -> Self {
        Area { fill, ..self }
    }
    pub fn world(self, origin: Coordinate) -> Self {
        let scale = self.corners.axis_scale();
        Area { world: Some(World::new(origin).axis_scale(scale)), ..self }
    }

    pub fn add<X: Into<XMLElement>>(&mut self, x: X) {
        self.world.as_mut().map(|w| w.add(x));
    }
}

impl Into<XMLElement> for Area {
    fn into(self) -> XMLElement {
        let mut data = String::new();
        let corners = self.corners.as_ref();
        write!(&mut data, "M{},{} ", corners[0].x, corners[0].y).unwrap();
        corners.iter().skip(1).for_each(|c| {
            write!(&mut data, "L{},{} ", c.x, c.y).unwrap();
        });
        write!(&mut data, "z").unwrap();
        XMLElement::new("g")
            .element(
                XMLElement::new("path")
                    .attr("d", data)
                    .attr("fill", self.fill)
                    .attr("stroke", self.color)
                    .attr("stroke-width", 0.25)
            )
            .element_opt(self.world)
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

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub p1: Coordinate,
    pub p2: Coordinate,
    pub color: Color,
}

impl Line {
    fn new(p1: Coordinate, p2: Coordinate, color: Color) -> Line {
        Line { p1, p2, color }
    }
}

impl Into<XMLElement> for Line {
    fn into(self) -> XMLElement {
        XMLElement::new("line")
            .attr("x1", self.p1.x)
            .attr("y1", self.p1.y)
            .attr("x2", self.p2.x)
            .attr("y2", self.p2.y)
            .attr("stroke", self.color)
            .attr("stroke-width", 0.25)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Axis {
    location: Coordinate,
    scale: f64,
}

impl Axis {
    pub fn new(scale: f64) -> Axis {
        Axis { location: Coordinate::new(0.0, 0.0), scale }
    }
}

impl Into<XMLElement> for Axis {
    fn into(self) -> XMLElement {
        let arrow_scale = self.scale / 10.0;
        let x_dir = self.location.translate(self.scale, 0.0);
        let y_dir = self.location.translate(0.0, self.scale);
        let x_a1 = x_dir.translate(-arrow_scale, arrow_scale);
        let x_a2 = x_dir.translate(-arrow_scale, -arrow_scale);
        let y_a1 = y_dir.translate(-arrow_scale, -arrow_scale);
        let y_a2 = y_dir.translate(arrow_scale, -arrow_scale);
        let color = Color::Grey;
        XMLElement::new("g")
            .element(Line::new(self.location, x_dir, color))
            .element(Line::new(self.location, y_dir, color))
            .element(Line::new(x_dir, x_a1, color))
            .element(Line::new(x_dir, x_a2, color))
            .element(Line::new(y_dir, y_a1, color))
            .element(Line::new(y_dir, y_a2, color))
            .element(Line::new(x_a1, x_a2, color))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct World {
    pub location: Coordinate,
    pub elements: Vec<XMLElement>,
    pub axis_scale: f64,
}

impl World {
    pub fn new(location: Coordinate) -> World {
        World { location, elements: vec![], axis_scale: 10.0 }
    }
    pub fn add<X: Into<XMLElement>>(&mut self, x: X) {
        self.elements.push(x.into())
    }
    pub fn axis_scale(self, axis_scale: f64) -> Self {
        World { axis_scale, ..self }
    }
}

impl Into<XMLElement> for World {
    fn into(self) -> XMLElement {
        let matrix = self.location.matrix();
        XMLElement::new("g")
            .attr("transform", format!("translate({} {}) matrix({} {} {} {} {} {})",
                                       self.location.x, self.location.y,
                                       matrix.m11, matrix.m12, matrix.m21, matrix.m22, matrix.m31, matrix.m32
            ))
            .element(Axis::new(self.axis_scale))
            .elements(self.elements)
    }
}