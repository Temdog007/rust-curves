// Code transpiled from Three JS
// https://github.com/mrdoob/three.js/blob/dev/src/extras/curves/EllipseCurve.js

use nalgebra::*;
use num_traits::*;

use super::*;

#[cfg(feature = "serde-serialize")]
use serde::*;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct EllipseCurve<N: CurveScalar> {
    pub x: N,
    pub y: N,
    pub x_radius: N,
    pub y_radius: N,
    pub start_angle: N,
    pub end_angle: N,
    pub clockwise: bool,
    pub rotation: UnitQuaternion<N>,
}

impl<N: CurveScalar> Default for EllipseCurve<N> {
    fn default() -> Self {
        Self {
            x: N::zero(),
            y: N::zero(),
            x_radius: N::one(),
            y_radius: N::one(),
            start_angle: N::zero(),
            end_angle: N::two_pi(),
            clockwise: false,
            rotation: UnitQuaternion::identity(),
        }
    }
}

impl<N: CurveScalar> Curve<N> for EllipseCurve<N> {
    fn valid(&self) -> bool {
        self.x_radius > N::zero() && self.y_radius > N::zero()
    }
    fn get_point_mut(&self, t: N, v: &mut Vector3<N>) {
        let mut delta_angle = self.end_angle - self.start_angle;
        let same_points = delta_angle < N::epsilon();

        while delta_angle < N::zero() {
            delta_angle += N::two_pi();
        }
        while delta_angle > N::two_pi() {
            delta_angle -= N::two_pi();
        }

        if delta_angle < N::epsilon() {
            delta_angle = if same_points { N::zero() } else { N::two_pi() };
        }

        if self.clockwise && !same_points {
            delta_angle = if delta_angle == N::two_pi() {
                -N::two_pi()
            } else {
                delta_angle - N::two_pi()
            };
        }

        let angle = self.start_angle + t * delta_angle;
        let x = self.x + self.x_radius * Float::cos(angle);
        let y = self.y + self.y_radius * Float::sin(angle);

        *v = self
            .rotation
            .transform_vector(&Vector3::new(x, y, N::zero()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cgmath::assert_relative_eq;

    #[test]
    fn test_points() {
        let curve = EllipseCurve::<f32>::default();
        let mut curve2 = curve;
        curve2.rotation = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), f32::frac_pi_2());

        for i in 0..16 {
            let t = i as f32 / 16f32;
            let angle = f32::two_pi() * t;
            let v = Vector3::new(angle.cos(), angle.sin(), 0f32);
            let v2 = Vector3::new(angle.cos(), 0f32, angle.sin());
            assert_eq!(curve.get_point(t), v, "Angle {} Index {}", angle, i);
            assert_relative_eq!(curve2.get_point(t), v2, epsilon = 1e-4f32);
        }
    }

    #[test]
    fn test_length() {
        let curve = EllipseCurve::<f32>::default();

        assert_relative_eq!(curve.get_length(512), f32::pi() * 2f32, epsilon = 1e-4f32);
    }
}
