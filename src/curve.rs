use nalgebra::*;
use num_traits::*;

pub trait Curve<N: CurveScalar> {
    fn get_point(&self, t: N) -> Vector3<N> {
        debug_assert!(self.valid());
        let mut v = Vector3::zeros();
        self.get_point_mut(t, &mut v);
        v
    }

    fn get_point_mut(&self, t: N, v: &mut Vector3<N>);

    fn get_tangent(&self, t: N, delta: N) -> Vector3<N> {
        let mut t1 = t - delta;
        let mut t2 = t + delta;

        if t1 < N::zero() {
            t1 = N::zero();
        }
        if t2 > N::one() {
            t2 = N::one();
        }

        let pt1 = self.get_point(t1);
        let pt2 = self.get_point(t2);
        (pt2 - pt1).normalize()
    }

    fn valid(&self) -> bool;
}

pub trait CurveScalar:
    Scalar
    + Float
    + Pow<Self, Output = Self>
    + One
    + SimdComplexField<SimdRealField = Self>
    + RealField
    + FromPrimitive
    + ToPrimitive
{
}

impl CurveScalar for f32 {}
impl CurveScalar for f64 {}