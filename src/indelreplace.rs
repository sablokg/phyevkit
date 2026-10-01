use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
Gaurav Sablok,
gsablok@proton.me
*/

pub fn substitute(path: &str, letter: &str) -> Result<String, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);

    let mut header: Vec<String> = Vec::new();
    let mut seq: Vec<String> = Vec::new();

    for i in fileread.lines() {
        let line = i.expect("line not found");
        if line.starts_with(">") {
            header.push(line.replace(">", ""));
        } else if !line.starts_with(">") {
            seq.push(line);
        }
    }

    let seqreplace: Vec<String> = seq.par_iter().map(|i| i.replace("-", letter)).collect();

    let mut filewrite = File::create("substitution-replaced.txt").expect("file not found");
    for i in 0..seqreplace.len() {
        writeln!(filewrite, ">{:?}\n{:?}", header[i], seqreplace[i]).expect("line not found");
    }

    Ok("Substitutions have been made".to_string())
}
