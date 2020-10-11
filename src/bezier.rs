use nalgebra::*;

use super::*;

#[cfg(feature = "seride-serialize")]
use serde::*;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Bezier<N: CurveScalar>{}