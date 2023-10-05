use generic_array::{functional::FunctionalSequence, typenum::Unsigned, GenericArray};

use crate::{reduce, variant};

#[derive(Clone, Copy, Default)]
pub(crate) struct Poly32 {
    pub(crate) coeffs: GenericArray<i32, variant::N>,
}

impl Poly32 {
    pub(crate) fn ntt(&self) -> Self {
        todo!();
    }
    pub(crate) fn invntt(&self) -> Self {
        todo!();
    }
    pub(crate) fn pointwise_mul_montgomery(&self, other: &Self) -> Self {
        let mut iter = other.coeffs.iter();
        let coeffs = self.coeffs.map(|a| {
            let a = i64::from(a);
            let b = i64::from(*iter.next().unwrap());
            let c_unreduced = i64::wrapping_mul(a, b);
            let c_montgomery = montgomery_reduce(c_unreduced);
            c_montgomery
        });
        Poly32 { coeffs }
    }
    pub(crate) fn add(&self, other: &Self) -> Self {
        let mut iter = other.coeffs.iter();
        let coeffs = self
            .coeffs
            .map(|x| i32::wrapping_add(x, *iter.next().unwrap()));
        Poly32 { coeffs }
    }
    pub(crate) fn power2round(&self) -> (Self, Self) {
        let mut t1 = Self::default();
        let mut t0 = Self::default();
        let mut i = 0;
        let _ = self.coeffs.map(|x| {
            let (t1i, t0i) = power2round(x);
            t1.coeffs[i] = t1i;
            t0.coeffs[i] = t0i;
            i += 1;
        });
        (t1, t0)
    }
}

pub(crate) const fn power2round(a: i32) -> (i32, i32) {
    let a1 = (a + (1 << (variant::D - 1)) - 1) >> variant::D;
    let a0 = a - (a1 << variant::D);
    (a0, a1)
}

pub(crate) const fn reduce32(a: i32) -> i32 {
    let t = a.wrapping_add(1 << 22);
    let t = t >> 23;
    a - t * variant::Q
}

pub(crate) const fn montgomery_reduce(a: i64) -> i32 {
    const Q_I64: i64 = variant::Q as i64;
    const QINV_I64: i64 = reduce::Q_INV as i64;
    let t: i32 = ((a as i32 as i64).wrapping_mul(QINV_I64)) as i32;
    let t: i64 = a.wrapping_sub((t as i64).wrapping_mul(Q_I64)) >> 32;
    t as i32
}

pub(crate) fn caddq(mut a: i32) -> i32 {
    let mask = a >> 31; // TODO: Add an optimization barrier.
    a += mask & variant::Q;
    a
}

pub(crate) trait PolyVec
where
    Self: Sized,
{
    fn flat_map(&self, f: &dyn Fn(i32) -> i32) -> Self;
    fn ntt(&self) -> Self;
    fn invntt(&self) -> Self;
    fn pointwise_mul(&self, other: &Self) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn power2round(&self) -> (Self, Self);
}

impl<N: generic_array::ArrayLength> PolyVec for GenericArray<Poly32, N> {
    fn flat_map(&self, f: &dyn Fn(i32) -> i32) -> Self {
        self.map(|x| Poly32 {
            coeffs: x.coeffs.map(f),
        })
    }
    fn ntt(&self) -> Self {
        self.map(&Poly32::ntt)
    }
    fn invntt(&self) -> Self {
        self.map(&Poly32::invntt)
    }
    fn pointwise_mul(&self, other: &Self) -> Self {
        let mut iter = other.iter();
        self.map(|x| x.pointwise_mul_montgomery(iter.next().unwrap()))
    }
    fn add(&self, other: &Self) -> Self {
        let mut iter = other.iter();
        self.map(|x| x.add(iter.next().unwrap()))
    }
    fn power2round(&self) -> (Self, Self) {
        let mut t1 = Self::default();
        let mut t0 = Self::default();
        let mut i = 0;
        let _ = self.map(|x| {
            let (t1i, t0i) = x.power2round();
            t1[i] = t1i;
            t0[i] = t0i;
            i += 1;
        });
        (t1, t0)
    }
}

pub(crate) fn matrix_mul_montgomery<V: variant::Variant>(
    mat: &crate::Matrix<V>,
    s: &crate::Poly32VecL<V>,
) -> crate::Poly32VecK<V> {
    let mut t = crate::Poly32VecK::<V>::default();
    for row in 0..<V as variant::Variant>::K::USIZE {
        for col in 0..<V as variant::Variant>::L::USIZE {
            t[row] = t[row].add(&mat[row][col].pointwise_mul_montgomery(&s[col]));
        }
    }
    t
}
