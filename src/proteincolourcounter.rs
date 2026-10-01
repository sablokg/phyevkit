use crate::astruct::count_amino_acids;
use colored::Colorize;
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok,
gsablok@proton.me
*/

pub fn proteomecolor(path: &str) -> Result<String, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut header: Vec<String> = Vec::new();
    let mut sequence: Vec<String> = Vec::new();

    for i in fileread.lines() {
        let line = i.expect("line not found");
        if line.starts_with(">") {
            header.push(line.replace(">", ""));
        } else if !line.starts_with(">") {
            sequence.push(line);
        }
    }

    // Same independent per-record counting as proteinstat.rs, run in
    // parallel across the rayon pool.
    let finalcount: Vec<(String, Vec<(String, usize)>)> = sequence
        .par_iter()
        .map(|seq| (seq.clone(), count_amino_acids(seq)))
        .collect();

    // Printing to the terminal has to stay in order, so the coloring pass
    // (which is cheap) is done sequentially over the already-parallel-computed counts.
    for i in finalcount.iter() {
        for j in i.1.iter() {
            match j.0.as_str() {
                "A" => print!("{}-{}", j.0.on_blue().bold(), j.1),
                "R" => print!("{}-{}", j.0.on_purple().bold(), j.1),
                "N" => print!("{}-{}", j.0.on_yellow().bold(), j.1),
                "D" => print!("{}-{}", j.0.on_magenta().bold(), j.1),
                "C" => print!("{}-{}", j.0.on_bright_yellow().bold(), j.1),
                "E" => print!("{}-{}", j.0.on_bright_red().bold(), j.1),
                "Q" => print!("{}-{}", j.0.on_bright_magenta().bold(), j.1),
                "G" => print!("{}-{}", j.0.on_red().bold(), j.1),
                "H" => print!("{}-{}", j.0.on_green().bold(), j.1),
                "I" => print!("{}-{}", j.0.on_bright_green().bold(), j.1),
                "L" => print!("{}-{}", j.0.on_cyan().bold(), j.1),
                "K" => print!("{}-{}", j.0.on_bright_cyan().bold(), j.1),
                "M" => print!("{}-{}", j.0.on_black().bold(), j.1),
                "F" => print!("{}-{}", j.0.on_bright_black().bold(), j.1),
                "P" => print!("{}-{}", j.0.on_white().bold(), j.1),
                "S" => print!("{}-{}", j.0.on_bright_white().bold(), j.1),
                "T" => print!("{}-{}", j.0.on_blue().underline(), j.1),
                "W" => print!("{}-{}", j.0.on_magenta().underline(), j.1),
                "Y" => print!("{}-{}", j.0.on_purple().underline(), j.1),
                "V" => print!("{}-{}", j.0.on_cyan().underline(), j.1),
                _ => {}
            }
        }
        println!();
    }
    let _ = header;
    Ok("proteome stats have been written".to_string())
}
