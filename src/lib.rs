use nalgebra::*;
use num_traits::*;

pub mod linear;
pub use linear::*;

pub trait Curve<N: CurveScalar> {
    fn get_point(&self, t: N) -> Vector3<N>;

    fn valid(&self) -> bool;
}

pub trait CurveScalar:
    Scalar + Float + One + SimdComplexField<SimdRealField = Self> + RealField
{
}

impl CurveScalar for f32{}
impl CurveScalar for f64{}