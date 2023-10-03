use core::ops::{Add, Mul};

use generic_array::typenum::*;

pub type U2000 = <U2 as Mul<U1000>>::Output;
pub type U2560 = <U2000 as Add<U560>>::Output;
