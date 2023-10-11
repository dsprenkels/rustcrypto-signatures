use generic_array::{functional::FunctionalSequence, typenum::Unsigned, GenericArray};

use crate::{ntt, reduce, variant};

#[derive(Clone, Copy, Default)]
pub(crate) struct Poly32 {
    pub(crate) coeffs: GenericArray<i32, variant::N>,
}

impl Poly32 {
    pub(crate) fn ntt(&self) -> Self {
        // TODO: Don't pass by reference
        let mut poly = self.clone();
        ntt::poly_ntt(&mut poly);
        poly
    }

    pub(crate) fn invntt(&self) -> Self {
        // TODO: Don't pass by reference
        let mut poly = self.clone();
        ntt::poly_invntt_tomont(&mut poly);
        poly
    }

    pub(crate) fn pointwise_mul_montgomery(&self, other: &Self) -> Self {
        let mut iter = other.coeffs.iter();
        let coeffs = self.coeffs.map(|a| {
            let a = i64::from(a);
            let b = i64::from(*iter.next().unwrap());
            let c_unreduced = i64::wrapping_mul(a, b);
            let c_montgomery = reduce::montgomery_reduce(c_unreduced);
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
            let (t1i, t0i) = reduce::power2round(x);
            t1.coeffs[i] = t1i;
            t0.coeffs[i] = t0i;
            i += 1;
        });
        (t1, t0)
    }
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
        self.map(Poly32::ntt)
    }
    fn invntt(&self) -> Self {
        self.map(Poly32::invntt)
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
