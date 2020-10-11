use nalgebra::*;
use num_traits::*;

pub mod linear;
pub use linear::*;

pub trait Curve<N: CurveScalar> {
    fn get_point(&self, t: N) -> Vector3<N>;

    fn get_points(&self, count: usize) -> Vec<Vector3<N>> {
        (0..count)
            .map(|i| {
                let t = N::from_usize(i).unwrap() / N::from_usize(count).unwrap();
                self.get_point(t)
            })
            .collect()
    }

    fn valid(&self) -> bool;
}

pub trait CurveScalar:
    Scalar
    + Float
    + One
    + SimdComplexField<SimdRealField = Self>
    + RealField
    + FromPrimitive
    + ToPrimitive
     
{
}

impl CurveScalar for f32 {}
impl CurveScalar for f64 {}
