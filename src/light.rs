use crate::geometry::Point3f;
use crate::spectrum::*;

pub struct Light {
    pub pos: Point3f,
    pub power: Spectrum,
}
