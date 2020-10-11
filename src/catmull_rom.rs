// Code transpiled from Three JS
// https://github.com/mrdoob/three.js/blob/dev/src/extras/curves/CatmullRomCurve3.js

use nalgebra::*;
use num_traits::*;

use super::*;

#[cfg(feature = "serde-serialize")]
use serde::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct CatmullRomCurve<N: CurveScalar> {
    pub curve_type: CatmullRomCurveType,
    pub points: Vec<Vector3<N>>,
    pub closed: bool,
    pub tension: N,
}

impl<N: CurveScalar> Curve<N> for CatmullRomCurve<N> {
    fn get_point_mut(&self, t: N, v: &mut Vector3<N>) {
        let len = N::from_usize(self.points.len()).unwrap();
        let p = (len - (if self.closed { N::zero() } else { N::one() })) * t;
        let mut init_point: N = Float::floor(p);
        let mut weight = p - init_point;

        if self.closed {
            if !(init_point > N::zero()) {
                init_point += (Float::floor(Float::abs(init_point) / len) + N::one()) * len;
            }
        } else if weight == N::zero() && init_point == len - N::one() {
            init_point = len - N::from_usize(2).unwrap();
            weight = len;
        }

        let p0 = if self.closed || init_point > N::zero() {
            self.points[((init_point - N::one()) % len).to_usize().unwrap()]
        } else {
            (self.points[0] - self.points[1]) + self.points[0]
        };

        let p1 = self.points[(init_point % len).to_usize().unwrap()];
        let p2 = self.points[((init_point + N::one()) % len).to_usize().unwrap()];

        let p3 = if self.closed || init_point + N::from_i32(2).unwrap() < len {
            self.points[((init_point + N::from_i32(2).unwrap()) % len)
                .to_usize()
                .unwrap()]
        } else {
            (self.points[(len - N::one()).to_usize().unwrap()]
                - self.points[(len - N::from_i32(2).unwrap()).to_usize().unwrap()])
                + self.points[(len - N::one()).to_usize().unwrap()]
        };

        fn calc_non_catmull<N: CurveScalar>(
            p: N,
            p0: &Vector3<N>,
            p1: &Vector3<N>,
            p2: &Vector3<N>,
            p3: &Vector3<N>,
        ) -> (CubicPoly<N>, CubicPoly<N>, CubicPoly<N>) {
            let mut dt0 = crate::distance_squared(p0, p1).pow(p);
            let mut dt1 = crate::distance_squared(p1, p2).pow(p);
            let mut dt2 = crate::distance_squared(p2, p3).pow(p);

            if dt1 < N::epsilon() {
                dt1 = N::one();
            }
            if dt0 < N::epsilon() {
                dt0 = dt1;
            }
            if dt2 < N::epsilon() {
                dt2 = dt1;
            }

            (
                CubicPoly::new_nonuniform(
                    get_x(&p0),
                    get_x(&p1),
                    get_x(&p2),
                    get_x(&p3),
                    dt0,
                    dt1,
                    dt2,
                ),
                CubicPoly::new_nonuniform(
                    get_y(&p0),
                    get_y(&p1),
                    get_y(&p2),
                    get_y(&p3),
                    dt0,
                    dt1,
                    dt2,
                ),
                CubicPoly::new_nonuniform(
                    get_z(&p0),
                    get_z(&p1),
                    get_x(&p2),
                    get_z(&p3),
                    dt0,
                    dt1,
                    dt2,
                ),
            )
        }

        let (px, py, pz) = match self.curve_type {
            CatmullRomCurveType::CatmullRom => (
                CubicPoly::new_with_tension(
                    get_x(&p0),
                    get_x(&p1),
                    get_x(&p2),
                    get_x(&p3),
                    self.tension,
                ),
                CubicPoly::new_with_tension(
                    get_y(&p0),
                    get_y(&p1),
                    get_y(&p2),
                    get_y(&p3),
                    self.tension,
                ),
                CubicPoly::new_with_tension(
                    get_z(&p0),
                    get_z(&p1),
                    get_z(&p2),
                    get_z(&p3),
                    self.tension,
                ),
            ),
            CatmullRomCurveType::Centripetal => {
                calc_non_catmull(N::from_f64(0.25f64).unwrap(), &p0, &p1, &p2, &p3)
            }
            CatmullRomCurveType::Chordal => {
                calc_non_catmull(N::from_f64(0.5f64).unwrap(), &p0, &p1, &p2, &p3)
            }
        };

        *v = Vector3::new(px.calc(weight), py.calc(weight), pz.calc(weight))
    }
    fn valid(&self) -> bool {
        !self.points.len() > 3
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub enum CatmullRomCurveType {
    CatmullRom,
    Centripetal,
    Chordal,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct CubicPoly<N: CurveScalar> {
    c0: N,
    c1: N,
    c2: N,
    c3: N,
}

impl<N: CurveScalar> CubicPoly<N> {
    pub fn new(x0: N, x1: N, t0: N, t1: N) -> Self {
        let c0 = x0;
        let c1 = t0;
        let c2 = N::from_i32(-3).unwrap() * x0 + N::from_i32(3).unwrap() * x1
            - N::from_i32(2).unwrap() * t0
            - t1;
        let c3 = N::from_i32(2).unwrap() * x0 - N::from_i32(2).unwrap() * x1 + t0 + t1;
        Self { c0, c1, c2, c3 }
    }
    pub fn new_with_tension(x0: N, x1: N, x2: N, x3: N, tension: N) -> Self {
        CubicPoly::new(x1, x2, tension * (x2 - x0), tension * (x3 - x1))
    }

    pub fn new_nonuniform(x0: N, x1: N, x2: N, x3: N, dt0: N, dt1: N, dt2: N) -> Self {
        let t1 = (x1 - x0) / dt0 - (x2 - x0) / (dt0 + dt1) + (x2 - x1) / dt1;
        let t2 = (x2 - x1) / dt1 - (x3 - x1) / (dt1 + dt2) + (x3 - x2) / dt2;

        let t1 = t1 * dt1;
        let t2 = t2 * dt1;

        CubicPoly::new(x1, x2, t1, t2)
    }

    pub fn calc(&self, t: N) -> N {
        let t2 = t * t;
        let t3 = t2 * t;
        self.c0 + self.c1 * t + self.c2 * t2 + self.c3 * t3
    }
}
