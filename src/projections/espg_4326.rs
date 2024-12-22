use crate::geometries::ProjectedPoint;

#[derive(Clone, Debug)]
pub struct Espg4326;

pub trait ToEspg4326 {
    const ESPG_4326: &str = "ESPG:4326";
    fn to_espg_4326(&self) -> ProjectedPoint<Espg4326>;
}
