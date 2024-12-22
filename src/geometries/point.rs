use std::marker::PhantomData;

use geo::Point;
use proj::Proj;

use crate::projections::{Espg3035, Espg4326, ToEspg3035, ToEspg4326, ESPG_3035, ESPG_4326};

#[derive(Clone, Debug)]
pub struct ProjectedPoint<Projection> {
    point: Point,
    _marker: PhantomData<Projection>,
}

impl ToEspg4326 for ProjectedPoint<Espg3035> {
    type Output = ProjectedPoint<Espg4326>;
    fn to_espg_4326(&self) -> Self::Output {
        let crs = Proj::new_known_crs(ESPG_3035, ESPG_4326, None).unwrap();
        let transformed = crs.convert(self.point).unwrap();
        ProjectedPoint {
            point: transformed,
            _marker: PhantomData,
        }
    }
}

impl ToEspg3035 for ProjectedPoint<Espg4326> {
    type Output = ProjectedPoint<Espg3035>;
    fn to_espg_3035(&self) -> Self::Output {
        let crs = Proj::new_known_crs(ESPG_4326, ESPG_3035, None).unwrap();
        let transformed = crs.convert(self.point).unwrap();
        ProjectedPoint {
            point: transformed,
            _marker: PhantomData,
        }
    }
}
