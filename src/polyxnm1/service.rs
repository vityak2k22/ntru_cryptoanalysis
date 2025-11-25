use crate::polyxnm1::UInteger;
use crate::polyxnm1::Integer;

//=======================================================================================================================
pub fn gcd(mut p: UInteger, mut q: UInteger) -> UInteger {
    let mut mod_center = p % q;
    while mod_center != 0 {
        p = q;
        q = mod_center;
        mod_center = p % q;
    }
    q
}
//=======================================================================================================================
pub fn mod_center (a: Integer, p: UInteger) -> Integer {
    let result = a % p as Integer;
    if result > (p as Integer - 1) / 2 {
        result - p as Integer
    }
    else if result < -(p as Integer - 1) / 2 {
        result + p as Integer
    }
    else {
        result
    }
}
//=======================================================================================================================