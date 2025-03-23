pub const EPSG_4326: &str = "EPSG:4326";

#[derive(Clone, Debug)]
pub struct Epsg4326;

pub trait ToEpsg4326 {
    type Output;

    fn to_epsg_4326(&self) -> Self::Output;
}
