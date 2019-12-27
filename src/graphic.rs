// (c) 2019 Joost Yervante Damad <joost@damad.be>

use std::fmt::Write as FmtWrite;
use std::vec::IntoIter;

use simple_xml_serialize::XMLElement;

use crate::color::Color;
use crate::Coordinate;
use crate::coordinate::Coordinates;
use crate::text::Title;

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub points: Vec<Coordinate>,
    pub color: Color,
    pub stroke_width: Option<f64>,
    pub stroke_dash: Option<String>,
    pub name: String,
}

impl Path {
    pub fn new<T: Into<Coordinate>, U: ToString>(name: U, points: Vec<T>) -> Self {
        Self {
            points: points.into_iter().map(|p| p.into()).collect(),
            color: Color::Black,
            stroke_width: None,
            stroke_dash: None,
            name: name.to_string(),
        }
    }

    pub fn color(self, color: Color) -> Self {
        Self { color, ..self }
    }

    pub fn stroke_width(self, stroke_width: f64) -> Self {
        Self { stroke_width: Some(stroke_width), ..self }
    }

    pub fn stroke_dash<T:ToString>(self, stroke_dash: T) -> Self {
        Self { stroke_dash: Some(stroke_dash.to_string()), ..self }
    }
}

impl Into<XMLElement> for Path {
    fn into(self) -> XMLElement {
        let mut data = String::new();
        write!(&mut data, "M{},{} ", self.points[0].x, self.points[0].y).unwrap();
        self.points.iter().skip(1).for_each(|c| {
            write!(&mut data, "L{},{} ", c.x, c.y).unwrap();
        });

        XMLElement::new("path")
            .attr("id", format!("area-{}", self.name))
            .attr("d", data)
            .attr("stroke", self.color)
            .attr("fill", "none")
            .attr_opt("stroke-width", self.stroke_width)
            .attr_opt("stroke-dasharray", self.stroke_dash)
            .element(Title(self.name))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Area {
    pub corners: Vec<Coordinate>,
    pub color: Color,
    pub fill: Color,
    pub world: Option<World>,
    pub stroke_width: Option<f64>,
    pub name: String,
}

impl Area {
    pub fn new<T: Into<Coordinate>, U: ToString>(name: U, corners: Vec<T>) -> Area {
        Area {
            corners: corners.into_iter().map(|c| c.into()).collect(),
            color: Color::Black,
            fill: Color::None,
            world: None,
            stroke_width: None,
            name: name.to_string(),
        }
    }

    pub fn color(self, color: Color) -> Self {
        Area { color, ..self }
    }

    pub fn fill(self, fill: Color) -> Self {
        Area { fill, ..self }
    }

    pub fn world(self, origin: Coordinate, stroke_width: Option<f64>) -> Self {
        let scale = Coordinates::axis_scale(&self.corners);
        let world = World::new(format!("world-{}", self.name), origin).axis_scale(scale).stroke_width_opt(stroke_width);
        Area { world: Some(world), ..self }
    }

    pub fn stroke_width(self, stroke_width: f64) -> Self {
        Area { stroke_width: Some(stroke_width), ..self }
    }

    pub fn add<X: Into<XMLElement>>(&mut self, x: X) {
        self.world.as_mut().map(|w| w.add(x));
    }

    pub fn add_all<X: Into<XMLElement>>(&mut self, xn: IntoIter<X>) {
        self.world.as_mut().map(|w| w.add_all(xn));
    }
}

impl Into<XMLElement> for Area {
    fn into(self) -> XMLElement {
        let mut data = String::new();
        let corners = self.corners;
        write!(&mut data, "M{},{} ", corners[0].x, corners[0].y).unwrap();
        corners.iter().skip(1).for_each(|c| {
            write!(&mut data, "L{},{} ", c.x, c.y).unwrap();
        });
        write!(&mut data, "z").unwrap();
        XMLElement::new("g")
            .element(
                XMLElement::new("path")
                    .attr("id", format!("area-{}", self.name))
                    //.attr("opacity", 0.5)
                    .attr("d", data)
                    .attr("fill", self.fill)
                    .attr("stroke", self.color)
                    .attr_opt("stroke-width", self.stroke_width)
                    .element(Title(self.name))
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
    pub stroke_width: Option<f64>,
}

impl Line {
    pub fn new<T: Into<Coordinate>>(p1: T, p2: T, color: Color) -> Line {
        Line { p1: p1.into(), p2: p2.into(), color, stroke_width: None }
    }

    pub fn stroke_width(self, stroke_width: f64) -> Line {
        Line { stroke_width: Some(stroke_width), ..self }
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
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Axis {
    location: Coordinate,
    scale: f64,
}

impl Axis {
    pub fn new(scale: f64) -> Axis {
        Axis { location: (0.0, 0.0).into(), scale }
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
    pub name: String,
    pub stroke_width: Option<f64>,
}

impl World {
    pub fn new<T: ToString>(name: T, location: Coordinate) -> World {
        World { location, elements: vec![], axis_scale: 10.0, name: name.to_string(), stroke_width: None }
    }
    pub fn add<X: Into<XMLElement>>(&mut self, x: X) {
        self.elements.push(x.into())
    }
    pub fn add_all<X: Into<XMLElement>>(&mut self, xn: IntoIter<X>) {
        xn.into_iter().for_each(|x| self.elements.push(x.into()))
    }
    pub fn axis_scale(self, axis_scale: f64) -> Self {
        World { axis_scale, ..self }
    }
    pub fn stroke_width_opt(self, stroke_width: Option<f64>) -> Self {
        World { stroke_width, ..self }
    }
}

impl Into<XMLElement> for World {
    fn into(self) -> XMLElement {
        let matrix = self.location.matrix();
        XMLElement::new("g")
            .attr("id", format!("world-{}", self.name))
            .attr("transform", format!("translate({} {}) matrix({} {} {} {} {} {})",
                                       self.location.x, self.location.y,
                                       matrix.m11, matrix.m12, matrix.m21, matrix.m22, matrix.m31, matrix.m32
            ))
            .attr_opt("stroke-width", self.stroke_width)
            .element(Axis::new(self.axis_scale))
            .element(Title(self.name))
            .elements(self.elements)
    }
}