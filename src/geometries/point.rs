use std::marker::PhantomData;

use geo::{Geometry, Point};
use proj::{Proj, Transform};

use crate::projections::{Epsg3035, Epsg4326, ToEpsg3035, ToEpsg4326, EPSG_3035, EPSG_4326};

#[derive(Clone, Debug)]
pub struct ProjectedPoint<Projection> {
    point: Point,
    _marker: PhantomData<Projection>,
}

impl ProjectedPoint<Epsg4326> {
    pub fn new(point: geo::Point) -> ProjectedPoint<Epsg4326> {
        Self {
            point,
            _marker: PhantomData,
        }
    }
}

impl<T> Into<Geometry> for ProjectedPoint<T> {
    fn into(self) -> Geometry {
        Geometry::Point(self.point)
    }
}

macro_rules! point_value {
    ( for $( $t:ty ),* ) => {
        $(
        impl ProjectedPoint<$t> {
            pub fn point(&self) -> &Point {
                &self.point
            }
        }
        )*
    };
}

point_value!(for Epsg4326, Epsg3035);

impl ToEpsg4326 for ProjectedPoint<Epsg3035> {
    type Output = ProjectedPoint<Epsg4326>;
    fn to_epsg_4326(&self) -> Self::Output {
        let crs = Proj::new_known_crs(EPSG_3035, EPSG_4326, None).unwrap();
        let transformed = self.point.transformed(&crs).unwrap();
        ProjectedPoint {
            point: transformed,
            _marker: PhantomData,
        }
    }
}

impl ToEpsg3035 for ProjectedPoint<Epsg4326> {
    type Output = ProjectedPoint<Epsg3035>;
    fn to_epsg_3035(&self) -> Self::Output {
        let crs = Proj::new_known_crs(EPSG_4326, EPSG_3035, None).unwrap();
        let transformed = self.point.transformed(&crs).unwrap();
        ProjectedPoint {
            point: transformed,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod macro_methods {
    use super::*;
    #[test]
    fn assign_correctly() {
        let p = Point::new(2.0, 2.1);
        let marco_method_point = ProjectedPoint::<Epsg4326>::new(p);
        assert_eq!(marco_method_point.point(), &p);
    }
}

#[cfg(test)]
mod projections {
    use super::*;

    #[test]
    fn change_when_converted() {
        let p = Point::new(2.0, 2.1);
        let p = ProjectedPoint::new(p);
        let projected = p.to_epsg_3035();
        assert_ne!(p.point(), projected.point());
    }
}
