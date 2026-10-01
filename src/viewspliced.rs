use colored::Colorize;
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok,
codeprog@icloud.com
*/

fn color_base(c: char) -> String {
    match c {
        'A' => c.to_string().on_yellow().bold().to_string(),
        'T' => c.to_string().on_red().bold().to_string(),
        'C' => c.to_string().on_green().bold().to_string(),
        'G' => c.to_string().on_blue().bold().to_string(),
        '-' => c.to_string().on_purple().bold().to_string(),
        _ => String::new(),
    }
}

pub fn splicedalignment(path: &str, start: usize, end: usize) -> Result<String, Box<dyn Error>> {
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

    // Clipping to [start, end) and coloring each row is independent work
    // per record, run across the rayon pool. Printing stays sequential to
    // preserve row order in the terminal.
    let colored_rows: Vec<String> = hold_sequence
        .par_iter()
        .map(|seq| {
            let chars: Vec<char> = seq.chars().collect();
            chars[start..end].iter().map(|&c| color_base(c)).collect::<String>()
        })
        .collect();

    for row in colored_rows.iter() {
        println!("{}", row);
    }

    let _ = hold_header;
    Ok("The spliced alignment has been written".to_string())
}
