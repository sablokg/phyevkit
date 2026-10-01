use crate::astruct::Alignment;
use crate::astruct::AlignmentStat;
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
Gaurav Sablok,
gsablok@proton.me
*/

pub fn alignmentstats(path: &str) -> Result<String, Box<dyn Error>> {
    let alignmentopen = File::open(path).expect("file not found");
    let alignmentread = BufReader::new(alignmentopen);

    let mut header: Vec<String> = Vec::new();
    let mut sequence: Vec<String> = Vec::new();
    let mut mergeseq: Vec<Alignment> = Vec::new();
    for i in alignmentread.lines() {
        let line = i.expect("line not found");
        if line.starts_with(">") {
            header.push(line);
        } else if !line.starts_with(">") {
            sequence.push(line);
        }
    }
    for i in 0..header.len() {
        mergeseq.push(Alignment {
            head: header[i].clone(),
            seq: sequence[i].clone(),
            length: sequence[i].len(),
        })
    }
    // Per-record stats are entirely independent of one another, so each
    // record is farmed out to the rayon thread pool instead of processed
    // one at a time. Base counting is also collapsed into a single pass
    // over the sequence rather than six separate filtered scans.
    let newheader: Vec<AlignmentStat> = mergeseq
        .par_iter()
        .map(|i| {
            let (mut basea, mut baset, mut baseg, mut basec, mut basen, mut baseabsent) =
                (0usize, 0usize, 0usize, 0usize, 0usize, 0usize);
            for x in i.seq.chars() {
                match x {
                    'A' => basea += 1,
                    'T' => baset += 1,
                    'G' => baseg += 1,
                    'C' => basec += 1,
                    'N' => basen += 1,
                    '-' => baseabsent += 1,
                    _ => {}
                }
            }
            AlignmentStat {
                name: i.head.clone(),
                length: i.seq.len(),
                basea,
                baset,
                baseg,
                basec,
                basen,
                baseabsent,
                gccontent: (baseg as f32 + basec as f32)
                    / (basea as f32 + baset as f32 + baseg as f32 + basec as f32),
            }
        })
        .collect();

    let mut filewrite = File::create("alignment-stats.txt").expect("file not found");
    for i in newheader.iter() {
        writeln!(
            filewrite,
            "{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}",
            i.name,
            i.length,
            i.basea,
            i.baset,
            i.baseg,
            i.basec,
            i.basen,
            i.baseabsent,
            i.gccontent
        )
        .expect("line not found");
    }

    Ok("the alignment stats for the given alignment are as follows".to_string())
}
