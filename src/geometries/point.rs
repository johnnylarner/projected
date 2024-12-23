use std::marker::PhantomData;

use geo::Point;
use proj::Proj;

use crate::projections::{Epsg3035, Epsg4326, ToEpsg3035, ToEpsg4326, EPSG_3035, EPSG_4326};

#[derive(Clone, Debug)]
pub struct ProjectedPoint<Projection> {
    point: Point,
    _marker: PhantomData<Projection>,
}

impl ProjectedPoint<Epsg4326> {
    fn new(point: Point) -> Self {
        Self {
            point,
            _marker: PhantomData,
        }
    }
}

impl ToEpsg4326 for ProjectedPoint<Epsg3035> {
    type Output = ProjectedPoint<Epsg4326>;
    fn to_epsg_4326(&self) -> Self::Output {
        let crs = Proj::new_known_crs(EPSG_3035, EPSG_4326, None).unwrap();
        let transformed = crs.convert(self.point).unwrap();
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
        let transformed = crs.convert(self.point).unwrap();
        ProjectedPoint {
            point: transformed,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod projections {
    use geo::Point;

    use crate::projections::ToEpsg3035;

    use super::ProjectedPoint;

    #[test]
    fn change_when_converted() {
        let p = Point::new(2.0, 2.1);
        let p = ProjectedPoint::new(p);
        let projected = p.to_epsg_3035();

        assert_ne!(p.point, projected.point);
    }
}
