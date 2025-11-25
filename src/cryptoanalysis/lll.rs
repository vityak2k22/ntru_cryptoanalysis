use crate::polyxnm1::Integer;
use num_traits::Zero;

pub type Float = f64;

pub fn lll (b: &mut Vec<Vec<Integer>>, delta: Float, mut fc: bool) -> (Vec<Float>, Vec<Vec<Float>>) {
    let n = b[0].len();

    let mut gs_coeff = vec![vec![0 as Float; n]; b.len()];
    let mut c = vec![0 as Float; b.len()];

    // 1. initialization
    let mut k = 1;
    let mut b_ = to_float(&b);

    while k < b.len() {
        // 2. GS-orthogonalization
        c[k] = squared_norm(&b_[k]);
        if k == 1 {
            c[0] = squared_norm(&b_[0]);
        }

        for j in 0..k {
            let s = if scalar_product(&b_[k], &b_[j]).abs() < (2 as Float).powf(-(Float::MANTISSA_DIGITS as Float / 2.)) * squared_norm(&b_[k]).sqrt() * squared_norm(&b_[j]).sqrt() {
                scalar_product(&b[k], &b[j]) as Float
            }
            else {
                scalar_product(&b_[k], &b_[j])
            };
            
            let mut sum = 0.;
            for i in 0..j {
                sum += gs_coeff[j][i] * gs_coeff[k][i] * c[i];
            }
            gs_coeff[k][j] = (s - sum) / c[j];
            c[k] -= gs_coeff[k][j] * gs_coeff[k][j] * c[j]; 
        }

        // 3. size-reduction of b
        for j in (0..k).rev() {
            if gs_coeff[k][j].abs() > 0.5 {
                let nu = gs_coeff[k][j].round() as Integer;
                if nu.abs() > 2i32.pow(Float::MANTISSA_DIGITS / 2) as Integer { fc = true; }
                for i in 0..j {
                    gs_coeff[k][i] -= nu as Float * gs_coeff[j][i]; 
                }
                gs_coeff[k][j] -= nu as Float;
                
                sub_vec(b, k, j, nu);
                assign_vec_to_float(&mut b_[k], &b[k]);
            }
        }
        if fc == true {
            fc = false;
            k = (k - 1).max(1);
            continue;
        }
        // additional assignment for step 3: zero-vectors removing
        if is_zero(&b[k]) == true {
            b.remove(k);
            b_.remove(k);
            c.remove(k);
            gs_coeff.remove(k);
            k = 1;
            continue;
        }
        
        //4. swap or increment
        if delta * c[k-1] > c[k] + gs_coeff[k][k-1] * gs_coeff[k][k-1] * c[k-1] {
            b.swap(k, k - 1);
            b_.swap(k, k - 1);
            k = (k - 1).max(1);
        }
        else {
            k += 1;
        }
    }
    (c, gs_coeff)
}

fn to_float(m: &Vec<Vec<Integer>>) -> Vec<Vec<Float>> {
    let mut result = vec![vec![0 as Float; m[0].len()]; m.len()];
    for i in 0..result.len() {
        for j in 0..result[0].len() {
            result[i][j] = m[i][j] as Float;
        }
    }
    result
}

#[inline(always)]
pub fn squared_norm<T: std::ops::Mul<Output = T> + std::ops::AddAssign + Zero + Copy>(v: &Vec<T>) -> T {
    scalar_product::<T>(v, v)
}


fn scalar_product<T: std::ops::Mul<Output = T> + std::ops::AddAssign + Zero + Copy>(a: &Vec<T>, b: &Vec<T>) -> T {
    let mut result: T = T::zero();
    for i in 0..a.len() {
        result += a[i] * b[i];
    }
    result
}

fn sub_vec(m: &mut Vec<Vec<Integer>>, index_1: usize, index_2: usize, b_mul: Integer) {
    for i in 0..m[0].len() {
        m[index_1][i] -= b_mul * m[index_2][i];
    }
}

fn assign_vec_to_float (a: &mut Vec<Float>, b: &Vec<Integer>) {
    for i in 0..a.len() {
        a[i] = b[i] as Float;
    }
}

#[allow(dead_code)]
fn assign_vec (v: &mut Vec<Vec<Integer>>, index_1: usize, index_2: usize) {
    for i in 0..v[0].len() {
        v[index_1][i] = v[index_2][i];
    }
}

fn is_zero(v: &Vec<Integer>) -> bool {
    for i in 0..v.len() {
        if v[i] != 0 {
            return false;
        }
    }
    true
}