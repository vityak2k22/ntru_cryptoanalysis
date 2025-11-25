pub mod service;
pub mod zp;

use num_traits::{One, Zero};
use service::*;
use zp::*;
use polynomial_ring::{Polynomial};
use std::{ops::{Add, Div, Mul, Rem, Sub}, sync::OnceLock};

pub type Integer = i32;
pub type UInteger = u32;

pub static N: OnceLock<UInteger> = OnceLock::new();
pub static P: OnceLock<UInteger> = OnceLock::new();
pub static Q: OnceLock<UInteger> = OnceLock::new();

//=======================================================================================================================
pub fn init_polynomial_ring (n: UInteger, p: UInteger, q: UInteger) {
    assert_eq!(gcd(p, q), 1, "GCD(p = {p}, q = {q}) is not equal to 1");
    N.set(n).unwrap();
    P.set(p).unwrap();
    Q.set(q).unwrap();
}
//=======================================================================================================================
pub struct PolyXNm1<M: GetModule> {
    coeffs: Polynomial<Zp<M>>
}
//=======================================================================================================================
impl <M: GetModule> PolyXNm1 <M> {
    pub fn from_polynomial (polynomial: Polynomial<Integer>) -> PolyXNm1<M> {
        assert_ne!(N.get(), None, "N parameter is not initialized. First initialize a polynomial ring via init_polynomial_ring() function");
        assert_ne!(P.get(), None, "P parameter is not initialized. First initialize a polynomial ring via init_polynomial_ring() function");
        assert_ne!(Q.get(), None, "Q parameter is not initialized. First initialize a polynomial ring via init_polynomial_ring() function");
        
        let polynomial_vec_coeffs: Vec<Zp<M>> = polynomial.coeffs().into_iter().map(|x| Zp::new(*x)).collect();
        
        PolyXNm1 { coeffs: Polynomial::new(polynomial_vec_coeffs) }
    }
//=======================================================================================================================
    #[inline(always)]
    pub fn to_polynomial (self) -> Polynomial<Zp<M>> {
        self.coeffs
    }
//=======================================================================================================================
    #[inline(always)]
    pub fn coeffs_len (&self) -> usize {
        self.coeffs.coeffs().len()
    }
//=======================================================================================================================
    #[inline(always)]
    pub fn get_little_coeff (&self) -> zp::Zp<M> {
        self.coeffs.coeffs()[0]
    }
//=======================================================================================================================
    #[inline(always)]
    pub fn get_xnm1 () -> PolyXNm1<M> {
        let n = *N.get().unwrap() as usize;
        let mut xnm1 = vec![Zp::new(-1)];
        xnm1.append(&mut vec![Zp::zero(); n - 1]);
        xnm1.push(Zp::one());

        PolyXNm1 { coeffs: Polynomial::new(xnm1) }
    }
//=======================================================================================================================
    pub fn change_module <NewM: Module> (&self) -> PolyXNm1<NewM> {
        let result: Vec<Zp<NewM>> = self.coeffs.coeffs().into_iter().map(|x| Zp::<NewM>::new(x.get())).collect();
        PolyXNm1 { coeffs: Polynomial::new(result) }
    }
//=======================================================================================================================
    #[allow(dead_code)]
    pub fn to_string (&self) -> String {
        self.coeffs.to_string()
    }
}
//=======================================================================================================================
impl <'a, 'b, M: Module> Add<&'b PolyXNm1<M>> for &'a PolyXNm1<M> {
    type Output = PolyXNm1<M>;

    fn add(self, rhs: &'b PolyXNm1<M>) -> Self::Output {
        PolyXNm1 {
            coeffs: &self.coeffs + &rhs.coeffs,
        }
    }
}

impl <M: Module> Add<PolyXNm1<M>> for PolyXNm1<M> {
    type Output = PolyXNm1<M>;

    fn add(self, rhs: PolyXNm1<M>) -> Self::Output {
        &self + &rhs
    }
}

impl <'a, M: Module> Add<&'a PolyXNm1<M>> for PolyXNm1<M> {
    type Output = PolyXNm1<M>;

    fn add(self, rhs: &'a PolyXNm1<M>) -> Self::Output {
        &self + rhs
    }
}

impl <'a, M: Module> Add<PolyXNm1<M>> for &'a PolyXNm1<M> {
    type Output = PolyXNm1<M>;

    fn add(self, rhs: PolyXNm1<M>) -> Self::Output {
        self + &rhs
    }
}
//=======================================================================================================================
impl <'a, 'b, M: Module> Sub<&'b PolyXNm1<M>> for &'a PolyXNm1<M> {
    type Output = PolyXNm1<M>;

    fn sub(self, rhs: &'b PolyXNm1<M>) -> Self::Output {
        PolyXNm1 {
            coeffs: &self.coeffs - &rhs.coeffs,
        }
    }
}

impl <'a, M: Module> Sub <&'a PolyXNm1<M>> for PolyXNm1<M> {
    type Output = PolyXNm1<M>;

