use rayon::prelude::*;
use std::collections::HashSet;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::Path;
use std::process;

struct Record {
    header: String,
    seq: String,
}

struct Options {
    strict: bool,
    datatype: String,
    pad: bool,
    interleave: bool,
}

pub fn convertfasta(inputpath: &str) {
    let input_path = inputpath;
    let output_prefix: Option<String> = None;
    let opts = Options {
        strict: false,
        datatype: "dna".to_string(),
        pad: false,
        interleave: false,
    };

    if !["dna", "rna", "protein", "standard"].contains(&opts.datatype.as_str()) {
        eprintln!(
            "Error: --datatype must be one of dna, rna, protein, standard (got '{}').",
            opts.datatype
        );
        process::exit(1);
    }

    let stem = Path::new(input_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("alignment");
    let prefix = output_prefix.unwrap_or_else(|| stem.to_string());

    let raw = fs::read_to_string(input_path).unwrap_or_else(|e| {
        eprintln!("Error reading '{}': {}", input_path, e);
        process::exit(1);
    });

    let mut records = parse_fasta(&raw).unwrap_or_else(|e| {
        eprintln!("Error parsing FASTA: {}", e);
        process::exit(1);
    });

    if records.is_empty() {
        eprintln!("Error: no sequences found in '{}'.", input_path);
        process::exit(1);
    }

    // Build sanitized, de-duplicated taxon names before touching sequence lengths.
    assign_taxon_names(&mut records, opts.strict);

    // Verify alignment (equal lengths), or pad if requested.
    let max_len = records.iter().map(|r| r.seq.len()).max().unwrap();
    let min_len = records.iter().map(|r| r.seq.len()).min().unwrap();
    if max_len != min_len {
        if opts.pad {
            for r in &mut records {
                let pad = max_len - r.seq.len();
                if pad > 0 {
                    r.seq.push_str(&"-".repeat(pad));
                }
            }
            eprintln!(
                "Warning: input sequences had unequal lengths ({}-{}); \
                 right-padded shorter ones with '-' to {} bp.",
                min_len, max_len, max_len
            );
        } else {
            eprintln!(
                "Error: sequences are not aligned (lengths range {}-{}). \
                 Align them first (e.g. with MAFFT/MUSCLE), or pass --pad to \
                 right-pad shorter sequences with gaps.",
                min_len, max_len
            );
            process::exit(1);
        }
    }

    let nchar = max_len;
    let ntax = records.len();

    // Sanity-check the declared datatype against observed characters (warning only).
    warn_if_datatype_mismatch(&records, &opts.datatype);

    let phy_path = format!("{}.phy", prefix);
    let nex_path = format!("{}.nex", prefix);

    let phy = build_phylip(&records, ntax, nchar, opts.strict);
    fs::write(&phy_path, phy).unwrap_or_else(|e| {
        eprintln!("Error writing '{}': {}", phy_path, e);
        process::exit(1);
    });

    let nex = build_nexus(&records, ntax, nchar, &opts);
    fs::write(&nex_path, nex).unwrap_or_else(|e| {
        eprintln!("Error writing '{}': {}", nex_path, e);
        process::exit(1);
    });

    println!(
        "Converted {} taxa x {} characters from '{}'.",
        ntax, nchar, input_path
    );
    println!(
        "  PHYLIP ({}) -> {}",
        if opts.strict { "strict" } else { "relaxed" },
        phy_path
    );
    println!(
        "  NEXUS ({}) -> {}",
        if opts.interleave {
            "interleaved"
        } else {
            "sequential"
        },
        nex_path
    );
}

/// Parse a (possibly multi-line) FASTA string into records, preserving order.
fn parse_fasta(raw: &str) -> Result<Vec<Record>, String> {
    let mut records = Vec::new();
    let mut header: Option<String> = None;
    let mut seq = String::new();

    for (i, line) in raw.lines().enumerate() {
        let line = line.trim_end_matches(['\r', '\n']);
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix('>') {
            if let Some(h) = header.take() {
                records.push(Record {
                    header: h,
                    seq: std::mem::take(&mut seq),
                });
            }
            header = Some(rest.trim().to_string());
        } else {
            if header.is_none() {
                return Err(format!(
                    "line {}: sequence data found before any '>' header",
                    i + 1
                ));
            }
            seq.push_str(line.trim());
        }
    }
    if let Some(h) = header.take() {
        records.push(Record { header: h, seq });
    }
    Ok(records)
}

/// Turn each FASTA header into a safe, unique taxon name and store it back
/// into `record.header` (overwriting the full description). PHYLIP/NEXUS
/// names can't contain whitespace or most punctuation, and strict PHYLIP
/// truncates to 10 characters, so we de-duplicate after truncation too.
fn assign_taxon_names(records: &mut [Record], strict: bool) {
    let mut used: HashSet<String> = HashSet::new();

    for r in records.iter_mut() {
        // Taxon name = first whitespace-delimited token of the header.
        let first_token = r.header.split_whitespace().next().unwrap_or("taxon");
        let mut name: String = first_token
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '-' {
                    c
                } else {
                    '_'
                }
            })
            .collect();

        if name.is_empty() {
            name = "taxon".to_string();
        }
        if strict && name.len() > 10 {
            name.truncate(10);
        }

        // De-duplicate by appending a numeric suffix, respecting the strict
        // 10-char limit by trimming the base name to make room.
        if used.contains(&name) {
            let mut n = 2;
            loop {
                let suffix = n.to_string();
                let base = if strict {
                    let max_base = 10usize.saturating_sub(suffix.len());
                    let mut b = name.clone();
                    if strict {
                        b.truncate(max_base.min(b.len()));
                    }
                    b
                } else {
                    name.clone()
                };
                let candidate = format!("{}{}", base, suffix);
                if !used.contains(&candidate) {
                    name = candidate;
                    break;
                }
                n += 1;
            }
        }

        used.insert(name.clone());
        r.header = name;
    }
}

