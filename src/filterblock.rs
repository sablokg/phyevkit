use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
Gaurav Sablok,
gsablok@proton.me
*/

/// A block is "collinear enough" to keep if, on average across its columns,
/// this fraction of sequences agree with the column's majority base.
/// 1.0 would require perfect conservation in every column of the block;
/// 0.5 requires a simple majority on average. This mirrors the kind of
/// conservation cutoff tools like Gblocks use per block.
const COLLINEARITY_THRESHOLD: f64 = 0.5;

/// Score for a single alignment column: the fraction of sequences whose
/// base at this column matches the most common (majority) base at that
/// column. Gap characters ('-') are counted like any other symbol, since a
/// shared gap is still a form of column agreement.
fn column_score(seq: &[String], col: usize) -> f64 {
    let mut counts: HashMap<u8, usize> = HashMap::new();
    for s in seq {
        let b = s.as_bytes()[col];
        *counts.entry(b).or_insert(0) += 1;
    }
    let majority = counts.values().copied().max().unwrap_or(0);
    majority as f64 / seq.len() as f64
}

/// Score for a block of columns [start, end): the mean per-column score.
fn block_score(seq: &[String], start: usize, end: usize) -> f64 {
    let sum: f64 = (start..end).map(|col| column_score(seq, col)).sum();
    sum / (end - start) as f64
}

pub fn filterblockalignment(path: &str, block: &str) -> Result<String, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut header: Vec<String> = Vec::new();
    let mut seq: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("file not present");
        if line.starts_with(">") {
            header.push(line.replace(">", ""));
        } else if !line.starts_with(">") {
            seq.push(line)
        }
    }

    if seq.is_empty() {
        return Err("no sequences found in alignment".into());
    }

    let aln_len = seq[0].len();
    for (i, s) in seq.iter().enumerate() {
        if s.len() != aln_len {
            return Err(format!(
                "sequences are not aligned: '{}' has length {} but expected {} \
                 (block filtering requires all sequences to be the same length)",
                header.get(i).map(String::as_str).unwrap_or("?"),
                s.len(),
                aln_len
            )
            .into());
        }
    }

    let blockinner = block.parse::<usize>().unwrap();
    if blockinner == 0 {
        return Err("block size must be greater than 0".into());
    }

    // Non-overlapping column ranges covering the whole alignment.
    let ranges: Vec<(usize, usize)> = (0..aln_len)
        .step_by(blockinner)
        .map(|start| (start, (start + blockinner).min(aln_len)))
        .collect();

    // Scoring is independent per block (each covers a disjoint column
    // range), so this is exactly the kind of embarrassingly parallel work
    // rayon is for - each block's O(block_len * n_sequences) column scan
    // runs on its own thread pool worker.
    let scores: Vec<f64> = ranges
        .par_iter()
        .map(|&(start, end)| block_score(&seq, start, end))
        .collect();

    let kept: Vec<(usize, usize)> = ranges
        .iter()
        .zip(scores.iter())
        .filter(|(_, &score)| score >= COLLINEARITY_THRESHOLD)
        .map(|(&range, _)| range)
        .collect();

    // Build the filtered alignment: for each sequence, concatenate only the
    // columns from kept blocks. Building each sequence's filtered string is
    // independent per sequence, so this is also parallelized.
    let filtered_seqs: Vec<String> = seq
        .par_iter()
        .map(|s| {
            let mut out = String::with_capacity(kept.iter().map(|(a, b)| b - a).sum());
            for &(start, end) in &kept {
                out.push_str(&s[start..end]);
            }
            out
        })
        .collect();

    let mut filewrite = File::create("block-filtered-alignment.fasta").expect("file not found");
    for (h, s) in header.iter().zip(filtered_seqs.iter()) {
        writeln!(filewrite, ">{}\n{}", h, s).expect("file not found");
    }

    let removed_blocks = ranges.len() - kept.len();
    let kept_cols: usize = kept.iter().map(|(a, b)| b - a).sum();

    Ok(format!(
        "kept {}/{} blocks ({} of {} columns, threshold {:.2}), {} blocks removed; \
         written to block-filtered-alignment.fasta",
        kept.len(),
        ranges.len(),
        kept_cols,
        aln_len,
        COLLINEARITY_THRESHOLD,
        removed_blocks
    ))
}
