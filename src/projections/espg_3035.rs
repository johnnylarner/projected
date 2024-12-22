pub const ESPG_3035: &str = "ESPG:3035";

#[derive(Clone, Debug)]
pub struct Espg3035;

pub trait ToEspg3035 {
    type Output;

    fn to_espg_3035(&self) -> Self::Output;
}
