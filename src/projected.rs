use std::marker::PhantomData;

use geo::{Geometry, Point};

use crate::{
    Epsg3035, Epsg4326, HasCentroid, Projectable, ProjectedPoint, ToEpsg3035,
    geometries::{ProjectedMultiPolygon, ProjectedPolygon},
};

pub struct ProjectedGeometry<P: Projectable> {
    geometry: Geometry,
    _marker: PhantomData<P>,
}

impl ProjectedGeometry<Epsg4326> {
    pub fn new(geometry: Geometry) -> ProjectedGeometry<Epsg4326> {
        Self {
            geometry,
            _marker: PhantomData,
        }
    }
}

impl<P> From<ProjectedPolygon<P>> for ProjectedGeometry<P>
where
    P: Projectable,
{
    fn from(value: ProjectedPolygon<P>) -> Self {
        let geo: Geometry = value.into();
        Self {
            geometry: geo,
            _marker: PhantomData,
        }
    }
}

impl ToEpsg3035 for ProjectedGeometry<Epsg4326> {
    type Output = ProjectedGeometry<Epsg3035>;
    fn to_epsg_3035(&self) -> Self::Output {
        match &self.geometry {
            Geometry::Point(p) => {
                let projectable = ProjectedPoint::new(*p);
                let projected = projectable.to_epsg_3035();
                ProjectedGeometry {
                    geometry: projected.into(),
                    _marker: PhantomData,
                }
            }
            Geometry::Polygon(p) => {
                let projectable = ProjectedPolygon::new(p.clone());
                let projected = projectable.to_epsg_3035();
                ProjectedGeometry {
                    geometry: projected.into(),
                    _marker: PhantomData,
                }
            }
            Geometry::MultiPolygon(p) => {
                let projectable = ProjectedMultiPolygon::new(p.clone());
                let projected = projectable.to_epsg_3035();
                ProjectedGeometry {
                    geometry: projected.into(),
                    _marker: PhantomData,
                }
            }
            _ => unreachable!("unsupported geometry"),
        }
    }
}

impl<P> HasCentroid for ProjectedGeometry<P>
where
    P: Projectable,
{
    fn centriod(&self) -> Point {
        match &self.geometry {
            Geometry::Point(p) => p.centriod(),
            Geometry::Polygon(p) => p.centriod(),
            Geometry::MultiPolygon(mp) => mp.centriod(),
            _ => unreachable!("unsupported geometry"),
        }
    }
}

#[cfg(test)]
mod projections {
    use super::*;
    use geo::{MultiPolygon, point, polygon};

    fn point() -> Geometry {
        let point = point!(x: -111., y: 45.);
        point.into()
    }

    fn poly() -> Geometry {
        let poly = polygon![
            (x: -111., y: 45.),
            (x: -111., y: 41.),
            (x: -104., y: 41.),
            (x: -104., y: 45.),
        ];
        poly.into()
    }

    fn multipoly() -> Geometry {
        let poly = MultiPolygon::new(
            [polygon![
                (x: -111., y: 45.),
                (x: -111., y: 41.),
                (x: -104., y: 41.),
                (x: -104., y: 45.),
            ]]
            .to_vec(),
        );
        poly.into()
    }

    #[test]
    fn change_when_converted() {
        let geom = point();
        let projectable = ProjectedGeometry::new(geom);
        let projected = projectable.to_epsg_3035();
        assert_ne!(projectable.geometry, projected.geometry);

        let geom = poly();
        let projectable = ProjectedGeometry::new(geom);
        let projected = projectable.to_epsg_3035();
        assert_ne!(projectable.geometry, projected.geometry);

        let geom = multipoly();
        let projectable = ProjectedGeometry::new(geom);
        let projected = projectable.to_epsg_3035();
        assert_ne!(projectable.geometry, projected.geometry);
    }

    #[test]
    fn does_not_change_enum_variant() {
        let geom = point();
        let projectable = ProjectedGeometry::new(geom);
        let projected = projectable.to_epsg_3035();
        assert!(matches!(projected.geometry, Geometry::Point(_)));

        let geom = poly();
        let projectable = ProjectedGeometry::new(geom);
        let projected = projectable.to_epsg_3035();
        assert!(matches!(projected.geometry, Geometry::Polygon(_)));

        let geom = multipoly();
        let projectable = ProjectedGeometry::new(geom);
        let projected = projectable.to_epsg_3035();
        assert!(matches!(projected.geometry, Geometry::MultiPolygon(_)));
    }
}
