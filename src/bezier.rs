// Code transpiled from Three JS
// https://github.com/mrdoob/three.js/blob/dev/src/extras/curves/CubicBezierCurve3.js

use nalgebra::*;

use super::*;

#[cfg(feature = "serde-serialize")]
use serde::*;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct BezierCurve<N: CurveScalar> {
    v0: Vector3<N>,
    v1: Vector3<N>,
    v2: Vector3<N>,
    v3: Vector3<N>,
}

impl<N: CurveScalar> Curve<N> for BezierCurve<N> {
    fn valid(&self) -> bool {
        let arr = [self.v0, self.v1, self.v2, self.v3];
        (1..arr.len()).any(|i| arr[i..].contains(&arr[i - 1]))
    }
    fn get_point_mut(&self, t: N, v: &mut Vector3<N>) {
        *v = Vector3::new(
            cubic_bezier(
                t,
                get_x(&self.v0),
                get_x(&self.v1),
                get_x(&self.v2),
                get_x(&self.v3),
            ),
            cubic_bezier(
                t,
                get_y(&self.v0),
                get_y(&self.v1),
                get_y(&self.v2),
                get_y(&self.v3),
            ),
            cubic_bezier(
                t,
                get_z(&self.v0),
                get_z(&self.v1),
                get_z(&self.v2),
                get_z(&self.v3),
            ),
        )
    }
}

pub fn cubic_bezier<N: CurveScalar>(t: N, p0: N, p1: N, p2: N, p3: N) -> N {
    cubic_bezier_p0(t, p0)
        + cubic_bezier_p1(t, p1)
        + cubic_bezier_p2(t, p2)
        + cubic_bezier_p3(t, p3)
}

pub fn cubic_bezier_p0<N: CurveScalar>(t: N, p: N) -> N {
    let k = N::one() - t;
    k * k * k * p
}

pub fn cubic_bezier_p1<N: CurveScalar>(t: N, p: N) -> N {
    let k = N::one() - t;
    N::from_usize(3).unwrap() * k * k * t * p
}

pub fn cubic_bezier_p2<N: CurveScalar>(t: N, p: N) -> N {
    N::from_usize(3).unwrap() * (N::one() - t) * t * t * p
}

pub fn cubic_bezier_p3<N: CurveScalar>(t: N, p: N) -> N {
    t * t * t * p
}
