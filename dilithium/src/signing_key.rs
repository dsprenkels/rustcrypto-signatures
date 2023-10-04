use crate::poly32;
use crate::variant;
use crate::ByteArray;
use generic_array::GenericArray;
use signature::rand_core::CryptoRngCore;

// TODO: Implement interop with RustCrypto signature trait
pub struct SigningKey<V>
where
    V: variant::Variant,
{
    rho: ByteArray<variant::SEEDBYTES>,
    key: ByteArray<variant::SEEDBYTES>,
    tr: ByteArray<variant::CRHBYTES>,
    // TODO [dsprenkels]: Allow for lazy initialization of A
    mat: GenericArray<GenericArray<poly32::Poly32, V::L>, V::K>,
    s1: GenericArray<poly32::Poly32, V::L>,
    s2: GenericArray<poly32::Poly32, V::K>,
    t0: GenericArray<poly32::Poly32, V::K>,
}

impl<V> SigningKey<V>
where
    V: variant::Variant,
{
    pub fn random(rng: &mut impl CryptoRngCore) -> Self {
        todo!()
    }

    pub fn from_seed(seed: ByteArray<variant::SEEDBYTES>) -> Self {
        todo!()
    }

    pub fn to_bytes(&self) -> ByteArray<V::SigningKeySize> {
        todo!()
    }
}
