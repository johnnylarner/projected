use std::marker::PhantomData;

use geo::Geometry;

pub struct ProjectedGeometry<Projection> {
    geometry: Geometry,
    _marker: PhantomData<Projection>,
}
