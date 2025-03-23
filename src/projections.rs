mod epsg_3035;
mod epsg_4326;

pub use epsg_3035::{EPSG_3035, Epsg3035, ToEpsg3035};
pub use epsg_4326::{EPSG_4326, Epsg4326, ToEpsg4326};

pub trait Projectable {}

impl Projectable for Epsg4326 {}

impl Projectable for Epsg3035 {}
