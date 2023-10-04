use generic_array::typenum;
use generic_array::ArrayLength;

use crate::consts;
use crate::expand_s;
use crate::poly32;
use crate::ByteArray;

pub(crate) type N = typenum::U256;
pub(crate) type SEEDBYTES = typenum::U32;
pub(crate) type CRHBYTES = typenum::U48;

mod private {
    pub trait Sealed {}
}

pub trait Variant: private::Sealed {
    type L: ArrayLength;
    type K: ArrayLength;

    type SigningKeySize: ArrayLength;
}

pub(crate) trait VariantImpl: private::Sealed {
    fn polySampleEta(seed: &ByteArray<CRHBYTES>, nonce: u16) -> poly32::Poly32;
}
