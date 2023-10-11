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

#[cfg(test)]
mod tests {
    use signature::rand_core::{CryptoRng, RngCore};

    struct DummyRng {
        pub(crate) ctr: u64,
    }

    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 {
            let mut x = [0; 4];
            self.fill_bytes(&mut x);
            u32::from_le_bytes(x)
        }

        fn next_u64(&mut self) -> u64 {
            let mut x = [0; 8];
            self.fill_bytes(&mut x);
            u64::from_le_bytes(x)
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for b in dest.iter_mut() {
                *b = self.ctr as u8;
                self.ctr += 1;
            }
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), signature::rand_core::Error> {
            Ok(self.fill_bytes(dest))
        }
    }

    impl CryptoRng for DummyRng {}

    #[test]
    fn test_signing_key_random() {
        let mut rng = DummyRng { ctr: 0 };
        let sk = super::SigningKey::<super::variant::Dilithium2>::random(&mut rng);
    }
}
