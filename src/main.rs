mod polyxnm1;
mod ntru;
mod cryptoanalysis;

use polyxnm1::PolyXNm1;
use polyxnm1::UInteger;
use ntru::*;
use cryptoanalysis::svp_create_lattice_basis;
use cryptoanalysis::cvp_create_lattice_basis;
use cryptoanalysis::bkz::bkz;

#[cfg(not(feature = "time-measurement"))] use cryptoanalysis::search_potentional_secret_key;
#[cfg(not(feature = "time-measurement"))] use cryptoanalysis::search_potentional_plaintext;
#[cfg(not(feature = "time-measurement"))] use polyxnm1::{init_polynomial_ring, zp::*};

#[cfg(feature = "time-measurement")] use rust_xlsxwriter::XlsxError;
#[cfg(feature = "time-measurement")] use std::time::Duration;
#[cfg(feature = "time-measurement")] use polyxnm1::init_polynomial_ring;
#[cfg(feature = "time-measurement")] use polyxnm1::zp::ModQ;
#[cfg(feature = "time-measurement")] use rust_xlsxwriter::*;

use std::str::FromStr;
use std::fmt::Debug;
use std::time::Instant;
use std::io::{self, Write};
//=======================================================================================================================
#[cfg(not(feature = "time-measurement"))]
fn main() {
    print!("Input n: ");
    let n: UInteger = stdin_read();
    
    print!("Input p: ");
    let p: UInteger = stdin_read();

    print!("Input q: ");
    let q: UInteger = stdin_read();

    print!("Input df: ");
    let df: u8 = stdin_read();

    print!("Input dg: ");
    let dg: u8 = stdin_read();

    print!("Input dr: ");
    let dr: u8 = stdin_read();
    
    // let n: UInteger = 11;
    // let p: UInteger = 3;
    // let q: UInteger = 32;
    // let df = 4u8;
    // let dg = 3u8;
    // let dr = 3u8;

    init_polynomial_ring(n, p, q);

    let m = gen_m();
    println!("message m = {}", m.to_string());
    let m = PolyXNm1::<ModQ>::from_polynomial(m);

    println!("\nNTRUEncrypt: GEN_KEYS");
    let (h, (f, fp)) = ntru_gen_keys(df, dg);
    println!("h = {}", h.to_string());

    println!("\nNTRUEncrypt: ENCRYPT");
    let e = ntru_encrypt(dr, &h, &m);
    println!("e = {}", e.to_string());

    println!("\nNTRUEncrypt: DECRYPT");
    let m = ntru_decrypt((&f, &fp), &e);
    println!("m = {}", m.to_string());

    println!("\nNTRUEncrypt: LATTICE CRYPTOANALYSIS:\nSECRET KEY ATTACK:");
    let mut basis = svp_create_lattice_basis(&h);
    println!("basis:");
    for i in 0..basis.len() {
        for j in 0..basis[0].len() {
            print!("{} ", basis[i][j]);
        }
        println!();
    }
    
    println!("BKZ");
    let start = Instant::now();
    bkz(&mut basis, 2 * n as usize, 0.99);
    let duration = start.elapsed();

    for i in 0..basis.len() {
        print!("{}:\t", i);
        for j in 0..basis[0].len() {
            print!("{} ", basis[i][j]);
        }
        println!();
    }
    println!("Execution time: {:?}", duration);

    println!("\nSECRET KEY SEARCHING");
    search_potentional_secret_key(&basis, df);

    println!("PLAINTEXT ATTACK:");
    let mut basis = cvp_create_lattice_basis(&h, &e, 1);
    println!("basis:");
    for i in 0..basis.len() {
        for j in 0..basis[0].len() {
            print!("{} ", basis[i][j]);
        }
        println!();
    }
    println!("BKZ");
    let start = Instant::now();
    bkz(&mut basis, 2 * n as usize + 1, 0.99);
    let duration = start.elapsed();
    
    for i in 0..basis.len() {
        print!("{}:\t", i);
        for j in 0..basis[0].len() {
            print!("{} ", basis[i][j]);
        }
        println!();
    }
    println!("Execution time: {:?}", duration);

    println!("\nPLAINTEXT SEARCHING");
    search_potentional_plaintext(&basis, dr);
}
//=======================================================================================================================
#[cfg(feature = "time-measurement")]
fn main () -> Result<(), XlsxError> {
    const NUM_COL: u16 = 0;
    const SVP_COL: u16 = 1;
    const CVP_COL: u16 = 2;

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let bold_format = Format::new().set_bold();

    print!("Input count_ex: ");
    let count_ex: usize = stdin_read();

    print!("Input n: ");
    let n: UInteger = stdin_read();
    
    print!("Input p: ");
    let p: UInteger = stdin_read();

    print!("Input q: ");
    let q: UInteger = stdin_read();

    print!("Input df: ");
    let df: u8 = stdin_read();

    print!("Input dg: ");
    let dg: u8 = stdin_read();

    print!("Input dr: ");
    let dr: u8 = stdin_read();

    // let count_ex = 10usize;
    // let n: UInteger = 53;
    // let p: UInteger = 3;
    // let q: UInteger = 128;
    // let df = 4u8;
    // let dg = 3u8;
    // let dr = 4u8;

    worksheet.write_with_format(0, 0, format!("N = {}, P = {}, Q = {}, count ex = {}", n, p, q, count_ex), &bold_format)?;
    worksheet.write_with_format(1, NUM_COL, "№", &bold_format)?;
    worksheet.write_with_format(1, SVP_COL, "SVP", &bold_format)?;
    worksheet.write_with_format(1, CVP_COL, "CVP", &bold_format)?;

    init_polynomial_ring(n, p, q);

    let (h, (_, _)) = ntru_gen_keys(df, dg);
    
    let mut time_sum = Duration::new(0, 0);
    for i in 2..=(count_ex + 1) {
        worksheet.write(i as u32, NUM_COL, i as u32 - 1)?;
        
        let mut basis = svp_create_lattice_basis(&h);

        let start = Instant::now();
        bkz(&mut basis, 2 * n as usize, 0.99);
        let duration = start.elapsed();
        println!("SVP {}, time: {}", i - 1, duration.as_secs_f32());
        worksheet.write(i as u32, SVP_COL, duration.as_secs_f32())?;
        time_sum += duration;
    }

    worksheet.write(count_ex as u32 + 2, NUM_COL, "time_sum")?;
    worksheet.write(count_ex as u32 + 2, SVP_COL, time_sum.as_secs_f32())?;
    worksheet.write(count_ex as u32 + 3, NUM_COL, "average")?;
    worksheet.write(count_ex as u32 + 3, SVP_COL, time_sum.as_secs_f32() / count_ex as f32)?;

    println!("SVP ended ({:?})", time_sum);

    let m = gen_m();
    let m = PolyXNm1::<ModQ>::from_polynomial(m);
    let e = ntru_encrypt(dr, &h, &m);

    let mut time_sum = Duration::new(0, 0);
    for i in 2..=(count_ex + 1) {
        let mut basis = cvp_create_lattice_basis(&h, &e, 1);

        let start = Instant::now();
        bkz(&mut basis, 2 * n as usize + 1, 0.99);
        let duration = start.elapsed();
        println!("CVP {}, time: {}", i - 1, duration.as_secs_f32());

        worksheet.write(i as u32, CVP_COL, duration.as_secs_f32())?;
        time_sum += duration;
    }

    worksheet.write(count_ex as u32 + 2, CVP_COL, time_sum.as_secs_f32())?;
    worksheet.write(count_ex as u32 + 3, CVP_COL, time_sum.as_secs_f32() / count_ex as f32)?;

    println!("CVP ended ({:?})", time_sum);

    let mut path = format!("count_ex {} N {} p {} q {} ", count_ex, n, p, q);
    if cfg!(feature = "center-mod") {
        path += &format!("center-mod.xlsx")[..];
    }
    else {
        path += &format!("default.xlsx")[..];
    }

    workbook.save(&path)?;

    println!("File successfully saved in \"{}\"", path);

    Ok(())
}
//=======================================================================================================================
fn stdin_read<T: FromStr> () -> T where <T as FromStr>::Err:Debug {
    io::stdout().flush().unwrap();
    let mut str = String::new();
    io::stdin().read_line(&mut str).unwrap();
    let input_var: T = str.trim().parse().unwrap();
    input_var
}
//=======================================================================================================================