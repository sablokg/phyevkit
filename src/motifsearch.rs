use crate::filesearch::read;
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::Write;

/*
Gaurav Sablok,
gsablok@proton.me
*/

pub fn motifsearchall(path: &str, motif: &str) -> Result<String, Box<dyn Error>> {
    let (header, sequence) = read(path).unwrap();
    // Searching each sequence for the motif is independent work, so it's
    // farmed out across the rayon pool with filter_map instead of calling
    // `.find()` three times per sequence in a sequential loop.
    let returnvec: Vec<(String, usize, usize)> = header
        .par_iter()
        .zip(sequence.par_iter())
        .filter_map(|(h, s)| {
            s.find(motif)
                .map(|motifstart| (h.clone(), motifstart, motifstart + motif.len()))
        })
        .collect();

    let mut filewrite = File::create("searched-motif.txt").expect("file not found");
    for i in returnvec.iter() {
        writeln!(filewrite, "{}\t{}\t{}", i.0, i.1, i.2).expect("file not found");
    }

    for i in returnvec.iter() {
        println!("{}\t{}\t{}", i.0, i.1, i.2);
    }

    Ok("The results are as follows:".to_string())
}
