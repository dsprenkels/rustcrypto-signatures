use generic_array::{typenum, ArrayLength};

use crate::{consts, expand_s, poly32, ByteArray};

pub(crate) const Q: i32 = 8380417;
pub(crate) const D: usize = 13;

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
    type SeedSize: ArrayLength;
}

pub struct Dilithium2;

impl private::Sealed for Dilithium2 {}

impl Variant for Dilithium2 {
    type L = typenum::U2;
    type K = typenum::U3;

    type SigningKeySize = consts::U2560;
    type SeedSize = typenum::U32;
}

/// This struct is instantiated once per variant to hold any functions that
/// differ between variants.  We use this method because (as far as I know)
/// there is currently no better way to specialize functions on concrete
/// instances of generic traits.
/// We can also not put these functions in the `Variant` trait, because the
/// `Variant` trait is exposed to the public.  Rust will complain about any
/// private types (like `Poly32`) that are used in the trait.
///
/// As a rule of thumb, all values that are sizes (i.e., `ArrayLengths` are)
/// to put in the `Variant` trait, and all other values (constants or functions)
/// are to be put in this struct.
#[derive(Clone, Copy)]
pub(crate) struct VariantImpl<V: Variant> {
    pub(crate) poly_sample_eta: fn(seed: &ByteArray<CRHBYTES>, nonce: u16) -> poly32::Poly32,
    // Bind V to VariantImpl to ensure that the variant is correct.
    _phantom: core::marker::PhantomData<V>,
}

pub(crate) const DILITHIUM2_IMPL: &VariantImpl<Dilithium2> = &VariantImpl {
    poly_sample_eta: expand_s::poly_uniform_eta_2,
    _phantom: core::marker::PhantomData,
};
