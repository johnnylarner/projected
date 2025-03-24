use crate::ProjectedGeometry;

use super::Epsg4326;

pub fn make_laea_str(y: f64, x: f64) -> String {
    format!(
        "+proj=laea +lat_0={} +lon_0={} +x_0=0 +y_0=0 +ellps=WGS84 +units=m +no_defs",
        y, x
    )
}

#[derive(Clone, Debug)]
pub struct Laea;

pub trait ToLaea {
    type Output;

    fn to_laea(&self, origin: &ProjectedGeometry<Epsg4326>) -> Self::Output;
}
