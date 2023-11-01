use generic_array::typenum::Unsigned;
use sha3::digest::{ExtendableOutput, Update, XofReader};
use signature::rand_core::CryptoRngCore;

use crate::{expand_a, expand_s, poly32, poly32::PolyVec, reduce, types::*, variant};

pub(crate) fn keypair_random<V: variant::Variant>(rng: &mut impl CryptoRngCore) -> SigningKey<V> {
    let mut seed = ByteArray::<variant::SeedSize>::default();
    rng.fill_bytes(&mut seed);
    keypair_from_seed(&seed)
}

pub(crate) fn keypair_from_seed<V: variant::Variant>(
    seed: &ByteArray<variant::SeedSize>,
) -> SigningKey<V> {
    // Generate seed buffers
    let mut xof = sha3::Shake256::default();
    xof.update(seed);
    let mut xofread = xof.finalize_xof();

    let mut rho = ByteArray::from_array([0; variant::SeedSize::USIZE]);
    let mut rhoprime = ByteArray::from_array([0; variant::CRHSize::USIZE]);
    let mut key = ByteArray::from_array([0; variant::SeedSize::USIZE]);

    xofread.read(&mut rho);
    xofread.read(&mut rhoprime);
    xofread.read(&mut key);
    drop(xofread);

    // Expand matrix A
    let mat_ntt = expand_a::expand_a::<V>(&mut rho);

    // Expand s1 and s2
    let (s1, nonce) = expand_s::expand_s::<V, V::L>(&mut rhoprime, 0);
    let (s2, nonce) = expand_s::expand_s::<V, V::K>(&mut rhoprime, nonce);
    assert_eq!(nonce, V::L::U16 + V::K::U16);

    // Matrix-vector multiplication
    let s1_ntt = s1.ntt();
    let as1_ntt_montgomery = poly32::matrix_mul_montgomery::<V>(&mat_ntt, &s1_ntt);
    let as1_ntt = as1_ntt_montgomery.flat_map(&reduce::reduce32);
    let as1 = as1_ntt.invntt();
    // TODO: Check if we need to do a reduction here before caddq'ing.
    let t = as1.add(&s2).flat_map(&reduce::caddq);
    let (_, t0) = t.power2round();

    SigningKey {
        rho,
        key,
        s1,
        s2,
        t0,
    }
}
