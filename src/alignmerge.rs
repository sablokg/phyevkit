use crate::astruct::Alignment;
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
Gaurav Sablok,
gsablok@proton.me
*/

pub fn alignmergeall(
    path: &str,
    start: &str,
    end: &str,
    mergestr: &str,
) -> Result<String, Box<dyn Error>> {
    let pathopen = File::open(path).expect("file not found");
    let pathread = BufReader::new(pathopen);
    let mut header: Vec<String> = Vec::new();
    let mut sequence: Vec<String> = Vec::new();
    for i in pathread.lines() {
        let line = i.expect("file not found");
        if line.starts_with(">") {
            header.push(line.replace(">", ""));
        } else if !line.starts_with(">") {
            sequence.push(line)
        }
    }
    let allseq: Vec<Alignment> = header
        .par_iter()
        .zip(sequence.par_iter())
        .map(|(h, s)| Alignment {
            head: h.clone(),
            seq: s.clone(),
            length: s.len(),
        })
        .collect();

    let alstart: usize = start.parse::<usize>().unwrap();
    let alend: usize = end.parse::<usize>().unwrap();

    // Slicing each record's region and splitting it back into
    // parallel header/sequence vectors is independent per record.
    let (spliced_header, spliced_seq): (Vec<String>, Vec<String>) = allseq
        .par_iter()
        .map(|i| (i.head.clone(), i.seq[alstart..alend].to_string()))
        .unzip();
    let _ = spliced_header;
    let mergehead = mergestr;
    let sequencemerge: String = spliced_seq.concat();
    println!(
        "The merged header and the merged sequence for the regions specific is \n>{}\n{}",
        mergehead, sequencemerge
    );

    let mut filewrite = File::create("alignment-merged.fasta").expect("file not found");
    writeln!(filewrite, ">{}\n{}", mergehead, sequencemerge).expect("file not found");

    Ok("The final merged sequence has been written".to_string())
}
