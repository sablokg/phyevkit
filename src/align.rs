use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
Gaurav Sablok,
gsablok@proton.me
*/

pub fn alignmerge(path: &str, mergeheader: &str) -> Result<String, Box<dyn Error>> {
    let alignmentopen = File::open(path).expect("file not found");

    let alignmentread = BufReader::new(alignmentopen);

    let mergehead = mergeheader.to_string();

    let mut header: Vec<String> = Vec::new();
    let mut sequence: Vec<String> = Vec::new();
    let mut length: Vec<usize> = Vec::new();
    for i in alignmentread.lines() {
        let line = i.expect("line not found");
        if line.starts_with(">") {
            header.push(line.replace(">", ""));
        } else if !line.starts_with(">") {
            sequence.push(line);
        } else if !line.starts_with(">") {
            length.push(line.len());
        }
    }
    for i in 0..sequence.len() - 1 {
        assert_eq!(sequence[i].len(), sequence[i + 1].len())
    }
    let mergesequence: String = sequence.concat().to_string();
    println!(">{}\n{}", mergehead, mergesequence);
    let mut filewrite = File::create("alignment-stringwrite.fasta").expect("file not found");
    writeln!(filewrite, ">{}\n{}", mergehead, mergesequence).expect("file not found");

    Ok::<String, Box<dyn Error>>(
        "The merged results for the final sequences along with the header are".to_string(),
    )
}
