use core::ops::{Add, Mul};

use generic_array::typenum::*;

type U2000 = <U2 as Mul<U1000>>::Output;
type U4000 = <U4 as Mul<U1000>>::Output;

pub(crate) type U2560 = <U2000 as Add<U560>>::Output;
pub(crate) type U4032 = <U4000 as Add<U32>>::Output;
pub(crate) type U4896 = <U4000 as Add<U896>>::Output;
