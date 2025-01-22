use std::marker::PhantomData;

use geo::MultiPolygon;
use proj::{Proj, Transform};

use crate::projections::{Epsg3035, Epsg4326, ToEpsg3035, ToEpsg4326, EPSG_3035, EPSG_4326};

#[derive(Clone, Debug)]
pub struct ProjectedMultiPolygon<Projection> {
    multi_polygon: MultiPolygon,
    _marker: PhantomData<Projection>,
}

impl ProjectedMultiPolygon<Epsg4326> {
    pub fn new(multi_polygon: MultiPolygon) -> ProjectedMultiPolygon<Epsg4326> {
        Self {
            multi_polygon,
            _marker: PhantomData,
        }
    }
}

macro_rules! polygon_value {
    ( for $( $t:ty ),* ) => {
        $(
        impl ProjectedMultiPolygon<$t> {
            pub fn multi_polygon(&self) -> &MultiPolygon {
                &self.multi_polygon
            }
        }
        )*
    };
}

polygon_value!(for Epsg4326, Epsg3035);

impl ToEpsg4326 for ProjectedMultiPolygon<Epsg3035> {
    type Output = ProjectedMultiPolygon<Epsg4326>;
    fn to_epsg_4326(&self) -> Self::Output {
        let crs = Proj::new_known_crs(EPSG_3035, EPSG_4326, None).unwrap();
        let transformed = self.multi_polygon.transformed(&crs).unwrap();
        ProjectedMultiPolygon {
            multi_polygon: transformed,
            _marker: PhantomData,
        }
    }
}

impl ToEpsg3035 for ProjectedMultiPolygon<Epsg4326> {
    type Output = ProjectedMultiPolygon<Epsg3035>;
    fn to_epsg_3035(&self) -> Self::Output {
        let crs = Proj::new_known_crs(EPSG_4326, EPSG_3035, None).unwrap();
        let transformed = self.multi_polygon.transformed(&crs).unwrap();
        ProjectedMultiPolygon {
            multi_polygon: transformed,
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
        let poly = MultiPolygon::new(
            [polygon![
                (x: -111., y: 45.),
                (x: -111., y: 41.),
                (x: -104., y: 41.),
                (x: -104., y: 45.),
            ]]
            .to_vec(),
        );

        let marco_method_point = ProjectedMultiPolygon::<Epsg4326>::new(poly.clone());
        assert_eq!(marco_method_point.multi_polygon(), &poly);
    }
}

#[cfg(test)]
mod projections {
    use super::*;
    use geo::polygon;

    #[test]
    fn change_when_converted() {
        let poly = MultiPolygon::new(
            [polygon![
                (x: -111., y: 45.),
                (x: -111., y: 41.),
                (x: -104., y: 41.),
                (x: -104., y: 45.),
            ]]
            .to_vec(),
        );
        let p = ProjectedMultiPolygon::new(poly);
        let projected = p.to_epsg_3035();
        assert_ne!(p.multi_polygon(), projected.multi_polygon());
    }
}
