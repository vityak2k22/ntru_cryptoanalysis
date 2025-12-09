use crate::cryptoanalysis::lll::*;
use crate::polyxnm1::Integer;
//=======================================================================================================================
pub fn bkz (b: &mut Vec<Vec<Integer>>, beta: usize, delta: Float) {
    let (mut c, mut gs_coeff) = lll(b, delta, false);

    let mut z = 0usize;
    let mut j = 0usize;
    while z < b.len() - 1 {
        let m = b.len();
        let k = (j + beta).min(m - 1);
        
        let (_, c_j, bj_new) = bkz_enum(b, &c, &gs_coeff, j, k);
        
        let h = (k + 1).min(m - 1);
        if delta * c[j] > c_j {
            let mut new_b = b[0..j].to_vec();
            new_b.push(bj_new);
            new_b.extend_from_slice(&b[j..=h]);
            
            *b = new_b;

            (c, gs_coeff) = lll(b, delta, true);
            z = 0;
        }
        else {
            z += 1;
            (c, gs_coeff) = lll(b, 0.99, false);
        }

        j += 1;
        if j == m - 1 {
            j = 0;
        }
    }
}
//=======================================================================================================================
fn bkz_enum (b: &Vec<Vec<Integer>>, c: &Vec<Float>, gs_coeff: &Vec<Vec<Float>>, j: usize, k: usize) -> (Vec<Integer>, Float, Vec<Integer>) {
    // 1. initialization
    let vec_size = k + 2;
    let mut c_j = c[j];
    let mut u = vec![0 as Integer; vec_size];
    let mut u_= vec![0 as Integer; vec_size];
    let mut big_delta = vec![0i8; vec_size];
    let mut y = vec![0 as Float; vec_size];
    let mut delta = vec![1i8; vec_size];
    let mut c_ = vec![0 as Float; vec_size];
    let mut v = vec![0 as Integer; vec_size];

    u[j] = 1;
    u_[j] = 1;

    // 2. loop
    let mut t = j;
    let mut s = t;
    while t <= k {
        c_[t] = c_[t+1] + (y[t] + u_[t] as Float) * (y[t] + u_[t] as Float) * c[t];
        if c_[t] < c_j {
            if t > j {
                t -= 1;

                let mut sum = 0 as Float;
                for i in (t+1)..=s {
                    sum += u_[i] as Float * gs_coeff[i][t];
                }
                y[t] = sum;
                v[t] = (-y[t]).round() as Integer;
                u_[t] = v[t];
                big_delta[t] = 0;

                delta[t] = if u_[t] as Float > -y[t] { -1 } else { 1 };
            }
            else {
                c_j = c_[j];
                for i in j..=k {
                    u[i] = u_[i];
                }
            }
        }
        else {
            t += 1;
            s = s.max(t);
            if t < s {
                big_delta[t] = -big_delta[t];
            }
            if big_delta[t] * delta[t] >= 0 {
                big_delta[t] += delta[t];
            }
            u_[t] = v[t] + big_delta[t] as Integer;
        }
    }
    let bj_new = sum_ui_bi(&u, b, j, k);
    (u, c_j, bj_new)
}
//=======================================================================================================================
fn sum_ui_bi (u: &Vec<Integer>, b: &Vec<Vec<Integer>>, j: usize, k: usize) -> Vec<Integer> {
    let mut result = vec![0 as Integer; b[0].len()];
    for i in j..=k {
        for x in 0..result.len() {
            result[x] += u[i] * b[i][x];
        }
    }
    result
}
//=======================================================================================================================