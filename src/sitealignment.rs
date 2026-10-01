use crate::filesearch::read;
use rayon::prelude::*;
use std::error::Error;

/*
Gaurav Sablok,
gsablok@proton.me
*/

pub fn sitespecific(path: &str, head: &str, base: &str) -> Result<String, Box<dyn Error>> {
    let (header, sequence) = read(path).unwrap();

    // NOTE: the previous implementation collected enumerated positions with
    // `.map(...)` assigned to `_`, but `map` is lazy and nothing ever drove
    // the iterator, so the closure never ran and `information` only ever
    // held empty Vecs. Using a real loop (`.filter()` below) fixes that.
    //
    // Matching records are independent of one another, so once the matching
    // header is selected, each of their per-base site searches runs on the
    // rayon pool.
    let information: Vec<Vec<usize>> = header
        .par_iter()
        .zip(sequence.par_iter())
        .filter(|(h, _)| h.as_str() == head)
        .map(|(_, seq)| {
            seq.chars()
                .enumerate()
                .filter(|(_, c)| c.to_string() == base)
                .map(|(pos, _)| pos)
                .collect::<Vec<usize>>()
        })
        .collect();

    let _ = information;
    Ok(
        "The specific information for those bases in the entire alignment have been written"
            .to_string(),
    )
}
