#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Alignment {
    pub head: String,
    pub seq: String,
    pub length: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AlignmentStat {
    pub name: String,
    pub length: usize,
    pub basea: usize,
    pub baset: usize,
    pub baseg: usize,
    pub basec: usize,
    pub basen: usize,
    pub baseabsent: usize,
    pub gccontent: f32,
}

/// Ordered list of the 20 standard amino acid one-letter codes.
pub const AMINO_ACIDS: [char; 20] = [
    'A', 'R', 'N', 'D', 'C', 'E', 'Q', 'G', 'H', 'I', 'L', 'K', 'M', 'F', 'P', 'S', 'T', 'W', 'Y',
    'V',
];

/// Count occurrences of each standard amino acid in a single sequence in one
/// pass, returning the same (letter, count) pairs the old per-branch `if`
/// chains produced, in the same order.
pub fn count_amino_acids(seq: &str) -> Vec<(String, usize)> {
    let mut counts = [0usize; 20];
    for c in seq.chars() {
        if let Some(pos) = AMINO_ACIDS.iter().position(|&a| a == c) {
            counts[pos] += 1;
        }
    }
    AMINO_ACIDS
        .iter()
        .zip(counts.iter())
        .map(|(letter, count)| (letter.to_string(), *count))
        .collect()
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Plotter {
    pub name: String,
    pub count_a: usize,
    pub count_t: usize,
    pub count_g: usize,
    pub count_c: usize,
}
