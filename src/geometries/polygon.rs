use std::marker::PhantomData;

use geo::Polygon;
use proj::{Proj, Transform};

use crate::projections::{Epsg3035, Epsg4326, ToEpsg3035, ToEpsg4326, EPSG_3035, EPSG_4326};

#[derive(Clone, Debug)]
pub struct ProjectedPolygon<Projection> {
    polygon: Polygon,
    _marker: PhantomData<Projection>,
}

impl ProjectedPolygon<Epsg4326> {
    pub fn new(polygon: Polygon) -> ProjectedPolygon<Epsg4326> {
        Self {
            polygon,
            _marker: PhantomData,
        }
    }
}

macro_rules! polygon_value {
    ( for $( $t:ty ),* ) => {
        $(
        impl ProjectedPolygon<$t> {
            pub fn polygon(&self) -> &Polygon {
                &self.polygon
            }
        }
        )*
    };
}

polygon_value!(for Epsg4326, Epsg3035);

impl ToEpsg4326 for ProjectedPolygon<Epsg3035> {
    type Output = ProjectedPolygon<Epsg4326>;
    fn to_epsg_4326(&self) -> Self::Output {
        let crs = Proj::new_known_crs(EPSG_3035, EPSG_4326, None).unwrap();
        let transformed = self.polygon.transformed(&crs).unwrap();
        ProjectedPolygon {
            polygon: transformed,
            _marker: PhantomData,
        }
    }
}

impl ToEpsg3035 for ProjectedPolygon<Epsg4326> {
    type Output = ProjectedPolygon<Epsg3035>;
    fn to_epsg_3035(&self) -> Self::Output {
        let crs = Proj::new_known_crs(EPSG_4326, EPSG_3035, None).unwrap();
        let transformed = self.polygon.transformed(&crs).unwrap();
        ProjectedPolygon {
            polygon: transformed,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod macro_methods {
    use super::*;
    use geo::polygon;

    #[test]
    fn assign_correctly() {
        let poly = polygon![
            (x: -111., y: 45.),
            (x: -111., y: 41.),
            (x: -104., y: 41.),
            (x: -104., y: 45.),
        ];

        let marco_method_point = ProjectedPolygon::<Epsg4326>::new(poly.clone());
        assert_eq!(marco_method_point.polygon(), &poly);
    }
}

#[cfg(test)]
mod projections {
    use super::*;
    use geo::polygon;

    #[test]
    fn change_when_converted() {
        let poly = polygon![
            (x: -111., y: 45.),
            (x: -111., y: 41.),
            (x: -104., y: 41.),
            (x: -104., y: 45.),
        ];
        let p = ProjectedPolygon::new(poly);
        let projected = p.to_epsg_3035();
        assert_ne!(p.polygon(), projected.polygon());
    }
}