    fn sub(self, rhs: &'a PolyXNm1<M>) -> Self::Output {
        &self - rhs
    }
}

impl <'a, M: Module> Sub <PolyXNm1<M>> for &'a PolyXNm1<M> {
    type Output = PolyXNm1<M>;

    fn sub(self, rhs: PolyXNm1<M>) -> Self::Output {
        self - &rhs
    }
}

impl <M: Module> Sub <PolyXNm1<M>> for PolyXNm1<M> {
    type Output = PolyXNm1<M>;

    fn sub(self, rhs: PolyXNm1<M>) -> Self::Output {
        &self - &rhs
    }
}
//=======================================================================================================================
impl <'a, 'b, M: Module> Mul<&'b PolyXNm1<M>> for &'a PolyXNm1<M> {
    type Output = PolyXNm1<M>;

    fn mul(self, rhs: &'b PolyXNm1<M>) -> Self::Output {
        let mut result  = &self.coeffs * &rhs.coeffs;

        let n = *N.get().unwrap() as usize;
        let result_len = result.coeffs().len();
        if result_len > n {
            let bigger_coeffs = Polynomial::new(result.coeffs()[n..result_len].to_vec());
            result += &bigger_coeffs;
            let mut substract_bigger_coeffs = vec![zp::Zp::zero(); n];
            substract_bigger_coeffs.append(&mut bigger_coeffs.coeffs().to_vec());
            result -= Polynomial::new(substract_bigger_coeffs);
        }

        PolyXNm1 {
            coeffs: result,
        }
    }
}

impl <M: Module> Mul<PolyXNm1<M>> for PolyXNm1<M> {
    type Output = PolyXNm1<M>;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl <'a, M: Module> Mul<PolyXNm1<M>> for &'a PolyXNm1<M> {
    type Output = PolyXNm1<M>;

    fn mul(self, rhs: PolyXNm1<M>) -> Self::Output {
        self * &rhs
    }
}

impl <'a, M: Module> Mul<&'a PolyXNm1<M>> for PolyXNm1<M> {
    type Output = PolyXNm1<M>;

    fn mul(self, rhs: &'a PolyXNm1<M>) -> Self::Output {
        &self * rhs
    }
}
//=======================================================================================================================
impl <'a, 'b, M: Module> Div<&'b PolyXNm1<M>> for &'a PolyXNm1<M> {
    type Output = PolyXNm1<M>;

    fn div(self, rhs: &'b PolyXNm1<M>) -> Self::Output {
        PolyXNm1 {
            coeffs: &self.coeffs / &rhs.coeffs,
        }
    }
}

impl <'a, M: Module> Div<&'a PolyXNm1<M>> for PolyXNm1<M> {
    type Output = PolyXNm1<M>;

    fn div(self, rhs: &'a PolyXNm1<M>) -> Self::Output {
        &self / rhs
    }
}

impl <'a, M: Module> Div<PolyXNm1<M>> for &'a PolyXNm1<M> {
    type Output = PolyXNm1<M>;

    fn div(self, rhs: PolyXNm1<M>) -> Self::Output {
        self / &rhs
    }
}

impl <M: Module> Div for PolyXNm1<M> {
    type Output = PolyXNm1<M>;

    fn div(self, rhs: PolyXNm1<M>) -> Self::Output {
        &self / &rhs
    }
}
//=======================================================================================================================
impl <'a, 'b, M: Module> Rem<&'b PolyXNm1<M>> for &'a PolyXNm1<M> {
    type Output = PolyXNm1<M>;

    fn rem(self, rhs: &'b PolyXNm1<M>) -> Self::Output {
        PolyXNm1 {
            coeffs: &self.coeffs - &self.coeffs / &rhs.coeffs * &rhs.coeffs,
        }
    }
}

impl <'a, M: Module> Rem<&'a PolyXNm1<M>> for PolyXNm1<M> {
    type Output = PolyXNm1<M>;

    fn rem(self, rhs: &'a PolyXNm1<M>) -> Self::Output {
        &self % rhs
    }
}

impl <'a, M: Module> Rem<PolyXNm1<M>> for &'a PolyXNm1<M> {
    type Output = PolyXNm1<M>;

    fn rem(self, rhs: PolyXNm1<M>) -> Self::Output {
        self % &rhs
    }
}

impl <M: Module> Rem<PolyXNm1<M>> for PolyXNm1<M> {
    type Output = PolyXNm1<M>;

    fn rem(self, rhs: PolyXNm1<M>) -> Self::Output {
        &self % &rhs
    }
}
//=======================================================================================================================
impl <M: GetModule> PartialEq for PolyXNm1<M> {
    fn eq(&self, other: &Self) -> bool {
        self.coeffs == other.coeffs
    }

    fn ne(&self, other: &Self) -> bool {
        self.coeffs != other.coeffs
    }
}
//=======================================================================================================================
impl <M: Module> Clone for PolyXNm1<M> {
    fn clone(&self) -> Self {
        Self { coeffs: self.coeffs.clone() }
    }
}
//=======================================================================================================================