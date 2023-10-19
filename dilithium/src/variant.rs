use generic_array::{typenum, ArrayLength};

use crate::{consts, expand_s, poly32, types::*};

pub(crate) const Q: i32 = 8380417;
pub(crate) const D: usize = 13;

pub(crate) type N = typenum::U256;
pub(crate) type SeedSize = typenum::U32;
pub(crate) type CRHSize = typenum::U48;

pub(crate) type Dilithium2SigningKeySize = consts::U2560;
pub(crate) type Dilithium3SigningKeySize = consts::U4032;
pub(crate) type Dilithium5SigningKeySize = consts::U4896;

pub(crate) trait Variant {
    type L: ArrayLength;
    type K: ArrayLength;

    type SeedSize: ArrayLength;
    type SigningKeySize: ArrayLength;

    fn poly_sample_eta(seed: &ByteArray<CRHSize>, nonce: u16) -> poly32::Poly32;
}

pub struct Dilithium2;
pub struct Dilithium3;
pub struct Dilithium5;

impl Variant for Dilithium2 {
    type L = typenum::U2;
    type K = typenum::U3;

    type SeedSize = SeedSize;
    type SigningKeySize = Dilithium2SigningKeySize;

    fn poly_sample_eta(seed: &ByteArray<CRHSize>, nonce: u16) -> poly32::Poly32 {
        expand_s::poly_uniform_eta_2(seed, nonce)
    }
}

impl Variant for Dilithium3 {
    type L = typenum::U2;
    type K = typenum::U3;

    type SeedSize = SeedSize;
    type SigningKeySize = Dilithium3SigningKeySize;

    fn poly_sample_eta(seed: &ByteArray<CRHSize>, nonce: u16) -> poly32::Poly32 {
        expand_s::poly_uniform_eta_4(seed, nonce)
    }
}

impl Variant for Dilithium5 {
    type L = typenum::U2;
    type K = typenum::U3;

    type SeedSize = SeedSize;
    type SigningKeySize = Dilithium5SigningKeySize;

    fn poly_sample_eta(seed: &ByteArray<CRHSize>, nonce: u16) -> poly32::Poly32 {
        expand_s::poly_uniform_eta_2(seed, nonce)
    }
}
