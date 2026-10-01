use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
Gaurav Sablok,
gsablok@proton.me
*/

pub fn filtersiteall(path: &str) -> Result<String, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);

    let mut head: Vec<String> = Vec::new();
    let mut seq: Vec<String> = Vec::new();

    for i in fileread.lines() {
        let line = i.expect("line not found");
        if line.starts_with(">") {
            head.push(line);
        } else if !line.starts_with(">") {
            seq.push(line)
        }
    }

    if seq.is_empty() {
        return Err("no sequences found in alignment".into());
    }
    let aln_len = seq[0].len();
    for s in seq.iter() {
        if s.len() != aln_len {
            return Err(
                "sequences are not aligned: all records must be the same length".into(),
            );
        }
    }

    // NOTE: the previous implementation cross-compared every character of
    // each sequence against every character of the *next* sequence,
    // building concatenated diff strings with no relationship to alignment
    // columns. That produced a result Vec whose length had nothing to do
    // with the number of headers, so zipping it against `head` positionally
    // would panic with an out-of-bounds index as soon as any real alignment
    // was used. "removes same bases across all the alignment" is
    // implemented here as it reads: strip out any alignment column where
    // every sequence has the same base (invariant sites), keeping only the
    // variable/polymorphic columns - a standard filtering step before
    // things like parsimony analysis.
    //
    // Checking whether a column is invariant is independent per column, so
    // it's run across the rayon pool.
    let variable_cols: Vec<usize> = (0..aln_len)
        .into_par_iter()
        .filter(|&col| {
            let first = seq[0].as_bytes()[col];
            seq.iter().any(|s| s.as_bytes()[col] != first)
        })
        .collect();

    // Building each sequence's filtered (variable-sites-only) string is
    // independent per sequence, so this is parallelized too.
    let filteredseq: Vec<String> = seq
        .par_iter()
        .map(|s| {
            let bytes = s.as_bytes();
            variable_cols.iter().map(|&c| bytes[c] as char).collect()
        })
        .collect();

    let mut filewrite = File::create("siteremoval.fasta").expect("file not found");
    for (h, s) in head.iter().zip(filteredseq.iter()) {
        writeln!(filewrite, "{}\n{}", h, s).expect("file not found");
    }

    Ok(format!(
        "kept {} variable column(s) out of {} ({} invariant columns removed); written to siteremoval.fasta",
        variable_cols.len(),
        aln_len,
        aln_len - variable_cols.len()
    ))
}
