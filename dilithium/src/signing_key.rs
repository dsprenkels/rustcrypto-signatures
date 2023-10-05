use signature::rand_core::CryptoRngCore;

use crate::{keypair, poly32, variant, ByteArray};

// TODO: Implement interop with RustCrypto signature trait
pub struct SigningKey<V>
where
    V: variant::Variant,
{
    // TODO: Cache A matrix and ntt'd s1, s2, t0.
    pub(crate) rho: ByteArray<variant::SEEDBYTES>,
    pub(crate) key: ByteArray<variant::SEEDBYTES>,
    pub(crate) s1: crate::VecL<V, poly32::Poly32>,
    pub(crate) s2: crate::VecK<V, poly32::Poly32>,
    pub(crate) t0: crate::VecK<V, poly32::Poly32>,
}

impl SigningKey<variant::Dilithium2> {
    pub fn random(rng: &mut impl CryptoRngCore) -> Self {
        keypair::keypair_random::<variant::Dilithium2>(variant::DILITHIUM2_IMPL, rng)
    }

    pub fn from_seed(seed: ByteArray<variant::SEEDBYTES>) -> Self {
        keypair::keypair_from_seed::<variant::Dilithium2>(variant::DILITHIUM2_IMPL, &seed)
    }

    pub fn to_bytes(&self) -> ByteArray<<variant::Dilithium2 as variant::Variant>::SigningKeySize> {
        todo!()
    }
}
