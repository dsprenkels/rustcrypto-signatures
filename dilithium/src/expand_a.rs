use generic_array::typenum::Unsigned;
use sha3::digest::{ExtendableOutput, Update, XofReader};

use crate::{poly32, variant, ByteArray};

/// This funcion corresponds to `poly_uniform`.
pub(crate) fn expand_a_poly(seed: &ByteArray<variant::SEEDBYTES>, nonce: u16) -> poly32::Poly32 {
    let mut xof = sha3::Shake128::default();
    xof.update(seed);
    xof.update(&nonce.to_le_bytes());
    let mut xofread = xof.finalize_xof();

    let mut poly = poly32::Poly32::default();
    'coeff: for coeff in poly.coeffs.iter_mut() {
        loop {
            let mut sample = [0; 4];
            xofread.read(&mut sample[0..3]);
            let mut t = i32::from_le_bytes(sample);
            t &= 0x7FFFFF;
            if t < variant::Q {
                // TODO: Consider pulling coefficients larger than Q/2 to the negative domain.
                *coeff = t;
                continue 'coeff;
            }
        }
    }
    poly
}

pub(crate) fn expand_a<V: variant::Variant>(
    seed: &ByteArray<variant::SEEDBYTES>,
) -> crate::Matrix<V> {
    let mut mat = crate::Matrix::<V>::default();
    for row in 0..V::K::USIZE {
        for col in 0..V::L::USIZE {
            let nonce = ((row as u16) << 8) | col as u16;
            mat[row][col] = expand_a_poly(seed, nonce);
        }
    }
    mat
}
