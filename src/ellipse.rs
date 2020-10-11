use nalgebra::*;
use num_traits::*;

use super::*;

#[cfg(feature = "seride-serialize")]
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
    pub rotation: N,
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
            rotation: N::zero(),
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

        let (x, y) = if self.rotation == N::zero() {
            (x, y)
        } else {
            let cos = Float::cos(self.rotation);
            let sin = Float::sin(self.rotation);

            let tx = x - self.x;
            let ty = y - self.y;
            (tx * cos - ty * sin + self.x, tx * sin + ty * cos + self.y)
        };

        *v = Vector3::new(x, y, N::zero())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let curve = EllipseCurve::<f32>::default();

        for i in 0..16 {
            let t = i as f32 / 16f32;
            let angle = f32::two_pi() * t;
            let v = Vector3::new(angle.cos(), angle.sin(), 0f32);
            assert_eq!(curve.get_point(t), v, "Angle {} Index {}", angle, i);
        }
    }
}
