macro_rules! dilithium_api {
    ($name:ident, $variant:ty, $sk_size:ty) => {
        pub mod $name {
            use signature::rand_core::CryptoRngCore;

            use crate::{consts, keypair, types::*, variant};

            #[repr(transparent)]
            pub struct SigningKey {
                inner: crate::types::SigningKey<$variant>,
            }

            impl SigningKey {
                #[must_use]
                pub fn random(rng: &mut impl CryptoRngCore) -> Self {
                    let inner = keypair::keypair_random::<$variant>(rng);
                    Self { inner }
                }

                #[must_use]
                pub fn from_seed(seed: &[u8]) -> Self {
                    let seed = ByteArray::<crate::variant::SeedSize>::from_slice(seed);
                    let inner = keypair::keypair_from_seed::<$variant>(&seed);
                    Self { inner }
                }

                #[must_use]
                pub fn to_bytes(&self) -> ByteArray<$sk_size> {
                    let mut buf = ByteArray::default();
                    (&mut buf[0..32]).copy_from_slice(&self.inner.rho);
                    (&mut buf[32..64]).copy_from_slice(&self.inner.key);
                    todo!("implement the rest of the secret key encoding");
                    buf
                }
            }
        }
    };
}

dilithium_api!(
    dilithium2,
    variant::Dilithium2,
    variant::Dilithium2SigningKeySize
);
dilithium_api!(
    dilithium3,
    variant::Dilithium3,
    variant::Dilithium3SigningKeySize
);
dilithium_api!(
    dilithium5,
    variant::Dilithium5,
    variant::Dilithium5SigningKeySize
);

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
        let _ = super::dilithium2::SigningKey::random(&mut rng);
    }
}
