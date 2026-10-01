use crate::filesearch::read;
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::Write;

/*
Gaurav Sablok,
gsablok@proton.me
*/

pub fn upstreamdownstream(
    path: &str,
    focal: &str,
    upstream: &str,
    downstream: &str,
) -> Result<String, Box<dyn Error>> {
    let (header, sequence) = read(path).unwrap();

    let focalcap: usize = focal.parse::<usize>().unwrap();
    let upstreamcap: usize = upstream.parse::<usize>().unwrap();
    let downstreamcap: usize = downstream.parse::<usize>().unwrap() + 1usize;
    let clipped_region: Vec<String> = sequence
        .par_iter()
        .map(|i| i[focalcap - upstreamcap..focalcap + downstreamcap].to_string())
        .collect();

    let mut filewrite = File::create("clipped-regions.txt").expect("file not found");
    for i in 0..clipped_region.len() {
        writeln!(filewrite, ">{:?}\n{:?}", header[i], clipped_region[i]).expect("line not found");
    }

    Ok("The results file have been written".to_string())
}
