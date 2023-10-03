use generic_array::typenum;
use generic_array::ArrayLength;

use crate::consts;

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