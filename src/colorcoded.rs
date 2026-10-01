use crate::filesearch::read;
use colored::Colorize;
use rayon::prelude::*;
use std::error::Error;

/*
Gaurav Sablok,
gsablok@proton.me
*/

pub fn readcodecolor(path: &str) -> Result<String, Box<dyn Error>> {
    let (header, sequence) = read(path).unwrap();

    // Previously each sequence was scanned four separate times (once per
    // base) with an intermediate Vec allocated each time. Now it's a single
    // pass per sequence, and each sequence is counted independently in
    // parallel via rayon.
    let vecsize: Vec<(String, usize, usize, usize, usize)> = header
        .par_iter()
        .zip(sequence.par_iter())
        .map(|(h, s)| {
            let (mut a, mut t, mut g, mut c) = (0usize, 0usize, 0usize, 0usize);
            for ch in s.chars() {
                match ch {
                    'A' => a += 1,
                    'T' => t += 1,
                    'G' => g += 1,
                    'C' => c += 1,
                    _ => {}
                }
            }
            (h.clone(), a, t, g, c)
        })
        .collect();

    for i in vecsize.iter() {
        if !i.0.is_empty() {
            print!(
                "{}\t{}-{}-{}-{}",
                i.0,
                i.1.to_string().on_yellow().bold(),
                i.2.to_string().on_blue().bold(),
                i.3.to_string().on_red().bold(),
                i.4.to_string().on_magenta().bold()
            );
        } else {
            continue;
        }
        println!();
    }

    Ok("The colour coded results are as follows".to_string())
}
