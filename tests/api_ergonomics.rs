use geo::point;
use projected::{Epsg3035, Epsg4326, ProjectedPoint, ToEpsg3035};

#[test]
fn enable_typed_function_args() {
    fn covert_to_3035(p: ProjectedPoint<Epsg4326>) -> ProjectedPoint<Epsg3035> {
        p.to_epsg_3035()
    }
    let p = point! { x: 181.2, y: 51.79 };
    let p = ProjectedPoint::new(p);
    let _projected = covert_to_3035(p);
}
