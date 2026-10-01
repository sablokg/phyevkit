use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
Gaurav Sablok,
gsablok@proton.me
*/

pub fn filtersiteremoval(path: &str, base: &str) -> Result<String, Box<dyn Error>> {
    let pathopen = File::open(path).expect("file not found");
    let pathread = BufReader::new(pathopen);

    let mut header: Vec<String> = Vec::new();
    let mut seq: Vec<String> = Vec::new();

    for i in pathread.lines() {
        let line = i.expect("line not found");
        if line.starts_with(">") {
            header.push(line.replace(">", ""))
        } else if !line.starts_with(">") {
            seq.push(line)
        }
    }

    let (headerfilter, seqfilter): (Vec<String>, Vec<String>) = header
        .par_iter()
        .zip(seq.par_iter())
        .map(|(h, s)| {
            let filtered: String = s.chars().filter(|x| x.to_string() != base).collect();
            (h.clone(), filtered)
        })
        .unzip();

    let mut filewrite = File::create("filtered-sites.fasta").expect("file not found");
    for i in 0..headerfilter.len() {
        writeln!(filewrite, ">{:?}\n{:?}", headerfilter[i], seqfilter[i]).expect("line not found");
    }

    Ok("the filtered sites have been written".to_string())
}
