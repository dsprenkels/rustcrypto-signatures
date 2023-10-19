pub(crate) use generic_array::{functional::FunctionalSequence, typenum::Unsigned, GenericArray};

use crate::{poly32, variant};

pub(crate) type ByteArray<Size> = GenericArray<u8, Size>;
pub(crate) type VecL<V, T> = GenericArray<T, <V as variant::Variant>::L>;
pub(crate) type VecK<V, T> = GenericArray<T, <V as variant::Variant>::K>;
pub(crate) type Poly32VecL<V> = VecL<V, poly32::Poly32>;
pub(crate) type Poly32VecK<V> = VecK<V, poly32::Poly32>;
pub(crate) type Matrix<V> = VecK<V, VecL<V, poly32::Poly32>>;

pub(crate) struct SigningKey<V>
where
    V: variant::Variant,
{
    // TODO: Cache A matrix and ntt'd s1, s2, t0.
    pub(crate) rho: ByteArray<variant::SeedSize>,
    pub(crate) key: ByteArray<variant::SeedSize>,
    pub(crate) s1: VecL<V, poly32::Poly32>,
    pub(crate) s2: VecK<V, poly32::Poly32>,
    pub(crate) t0: VecK<V, poly32::Poly32>,
}
