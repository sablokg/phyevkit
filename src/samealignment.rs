use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
Gaurav Sablok,
gsablok@proton.me
*/

pub fn dealignment(path: &str) -> Result<String, Box<dyn Error>> {
    let alignmentopen = File::open(path).expect("file not found");
    let alignmentread = BufReader::new(alignmentopen);

    let mut header: Vec<String> = Vec::new();
    let mut seq: Vec<String> = Vec::new();
    for i in alignmentread.lines() {
        let line = i.expect("line not found");
        if line.starts_with(">") {
            header.push(line.replace(">", ""));
        } else if !line.starts_with(">") {
            seq.push(line);
        }
    }

    // NOTE: the previous implementation deduped headers and sequences into
    // two *independent* HashSets. That breaks the header<->sequence
    // pairing entirely (a HashSet's iteration order isn't even the
    // insertion order), and whenever any duplicate sequence existed the two
    // sets ended up different sizes, so indexing them together by the same
    // position panicked with an out-of-bounds index. Deduplication has to
    // happen on the (header, sequence) pairing, keeping the first
    // occurrence of each distinct sequence in its original order. This is
    // an inherently sequential scan (each decision depends on what's been
    // seen so far), so it isn't a rayon candidate the way the per-record
    // work elsewhere in this codebase is.
    let mut seen: HashSet<String> = HashSet::new();
    let mut uniquealignment = File::create("filtered-alignment.fasta").expect("file not found");
    let mut kept = 0usize;
    for (h, s) in header.iter().zip(seq.iter()) {
        if seen.insert(s.clone()) {
            writeln!(uniquealignment, ">{}\n{}", h, s).expect("line not found");
            kept += 1;
        }
    }

    Ok(format!(
        "kept {} unique record(s) out of {} ({} duplicate(s) removed); written to filtered-alignment.fasta",
        kept,
        header.len(),
        header.len() - kept
    ))
}
