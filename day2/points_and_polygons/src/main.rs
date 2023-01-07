// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

use std::ops::Add;
use std::f64::consts::TAU; // 2 PIs for the price of one

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn magnitude(&self) -> f64 {
        f64::from(self.x.pow(2) + self.y.pow(2)).sqrt()
    }

    fn dist(&self, p2: Point) -> f64 {
        f64::from((self.x - p2.x).pow(2) + (self.y - p2.y).pow(2)).sqrt()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x : self.x + other.x,
            y : self.y + other.y
        }
    }
}

pub struct Polygon {
    points: Vec<Point>
}

impl Polygon {
    fn new() -> Polygon {
        Polygon { points : Vec::new() }
    }

    fn add_point(&mut self, next: Point) {
        self.points.push(next)
    }

    fn left_most_point(&self) -> Option<Point> {
        self.points.iter().min_by_key(|point| point.x).copied()
    }

    fn iter(&self) -> impl Iterator<Item = &Point> {
        self.points.iter()
    }

    fn length(&self) -> f64 {
        // dirty javascript style hax
        let mut i = 0;
        self.iter().map(|e| {
            if i<self.points.len()-1 {
                i += 1;
                e.dist(self.points[i])
            } else {
                e.dist(self.points[0])
            }
        }).sum()
    }
}

pub struct Circle {
    centre: Point,
    radius: u32
}

impl Circle {
    fn new(centre: Point, radius: u32) -> Circle {
        Circle { centre, radius }
    }

    fn circumference(&self) -> f64 {
        f64::from(self.radius) * TAU
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Polygon> for Shape {
    fn from(poly: Polygon) -> Self {
        Shape::Polygon(poly)
    }
}

impl From<Circle> for Shape {
    fn from(circ: Circle) -> Self {
        Shape::Circle(circ)
    }
}

impl Shape {
    fn circumference(&self) -> f64 {
        match self {
            Shape::Polygon(poly) => poly.length(),
            Shape::Circle(circ) => circ.circumference()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_circumferences() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let circumferences = shapes
            .iter()
            .map(Shape::circumference)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(circumferences, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}
