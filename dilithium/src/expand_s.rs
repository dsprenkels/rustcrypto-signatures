use sha3::digest::{ExtendableOutput, Update, XofReader};

use crate::{poly32, variant, ByteArray};

struct Sampler4Bit {
    xofread: sha3::Shake256Reader,
    sample: [u8; 1],
    offset: usize,
}

impl Iterator for Sampler4Bit {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Buffer in larger chunks than a single byte.
        if self.offset == 0 {
            self.xofread.read(&mut self.sample);
            self.offset = 1;
            Some(i32::from(self.sample[0] & 0x0F))
        } else {
            self.offset = 0;
            Some(i32::from(self.sample[0] >> 4))
        }
    }
}

impl Sampler4Bit {
    fn new(seed: &ByteArray<variant::CRHBYTES>, nonce: u16) -> Self {
        let mut xof = sha3::Shake256::default();
        xof.update(seed);
        xof.update(&nonce.to_le_bytes());
        let xofread = xof.finalize_xof();
        Self {
            xofread,
            sample: [0; 1],
            offset: 0,
        }
    }
}

/// Sample a polynomial with coefficients in uniformly distributed in `[-2, 2]`.
///
/// This function corresponds to `poly_uniform_eta`, specialized for when
/// eta is 2.
pub(crate) fn poly_uniform_eta_2(
    seed: &ByteArray<variant::CRHBYTES>,
    nonce: u16,
) -> poly32::Poly32 {
    let mut poly = poly32::Poly32::default();
    let mut iter = poly.coeffs.iter_mut();
    let mut coeff = iter.next().expect("poly has no coefficients");
    for t0 in Sampler4Bit::new(seed, nonce) {
        if t0 < 15 {
            let t0 = t0.wrapping_sub((205i32.wrapping_mul(t0) >> 10).wrapping_mul(5));
            *coeff = 2i32.wrapping_sub(t0);
            coeff = match iter.next() {
                Some(x) => x,
                None => break,
            };
        }
    }
    poly
}

/// Sample a polynomial with coefficients in uniformly distributed in `[-4, 4]`.
///
/// This function corresponds to `poly_uniform_eta`, specialized for when
/// eta is 4.
pub(crate) fn poly_uniform_eta_4(
    seed: &ByteArray<variant::CRHBYTES>,
    nonce: u16,
) -> poly32::Poly32 {
    let mut poly = poly32::Poly32::default();
    let mut iter = poly.coeffs.iter_mut();
    let mut coeff = iter.next().expect("poly has no coefficients");
    for t0 in Sampler4Bit::new(seed, nonce) {
        *coeff = 4i32.wrapping_sub(t0);
        coeff = match iter.next() {
            Some(x) => x,
            None => break,
        };
    }
    poly
}

pub(crate) fn expand_s<V: variant::Variant, N: generic_array::ArrayLength>(
    vi: &variant::VariantImpl<V>,
    seed: &ByteArray<variant::CRHBYTES>,
    mut nonce: u16,
) -> (generic_array::GenericArray<poly32::Poly32, N>, u16) {
    let mut vec = generic_array::GenericArray::default();
    for poly in vec.iter_mut() {
        *poly = (vi.poly_sample_eta)(seed, nonce);
        nonce += 1;
    }
    (vec, nonce)
}
