use std::marker::PhantomData;

use geo::Point;

use crate::projections::{Espg4326, ToEspg4326};

#[derive(Clone, Debug)]
pub struct ProjectedPoint<Projection> {
    point: Point,
    _marker: PhantomData<Projection>,
}

impl ToEspg4326 for ProjectedPoint<Espg4326> {
    type Output = ProjectedPoint<Espg4326>;
    fn to_espg_4326(&self) -> Self::Output {
        self.clone()
    }
}
