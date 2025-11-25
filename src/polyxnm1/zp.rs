use num_traits::One;
use num_traits::Zero;

use crate::polyxnm1::Integer;
use crate::polyxnm1::UInteger;
use crate::polyxnm1::P;
use crate::polyxnm1::Q;
#[cfg(feature = "center-mod")] use crate::polyxnm1::service::mod_center;

use std::marker::PhantomData;
use core::fmt;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

//=======================================================================================================================
pub struct Zp<M: GetModule> {
    value: Integer,
    _marker: PhantomData<M>
}
//=======================================================================================================================
impl <M: GetModule> Zp<M> {
    #[cfg(feature = "center-mod")]
    pub fn new (value: Integer) -> Zp<M>{
        let p = M::get();
        Zp {
            value: mod_center(value, p),
            _marker: PhantomData
        }
    }
    
    #[cfg(not(feature = "center-mod"))]
    pub fn new (value: Integer) -> Zp<M>{
        let p = M::get() as Integer;
        if value >= p || value < 0 {
            Zp {
                value: (value % p + p) % p,
                _marker: PhantomData
            }
        }
        else {
            Zp {
                 value,
                 _marker: PhantomData
            } 
        }
    }
//=======================================================================================================================
    pub fn get (&self) -> Integer {
        self.value
    }
}
//=======================================================================================================================
impl <M: GetModule> fmt::Display for Zp<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
//=======================================================================================================================
impl <M: GetModule> Clone for Zp<M> {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            _marker: PhantomData
        }
    }
}

impl <M: GetModule> Copy for Zp<M> { }
//=======================================================================================================================
impl <M: GetModule> Add for Zp<M> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Zp::new(self.value + rhs.value)
    }
}

impl <M: Module> AddAssign for Zp<M> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<'a, M: Module> AddAssign<&'a Zp<M>> for Zp<M> {
    fn add_assign(&mut self, rhs: &'a Zp<M>) {
        *self = *self + *rhs;
    }
}
//=======================================================================================================================
impl <M: GetModule> Sub for Zp<M> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Zp::new(self.value - rhs.value)
    }
}

impl <M: GetModule> SubAssign for Zp<M> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<'a, M: Module> SubAssign<&'a Zp<M>> for Zp<M> {
    fn sub_assign(&mut self, rhs: &'a Zp<M>) {
        *self = *self - *rhs;
    }
}
//=======================================================================================================================
impl <M: GetModule> Mul for Zp<M> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Zp::new(self.value * rhs.value)
    }
}

impl <M: GetModule> MulAssign for Zp<M> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<'a, M: Module> MulAssign<&'a Zp<M>> for Zp<M> {
    fn mul_assign(&mut self, rhs: &'a Zp<M>) {
        *self = *self * *rhs;
    }
}
//=======================================================================================================================
impl <M: GetModule> Div for Zp<M> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert_ne!(rhs.value, 0, "Zp div: Division by zero");
        let inv = find_inv(&rhs.value, &M::get()).unwrap();
        Zp::new(self.value * inv)
    }
}

impl <M: GetModule> DivAssign for Zp<M> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<'a, M: Module> DivAssign<&'a Zp<M>> for Zp<M> {
    fn div_assign(&mut self, rhs: &'a Zp<M>) {
        *self = *self / *rhs;
    }
}
//=======================================================================================================================
impl<M: GetModule> PartialEq for Zp<M> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }

    fn ne(&self, other: &Self) -> bool {
        self.value != other.value
    }
}
//=======================================================================================================================
impl <M: GetModule> Zero for Zp<M> {
    fn zero() -> Self {
        Zp { value: 0, _marker: PhantomData }
    }

    fn is_zero(&self) -> bool {
        self.value == 0
    }
}

impl <M: GetModule> One for Zp<M> {
    fn one() -> Self {
        Zp { value: 1, _marker: PhantomData }
    }
}
//=======================================================================================================================
pub fn find_inv (a: &Integer, p: &UInteger) -> Option<Integer> {
    let mut r0 = *p as Integer;
    let mut r1 = *a;
    let mut t0 = 0 as Integer;
    let mut t1 = 1 as Integer;
    while r1 != 0 {
        let r2_div = r0 / r1;
        let r2_rem = r0 % r1;
        let t2 = t0 - r2_div * t1;

        r0 = r1;
        r1 = r2_rem;
        t0 = t1;
        t1 = t2;
    }
    if r0 == 1 || r0 == -1 { Some(r0 * t0) }
    else { None }
}
//=======================================================================================================================
pub trait GetModule {
    fn get () -> UInteger;
}

#[derive(Clone, Copy)]
pub struct ModP;

#[derive(Clone, Copy)]
pub struct ModQ;

#[derive(Clone, Copy)]
pub struct Mod2;

impl GetModule for ModP {
    fn get () -> UInteger {
        *P.get().unwrap()
    }
}

impl GetModule for ModQ {
    fn get () -> UInteger {
        *Q.get().unwrap()
    }
}

impl GetModule for Mod2 {
    fn get () -> UInteger {
        2
    }
}
//=======================================================================================================================
pub trait Module: GetModule + Copy {}
impl <T: GetModule + Copy> Module for T {}
//=======================================================================================================================