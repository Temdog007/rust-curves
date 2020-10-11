use nalgebra::*;

use super::*;

#[cfg(feature = "seride-serialize")]
use serde::*;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Linear<N: CurveScalar> {
    pub start: Vector3<N>,
    pub end: Vector3<N>,
}

impl<N: CurveScalar> Curve<N> for Linear<N> {
    fn get_point(&self, t: N) -> Vector3<N> {
        self.start.scale(N::one() - t) + (&self.end.scale(t))
    }
    fn valid(&self) -> bool {
        self.start != self.end
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let curve = Linear {
            start: Vector3::zeros(),
            end: Vector3::from_element(1f32),
        };

        assert!(curve.valid());
        for i in 0..10 {
            let t = i as f32 / 10f32;
            assert_eq!(curve.get_point(t), Vector3::from_element(t));
        }

        let curve = Linear {
            start: Vector3::zeros(),
            end: Vector3::from_element(1f64),
        };

        assert!(curve.valid());
        let points = curve.get_points(10);
        for i in 0..10 {
            let t = i as f64 / 10f64;
            let point = curve.get_point(t);
            assert_eq!(points[i], point);
            assert_eq!(point, Vector3::from_element(t));
        }
    }

    #[test]
    fn test2() {
        let curve: Linear<f64> = Linear {
            start: Vector3::zeros(),
            end: Vector3::zeros(),
        };

        assert!(!curve.valid());
    }
}
