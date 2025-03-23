pub const EPSG_3035: &str = "EPSG:3035";

#[derive(Clone, Debug)]
pub struct Epsg3035;

pub trait ToEpsg3035 {
    type Output;

    fn to_epsg_3035(&self) -> Self::Output;
}
