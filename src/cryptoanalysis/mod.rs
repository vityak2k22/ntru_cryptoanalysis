pub mod lll;
pub mod bkz;

use std::vec;

#[cfg(not(feature = "time-measurement"))] use crate::cryptoanalysis::lll::squared_norm;
use crate::polyxnm1::zp::{ModQ, Zp};
use crate::polyxnm1::{Integer, N, P, Q};
use crate::PolyXNm1;
//=======================================================================================================================
pub fn svp_create_lattice_basis (h_poly: &PolyXNm1<ModQ>) -> Vec<Vec<Integer>> {
    //let q = *Q.get().unwrap();
    let mut h: Vec<Integer> = h_poly.clone().to_polynomial().coeffs().into_iter().map(|x| x.get()).collect();
    let basis = create_lattice_basis(&mut h);

    basis
}
//=======================================================================================================================
#[cfg(not(feature = "time-measurement"))]
pub fn search_potentional_secret_key (basis: &Vec<Vec<Integer>>, df: u8) {
    let n = *N.get().unwrap() as usize;
    for i in 0..basis.len() {
        let f_candidate = &basis[i][0..n];

        // check 1: {-1 0 1} coeffs
        let is_mod_p = f_candidate.into_iter().all(|&x| x == 0 || x == 1 || x == -1);
        if is_mod_p == false {
            continue;
        }

        // check 2: df + df - 1 == norm(f)^2
        let norm_sq: Integer = squared_norm(&f_candidate.to_vec());
        if norm_sq == (df + df - 1) as Integer {
            println!("Potentional f found: {:?} (vec_index = {})", f_candidate, i);
            println!("Potentional g found: {:?}", basis[i][n..2 * n].to_vec());
            break;
        }
    }
}
//=======================================================================================================================
pub fn cvp_create_lattice_basis (h_poly: &PolyXNm1<ModQ>, e_poly: &PolyXNm1<ModQ>, m_coeff: Integer) -> Vec<Vec<Integer>> {
    let p = *P.get().unwrap() as Integer;

    let mut h: Vec<Integer> = h_poly.clone().to_polynomial().coeffs().into_iter().map(|&x| (Zp::<ModQ>::new(p) * x).get()).collect();
    let mut basis = create_lattice_basis(&mut h);

    for vec in &mut basis {
        vec.push(0);
    }

    let n = *N.get().unwrap() as usize;
    basis.push(vec![0 as Integer; 2 * n + 1]);

    let e = e_poly.clone().to_polynomial();
    for i in 0..e.coeffs().len() {
        basis[2 * n][n + i] = e.coeffs()[i].get();
    }
    basis[2 * n][2 * n] = m_coeff;


    basis
}
//=======================================================================================================================
#[cfg(not(feature = "time-measurement"))]
pub fn search_potentional_plaintext (basis: &Vec<Vec<Integer>>, dr: u8) {
    let n = *N.get().unwrap() as usize;

    for i in 0..basis.len() {
        let r_candidate = &basis[i][0..n];

        // check 1: {-1 0 1} coeffs
        let is_mod_p = r_candidate.into_iter().all(|&x| x == 0 || x == 1 || x == -1);
        if is_mod_p == false {
            continue;
        }

        // check 2: dr + dr == norm(r)^2
        let norm_sq: Integer = squared_norm(&r_candidate.to_vec());
        if norm_sq == (dr + dr) as Integer {
            println!("Potentional (r, m) found: {:?} (vec_index = {}). Except last element!", basis[i], i);
            break;
        }

    }
}
//=======================================================================================================================
#[inline(always)]
fn create_lattice_basis (h: &mut Vec<Integer>) -> Vec<Vec<Integer>> {
    let n = *N.get().unwrap() as usize;
    let q = *Q.get().unwrap();

    let mut basis = vec![vec![0 as Integer; 2 * n]; 2 * n];
    
    // unit matrix
    for i in 0..n {
        basis[i][i] = 1;
    }

    // h matrix
    if h.len() < n { h.append(&mut vec![0 as Integer; n - h.len()]); }

    for i in 0..n {
        for j in 0..n {
            basis[i][j+n] = h[j];
        }
        for x in (1..n).rev() {
            let temp = h[x];
            h[x] = h[x-1];
            h[x-1] = temp;
        }
    }
    
    // q matrix
    for i in n..(2 * n) {
        basis[i][i] = q as Integer;
    }

    basis
}
//=======================================================================================================================