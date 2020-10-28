use nalgebra::*;

pub mod linear;
pub use linear::*;

pub mod catmull_rom;
pub use catmull_rom::*;

pub mod curve;
pub use curve::*;

pub mod ellipse;
pub use ellipse::*;

pub mod bezier;
pub use bezier::*;

pub(crate) fn get_x<N: CurveScalar>(v: &Vector3<N>) -> N {
    unsafe { *v.get_unchecked(0) }
}
pub(crate) fn get_y<N: CurveScalar>(v: &Vector3<N>) -> N {
    unsafe { *v.get_unchecked(1) }
}
pub(crate) fn get_z<N: CurveScalar>(v: &Vector3<N>) -> N {
    unsafe { *v.get_unchecked(2) }
}

pub(crate) fn distance<N: CurveScalar>(a: &Vector3<N>, b: &Vector3<N>) -> N {
    (a - b).norm()
}

pub(crate) fn distance_squared<N: CurveScalar>(a: &Vector3<N>, b: &Vector3<N>) -> N {
    (a - b).norm_squared()
}
pub trait PointContainer<N: CurveScalar>:
    std::ops::Index<usize, Output = Vector3<N>> + std::ops::IndexMut<usize, Output = Vector3<N>>
{
    fn add_point(&mut self, t: Vector3<N>);
    fn point_count(&self) -> usize;
}

impl<N: CurveScalar> PointContainer<N> for Vec<Vector3<N>> {
    fn add_point(&mut self, t : Vector3<N>){
        self.push(t);
    }
    fn point_count(&self) -> usize{
        self.len()
    }
}