fn warn_if_datatype_mismatch(records: &[Record], datatype: &str) {
    if datatype == "standard" {
        return;
    }
    let mut acgt = 0u64;
    let mut u = 0u64;
    let mut other = 0u64;
    let mut total = 0u64;
    for r in records {
        for c in r.seq.chars() {
            let uc = c.to_ascii_uppercase();
            total += 1;
            match uc {
                'A' | 'C' | 'G' | 'T' | 'N' | '-' | '?' => acgt += 1,
                'U' => u += 1,
                _ => other += 1,
            }
        }
    }
    if total == 0 {
        return;
    }
    let nucleotide_like = (acgt + u) as f64 / total as f64 > 0.9;
    match datatype {
        "dna" | "rna" if !nucleotide_like => {
            eprintln!(
                "Warning: --datatype {} was requested, but the sequences look more like \
                 protein data ({} non-nucleotide characters out of {}). Consider \
                 --datatype protein if this is unexpected.",
                datatype, other, total
            );
        }
        "protein" if nucleotide_like => {
            eprintln!(
                "Warning: --datatype protein was requested, but the sequences look mostly \
                 like nucleotide data. Consider --datatype dna/rna if this is unexpected."
            );
        }
        _ => {}
    }
}

/// Build PHYLIP text (relaxed or strict, sequential).
fn build_phylip(records: &[Record], ntax: usize, nchar: usize, strict: bool) -> String {
    let mut out = String::new();
    let _ = writeln!(out, " {} {}", ntax, nchar);
    // Each record's PHYLIP line is independent, so formatting is done in
    // parallel across the rayon pool and joined afterward in order.
    let lines: String = records
        .par_iter()
        .map(|r| {
            if strict {
                let mut name = r.header.clone();
                if name.len() > 10 {
                    name.truncate(10);
                }
                while name.len() < 10 {
                    name.push(' ');
                }
                format!("{}{}\n", name, r.seq)
            } else {
                format!("{}  {}\n", r.header, r.seq)
            }
        })
        .collect();
    out.push_str(&lines);
    out
}

/// Build a NEXUS DATA block (sequential by default, optionally interleaved
/// in blocks of 500 characters).
fn build_nexus(records: &[Record], ntax: usize, nchar: usize, opts: &Options) -> String {
    let nexus_datatype = match opts.datatype.as_str() {
        "dna" => "DNA",
        "rna" => "RNA",
        "protein" => "PROTEIN",
        _ => "STANDARD",
    };

    let name_width = records.iter().map(|r| r.header.len()).max().unwrap_or(1) + 2;

    let mut out = String::new();
    out.push_str("#NEXUS\n");
    out.push_str("BEGIN DATA;\n");
    let _ = writeln!(out, "  DIMENSIONS NTAX={} NCHAR={};", ntax, nchar);
    let _ = writeln!(
        out,
        "  FORMAT DATATYPE={} MISSING=? GAP=- INTERLEAVE={};",
        nexus_datatype,
        if opts.interleave { "YES" } else { "NO" }
    );
    out.push_str("  MATRIX\n");

    if opts.interleave {
        const BLOCK: usize = 500;
        let mut pos = 0;
        while pos < nchar {
            let end = (pos + BLOCK).min(nchar);
            // Records within a block are independent to format; the block
            // loop itself stays sequential (there are far fewer blocks than
            // records for realistic alignments, so the per-record work
            // inside each block is where the parallelism pays off).
            let block_lines: String = records
                .par_iter()
                .map(|r| {
                    let chunk = &r.seq[pos..end];
                    format!("  {:width$}{}\n", r.header, chunk, width = name_width)
                })
                .collect();
            out.push_str(&block_lines);
            out.push('\n');
            pos = end;
        }
        // Trim the trailing blank line before the closing semicolon.
        while out.ends_with("\n\n") {
            out.pop();
        }
        out.push('\n');
    } else {
        let lines: String = records
            .par_iter()
            .map(|r| format!("  {:width$}{}\n", r.header, r.seq, width = name_width))
            .collect();
        out.push_str(&lines);
    }

    out.push_str("  ;\n");
    out.push_str("END;\n");
    out
}
