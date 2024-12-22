pub const ESPG_4326: &str = "ESPG:4326";

#[derive(Clone, Debug)]
pub struct Espg4326;

pub trait ToEspg4326 {
    type Output;

    fn to_espg_4326(&self) -> Self::Output;
}
