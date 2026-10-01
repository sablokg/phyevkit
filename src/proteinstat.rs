use crate::astruct::count_amino_acids;
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
Gaurav Sablok,
gsablok@proton.me
*/

pub fn proteomestats(path: &str) -> Result<String, Box<dyn Error>> {
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

    // Counting amino acids in each sequence is independent per record, so
    // it's handed to the rayon thread pool instead of looping one sequence
    // at a time. `.collect()` on an indexed parallel iterator preserves the
    // original ordering, so this still lines up with `header`/`sequence`.
    let finalcount: Vec<Vec<(String, usize)>> = sequence
        .par_iter()
        .map(|seq| count_amino_acids(seq))
        .collect();

    let mut filewrite = File::create("proteome-stats.txt").expect("file not found");
    for i in 0..finalcount.len() {
        writeln!(
            filewrite,
            "{:?}\t{:?}\t{:?}",
            header[i], sequence[i], finalcount[i]
        )
        .expect("file not found");
    }

    Ok("proteome stats have been written".to_string())
}
