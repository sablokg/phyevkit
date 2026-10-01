use colored::Colorize;
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok,
gsablok@proton.me
*/

/// Render a single base as its colored terminal representation, or an empty
/// string for anything outside A/T/C/G/- (matching the original behavior of
/// silently skipping unrecognized characters).
fn color_base(c: char) -> String {
    match c {
        'A' => c.to_string().on_yellow().bold().to_string(),
        'T' => c.to_string().on_red().bold().to_string(),
        'C' => c.to_string().on_blue().bold().to_string(),
        'G' => c.to_string().on_green().bold().to_string(),
        '-' => c.to_string().on_purple().bold().to_string(),
        _ => String::new(),
    }
}

pub fn alignment_embedded_common(path: &str) -> Result<String, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(&fileopen);
    let mut hold_header: Vec<String> = Vec::new();
    let mut hold_sequence: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            hold_header.push(line);
        } else {
            hold_sequence.push(line);
        }
    }

    // Building the colored representation of each row is independent work,
    // so it's computed across the rayon pool. Terminal output has to stay
    // in the original row order though, so the actual printing below is a
    // plain sequential loop over the already-computed strings.
    let colored_rows: Vec<String> = hold_sequence
        .par_iter()
        .map(|seq| seq.chars().map(color_base).collect::<String>())
        .collect();

    for row in colored_rows.iter() {
        println!("{}", row);
    }

    let _ = hold_header;
    Ok("The alignment has been printed".to_string())
}
