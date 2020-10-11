use nalgebra::*;

use super::*;

#[cfg(feature = "seride-serialize")]
use serde::*;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct EllipseCurve<N: CurveScalar> {
    x: N,
    y: N,
    x_radius: N,
    y_radius: N,
    start_angle: N,
    end_angle: N,
    clockwise: bool,
    rotation: N,
}

impl<N: CurveScalar> Curve<N> for EllipseCurve<N> {
    fn valid(&self) -> bool {
        self.x_radius > N::zero() && self.y_radius > N::zero() && self.start_angle != self.end_angle
    }
}
