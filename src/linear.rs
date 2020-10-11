use nalgebra::*;

use super::*;

#[cfg(feature = "seride-serialize")]
use serde::*;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct LinearCurve<N: CurveScalar> {
    pub start: Vector3<N>,
    pub end: Vector3<N>,
}

impl<N: CurveScalar> Curve<N> for LinearCurve<N> {
    fn get_point_mut(&self, t: N, v: &mut Vector3<N>) {
        *v = self.start.scale(N::one() - t) + (&self.end.scale(t))
    }
    fn valid(&self) -> bool {
        self.start != self.end
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_point() {
        let curve = LinearCurve {
            start: Vector3::zeros(),
            end: Vector3::from_element(1f32),
        };

        assert!(curve.valid());
        for i in 0..10 {
            let t = i as f32 / 10f32;
            assert_eq!(curve.get_point(t), Vector3::from_element(t));
        }

        let curve = LinearCurve {
            start: Vector3::zeros(),
            end: Vector3::from_element(1f64),
        };

        assert!(curve.valid());
        let points: Vec<_> = get_points(&curve, 10).collect();
        for i in 0..10 {
            let t = i as f64 / 10f64;
            let point = curve.get_point(t);
            assert_eq!(points[i], point);
            assert_eq!(point, Vector3::from_element(t));
        }
    }

    #[test]
    fn test_valid() {
        let curve: LinearCurve<f64> = LinearCurve {
            start: Vector3::zeros(),
            end: Vector3::zeros(),
        };

        assert!(!curve.valid());
    }

    #[test]
    fn test_get_length() {
        let curve = LinearCurve {
            start: Vector3::zeros(),
            end: Vector3::from_element(1f32).normalize(),
        };

        for i in 2..100 {
            let len = curve.get_length(i);
            assert!(
                (len - 1f32).abs() < 1e-4f32,
                "Failed for {}. Length = {}",
                i,
                len
            );
        }
    }

    #[test]
    #[should_panic]
    fn test_get_length_panic() {
        let curve: LinearCurve<f64> = LinearCurve {
            start: Vector3::zeros(),
            end: Vector3::zeros(),
        };

        curve.get_length(0);
    }
}
