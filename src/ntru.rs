use crate::polyxnm1::{Integer, N, PolyXNm1, zp::*};
use crate::polyxnm1::service::mod_center;
use crate::polyxnm1::P;
use polynomial_ring::{Polynomial, polynomial};
use rand::Rng;
//=======================================================================================================================
#[allow(unused_variables)]
pub fn ntru_gen_keys (df: u8, dg: u8) -> (PolyXNm1<ModQ>, (Polynomial<Integer>, PolyXNm1<ModP>)) {
    let (f, fp, fq) = gen_f_fp_fq(df);
    
    let g = gen_polynomial(dg, dg);
    println!("g = {}", g.to_string());

    let g = PolyXNm1::from_polynomial(g);
    let h = fq * g;
    
    (h, (f, fp))
}
//=======================================================================================================================
#[allow(unused_variables)]
pub fn ntru_encrypt (dr: u8, h: &PolyXNm1<ModQ>, message: &PolyXNm1<ModQ>) -> PolyXNm1<ModQ> {
    let r = gen_polynomial(dr, dr);
    println!("r = {}", r.to_string());

    let r = PolyXNm1::<ModQ>::from_polynomial(r);
    
    let p = PolyXNm1::from_polynomial(polynomial![ModP::get() as Integer]);

    p * r * h + message
}
//=======================================================================================================================
#[cfg(not(feature = "time-measurement"))]
pub fn ntru_decrypt ((f, fp): (&Polynomial<Integer>, &PolyXNm1<ModP>), cipher: &PolyXNm1<ModQ>) -> Polynomial<Integer> {
    let a = PolyXNm1::<ModQ>::from_polynomial(f.clone()) * cipher;
    
    let q = ModQ::get();
    let a: Vec<Integer> = a.to_polynomial().coeffs().into_iter().map(|x| mod_center(x.get(), q)).collect();
    
    let m = PolyXNm1::from_polynomial(Polynomial::new(a)) * fp;
    
    let p = ModP::get();
    let m: Vec<Integer> = m.to_polynomial().coeffs().into_iter().map(|x| mod_center(x.get(), p)).collect();
    
    Polynomial::new(m)
}
//=======================================================================================================================
#[allow(dead_code, unused_variables)]
fn gen_f_fp_fq (df: u8) -> (Polynomial<Integer>, PolyXNm1<ModP>, PolyXNm1<ModQ>) {
    let mut f;
    let mut fp;
    let fq;
    loop {
        f = gen_polynomial(df, df - 1);
        println!("f = {}", f.to_string());

        let f_fp = PolyXNm1::<ModP>::from_polynomial(f.clone());
        fp = find_inv_polynomial::<ModP>(&f_fp);
        if fp == None {
            continue;
        }

        let f_fq = PolyXNm1::<ModQ>::from_polynomial(f.clone());
        let f2 = find_inv_polynomial(&f_fq.change_module::<Mod2>());

        if f2 == None {
            continue;
        }
        fq = f2_to_fq(&f_fq, &f2.unwrap());

        println!("fp = {}", fp.as_mut().unwrap().to_string());
        println!("fq = {}", fq.to_string());
        break;
    }
    (f, fp.unwrap(), fq)
}
//=======================================================================================================================
#[allow(dead_code)]
fn gen_polynomial (mut d_pos: u8, mut d_neg: u8) -> Polynomial<Integer> {
    let n = *N.get().unwrap() as usize;
    let mut polynomial = vec![0 as Integer; n];

    let mut i = 0usize;
    while d_pos != 0 {
        if i == n { i = 0; }
        polynomial[i] = rand::random_bool(0.5) as Integer;
        if polynomial[i] == 1 { d_pos -= 1; }
        i += 1;
    }

    i = 0usize;
    while d_neg != 0 {
        if i == n { i = 0; }
        if polynomial[i] == 0 {
            polynomial[i] = -(rand::random_bool(0.5) as Integer);
            if polynomial[i] == -1 { d_neg -= 1; }
        }
        i += 1;
    }
    Polynomial::new(polynomial)
}
//=======================================================================================================================
pub fn gen_m () -> Polynomial<Integer> {
    let n = *N.get().unwrap() as usize;
    let mut m = vec![0 as Integer; n];

    let mut rng = rand::rng();

    for i in 0..n {
        m[i] = mod_center(rng.random_range(0..10), *P.get().unwrap());
    }

    Polynomial::new(m)
}
//=======================================================================================================================
fn find_inv_polynomial <M: Module> (f: &PolyXNm1<M>) -> Option<PolyXNm1<M>> {
    // 1. initialization
    let mut r0 = PolyXNm1::get_xnm1();
    let mut r1 = f.clone();
    let mut t0 = PolyXNm1::from_polynomial(polynomial![0 as Integer]);
    let mut t1 = PolyXNm1::from_polynomial(polynomial![1 as Integer]);

    // 2. search GCD via r1
    while r1.coeffs_len() != 0 {
        let r2_div = &r0 / &r1;
        let r2_mod = r0 % &r1;
        let t2 = t0 - r2_div * &t1;

        r0 = r1;
        r1 = r2_mod;
        t0 = t1;
        t1 = t2;
    }

    if r0.coeffs_len() > 1 {
        None
    }
    else {
        let c = r0.get_little_coeff();
        match find_inv(&c.get(), &M::get()) {
            None => None,
            Some(inv_c) => Some(PolyXNm1::from_polynomial(polynomial![inv_c]) * &t0)
        }
        
    }
}
//=======================================================================================================================
fn f2_to_fq(f: &PolyXNm1<ModQ>, f2: &PolyXNm1<Mod2>) -> PolyXNm1<ModQ> {
    let mut fq = f2.change_module::<ModQ>();

    let q = ModQ::get();
    let two = PolyXNm1::<ModQ>::from_polynomial(polynomial![2 as Integer]);

    let mut i = 2;
    while i <= q {
        i *= i;
        fq = &fq * (&two - f * fq.change_module::<ModQ>());
    }

    fq
}
//=======================================================================================================================