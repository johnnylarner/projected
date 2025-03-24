mod epsg_3035;
mod epsg_4326;
mod laea;

pub use epsg_3035::{EPSG_3035, Epsg3035, ToEpsg3035};
pub use epsg_4326::{EPSG_4326, Epsg4326, ToEpsg4326};
use geo::{Centroid, Geometry, MultiPolygon, Point, Polygon};
pub use laea::{Laea, ToLaea, make_laea_str};

pub trait Projectable {}

impl Projectable for Epsg4326 {}

impl Projectable for Epsg3035 {}

pub trait HasCentroid {
    fn centriod(&self) -> Point;
}

impl HasCentroid for Point {
    fn centriod(&self) -> Point {
        self.centroid()
    }
}

impl HasCentroid for Polygon {
    fn centriod(&self) -> Point {
        self.centroid().unwrap()
    }
}

impl HasCentroid for MultiPolygon {
    fn centriod(&self) -> Point {
        self.centroid().unwrap()
    }
}

impl HasCentroid for Geometry {
    fn centriod(&self) -> Point {
        match self {
            Geometry::Point(p) => p.centriod(),
            Geometry::Polygon(p) => p.centriod(),
            Geometry::MultiPolygon(mp) => mp.centriod(),
            _ => unreachable!("unsupported geometry provided"),
        }
    }
}
