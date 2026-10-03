# phyloEvolve

- alignment kit for genome, metagenome, pangenome. 
- It gives you all the alignments functionalities and also gives you colour coded alignment for each alignment 
     and sub-alignment region. check the alignment view and alignment-clipview.

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.21902674.svg)](https://doi.org/10.5281/zenodo.21902674)


 ```
 git clone ssh://git@codeberg.org/gsablok/phyloevolve.git
 cd phyloevolve
 cargo build
 ```
 ```

                                                                                                                                           
                                                                   _|                                            _|         _|     _|      
                                                        _|_|_|     _|_|_|     _|    _|     _|_|     _|      _|   _|  _|          _|_|_|_|  
                                                        _|    _|   _|    _|   _|    _|   _|_|_|_|   _|      _|   _|_|       _|     _|      
                                                        _|    _|   _|    _|   _|    _|   _|           _|  _|     _|  _|     _|     _|      
                                                        _|_|_|     _|    _|     _|_|_|     _|_|_|       _|       _|    _|   _|       _|_|  
                                                        _|                          _|                                                     
                                                        _|                      _|_|                                                       
phylogenomics made easy for gene and marker alignment
    ************************************************
    Author Gaurav Sablok,
    Email: gsablok@proton.me
    ************************************************

Usage: phyevkit <COMMAND>

Commands:
  alignmerge                 merge all the alignment into a single string
  alignmergeinterval         alignmerge the specific region of the alignment
  same-alignment             remove same alignment
  alignmentstats             alignmentstats
  filter-site                filter the sites with the given base
  filter-all                 removes same bases across all the alignment
  filter-block               collineratiy block based alignment filtering
  alignment-view             allows for the view of the alignment
  alignment-clipview         allows for the embedded view of the clipped Alignment
  sitereplace                replace the specific sites based on the site probability
  protein-stat               estimate the protein stats
  indelreplace               indel substituter
  motif-search               search for the specific protein and the nucleotide motifs
  up-down-stream             specific part of the alignment and the upstream and the downstream of alignment
  site-alignment             extract the places of the specified base for LD analysis
  plotter                    frequency plot for the alignments
  nucleotidecolour           color coded map
  proteomecolour             proteome heatmap
  alignment-html             alignment to html
  sub-alignment-html         sub alignment to html
  alignment-convert          convert alignment
  padded-sub-alignment-html  padded sub alignment to html
  help                       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

 ```

### Example data

The [`examples/`](examples/) directory has three small FASTA files used in
every example below:

- **`examples/nucleotide_alignment.fasta`** — 6 aligned DNA sequences, 60bp
  each. Columns 1-20 and 41-60 are identical across all sequences; columns
  21-40 differ per sequence. `seq4` is an exact duplicate of `seq1`. Every
  sequence starts with the motif `ATGC`.
- **`examples/protein_alignment.fasta`** — 3 aligned protein sequences (20
  standard amino acids each).
- **`examples/indel_alignment.fasta`** — 3 short DNA sequences containing
  gap (`-`) characters.

You can swap in your own FASTA files anywhere `<ALIGNMENT>` / `<INPUTFASTA>`
appears below — all commands here just take a path.

### Commands

Run `phyloevolve --help` or `phyloevolve <COMMAND> --help` for the full,
generated argument list at any time.

---

### `alignmerge` — merge every record into one sequence

```sh
phyloevolve alignmerge examples/nucleotide_alignment.fasta merged_all 4
```

Concatenates all sequences in the file into a single sequence under a new
header (`merged_all`), and writes `alignment-stringwrite.fasta`.

---

### `alignmergeinterval` — merge a specific column range

```sh
phyloevolve alignmergeinterval examples/nucleotide_alignment.fasta 20 40 merged_block2 4
```

Same as `alignmerge`, but only over columns `[20, 40)` (the divergent middle
block in the example file). Writes `alignment-merged.fasta`.

---

### `same-alignment` — remove duplicate records

```sh
phyloevolve same-alignment examples/nucleotide_alignment.fasta 4
```

Keeps the first record for each distinct sequence and drops the rest.
`seq4` (identical to `seq1`) is removed, so the example goes from 6 records
to 5. Writes `filtered-alignment.fasta`.

```
kept 5 unique record(s) out of 6 (1 duplicate(s) removed); written to filtered-alignment.fasta
```

---

### `alignmentstats` — per-record base composition

```sh
phyloevolve alignmentstats examples/nucleotide_alignment.fasta 4
```

Writes `alignment-stats.txt`: one row per record with length, A/T/G/C/N/gap
counts, and GC content.

```
"seq1"  60  10  10  20  20  0  0  0.6666667
"seq2"  60  20  20  10  10  0  0  0.33333334
"seq3"  60  10  10  20  20  0  0  0.6666667
...
```

---

### `filter-site` — strip a specific base out of every sequence

```sh
phyloevolve filter-site examples/nucleotide_alignment.fasta G 4
```

Removes every `G` character from every sequence (this collapses column
positions rather than preserving alignment — useful for degapping-style
cleanup, not for keeping columns in register). Writes `filtered-sites.fasta`.

---

### `filter-all` — remove invariant (non-polymorphic) columns

```sh
phyloevolve filter-all examples/nucleotide_alignment.fasta 4
```

Keeps only alignment columns where at least one sequence differs from the
rest, discarding columns that are identical across every sequence — a
standard step before parsimony-style analyses. Writes `siteremoval.fasta`.

On the example file this correctly identifies the 20-column divergent
region and discards the 40 conserved columns:

```
kept 20 variable column(s) out of 60 (40 invariant columns removed); written to siteremoval.fasta
```

---

### `filter-block` — collinearity-based block filtering

```sh
phyloevolve filter-block examples/nucleotide_alignment.fasta 20 4
```

Splits the alignment into non-overlapping blocks of `20` columns, scores
each block by average per-column majority agreement, and keeps only blocks
scoring ≥ 0.5. Writes `block-filtered-alignment.fasta`. On the example file
it keeps the two conserved 20bp blocks and drops the divergent middle one:

```
kept 2/3 blocks (40 of 60 columns, threshold 0.50), 1 blocks removed; written to block-filtered-alignment.fasta
```

---

### `alignment-view` — colorized terminal view

```sh
phyloevolve alignment-view examples/nucleotide_alignment.fasta 4
```

Prints every sequence to the terminal with A/T/G/C/gap color-coded.

---

### `alignment-clipview` — colorized view of a column range

```sh
phyloevolve alignment-clipview examples/nucleotide_alignment.fasta 20 30 4
```

Same as `alignment-view`, restricted to columns `[20, 30)`.

---

### `sitereplace` — replace one base with another

```sh
phyloevolve sitereplace examples/nucleotide_alignment.fasta G N 4
```

Replaces every occurrence of `G` with `N` in every sequence. Writes
`replaced-sites.fasta`.

---

### `protein-stat` — amino acid composition

```sh
phyloevolve protein-stat examples/protein_alignment.fasta 4
```

Writes `proteome-stats.txt`: for each record, its sequence and a count of
each of the 20 standard amino acids.

---

### `indelreplace` — substitute gap characters

```sh
phyloevolve indelreplace examples/indel_alignment.fasta N 4
```

Replaces every `-` with `N`. Writes `substitution-replaced.txt`.

```
>"i1"
"ATGNNCATG"
>"i2"
"ATGGGCATG"
>"i3"
"ATNNGCATG"
```

---

### `motif-search` — find a motif in every sequence

```sh
phyloevolve motif-search examples/nucleotide_alignment.fasta ATGC 4
```

Reports the start/end column of the first match of `ATGC` in each sequence
that contains it. Writes `searched-motif.txt`:

```
seq1  0  4
seq2  0  4
seq3  0  4
seq4  0  4
seq5  0  4
seq6  0  4
```

---

### `up-down-stream` — clip a window around a focal site

```sh
phyloevolve up-down-stream examples/nucleotide_alignment.fasta 30 5 5 4
```

For each sequence, extracts 5 columns upstream and 5 columns downstream of
column 30. Writes `clipped-regions.txt`.

---

### `site-alignment` — locate a base within one record

```sh
phyloevolve site-alignment examples/nucleotide_alignment.fasta seq1 A 4
```

Finds the column positions of base `A` within the named record (`seq1`).

---

### `plotter` — base-frequency bar chart

```sh
phyloevolve plotter examples/nucleotide_alignment.fasta 4
```

Writes `heatplot.txt` (per-record A/T/C/G counts) and `line-chart.svg` (a
horizontal bar chart of those counts).

---

### `nucleotidecolour` — colorized per-record base counts

```sh
phyloevolve nucleotidecolour examples/nucleotide_alignment.fasta 4
```

Prints each record's A/T/G/C counts to the terminal with color-coded
backgrounds.

---

### `proteomecolour` — colorized per-record amino acid counts

```sh
phyloevolve proteomecolour examples/protein_alignment.fasta 4
```

Same idea as `nucleotidecolour`, for the 20 standard amino acids.

---

### `alignment-html` — full alignment as a styled HTML page

```sh
phyloevolve alignment-html examples/nucleotide_alignment.fasta alignment_view.html 4
```

Renders the whole alignment (all columns) as a color-coded HTML table with
a consensus row, written to the given output path.

---

### `sub-alignment-html` — HTML view of a column range

```sh
phyloevolve sub-alignment-html examples/nucleotide_alignment.fasta 1 20 block1_view 4
```

Extracts columns `[1, 20]` (1-indexed, inclusive) and writes both
`block1_view.fasta` and `block1_view.html`.

---

### `padded-sub-alignment-html` — HTML view in fixed-width blocks

```sh
phyloevolve padded-sub-alignment-html examples/nucleotide_alignment.fasta 1 60 padded_view 4
```

Same idea as `sub-alignment-html`, but renders the HTML output stacked in
20-column blocks (useful for wide alignments). Writes `padded_view.fasta`
and `padded_view.html`.

> `<start>` is 1-indexed for both `sub-alignment-html` and
> `padded-sub-alignment-html` — `0` is rejected with a clear error rather
> than a crash.

---

### `alignment-convert` — FASTA to PHYLIP/NEXUS

```sh
phyloevolve alignment-convert examples/nucleotide_alignment.fasta 4
```

Writes `<input-stem>.phy` (relaxed PHYLIP) and `<input-stem>.nex`
(sequential NEXUS) next to the input file's basename, e.g.
`nucleotide_alignment.phy` / `nucleotide_alignment.nex`.
---

### Benchmarking 

| command                   | time_15mb | ram_mb_15mb | time_50mb | ram_mb_50mb | time_100mb | ram_mb_100mb |
|---------------------------|-----------|-------------|-----------|-------------|------------|--------------|
| alignmerge                | 0:09.06   | 54.5        | 1:43.58   | 156.6       | OOM-killed | N/A          |
| alignmergeinterval        | 0:00.03   | 40.1        | 0:00.07   | 108.4       | 0:00.28    | 205.3        |
| same-alignment            | 0:00.12   | 39.8        | 0:00.56   | 108.4       | 0:01.08    | 205.2        |
| alignmentstats            | 0:00.23   | 40.1        | 0:00.78   | 108.5       | 0:01.64    | 205.2        |
| filter-site               | 0:01.24   | 40.5        | 0:04.23   | 107.2       | 0:08.60    | 203.4        |
| filter-all                | 0:00.23   | 41.3        | 0:00.81   | 111.7       | 0:01.82    | 212.4        |
| filter-block              | 0:02.75   | 25.8        | 0:08.30   | 61.1        | 0:17.18    | 110.5        |
| sitereplace               | 0:00.16   | 39.9        | 0:00.57   | 108.6       | 0:01.26    | 205.0        |
| protein-stat              | 0:00.80   | 25.7        | 0:02.56   | 59.7        | 0:05.12    | 108.0        |
| indelreplace              | 0:00.16   | 40.2        | 0:00.60   | 108.5       | 0:01.20    | 205.3        |
| motif-search              | 0:00.01   | 25.7        | 0:00.04   | 59.5        | 0:00.08    | 108.0        |
| up-down-stream            | 0:00.01   | 25.6        | 0:00.04   | 59.6        | 0:00.08    | 108.1        |
| site-alignment            | 0:00.03   | 26.0        | 0:00.08   | 60.7        | 0:00.16    | 110.0        |
| plotter                   | 0:00.26   | 27.0        | 0:00.79   | 60.6        | 0:01.61    | 109.1        |
| nucleotidecolour          | 0:00.25   | 25.7        | 0:00.80   | 59.7        | 0:01.61    | 107.8        |
| proteomecolour            | 0:00.70   | 40.3        | 0:02.52   | 108.6       | 0:04.85    | 205.1        |
| alignment-html            | 0:27.62   | 3023.9      | 0:41.19   | 3846.0      | 0:54.77    | 3841.8       |
| sub-alignment-html        | 0:00.03   | 40.6        | 0:00.09   | 108.9       | 0:00.24    | 205.7        |
| alignment-convert         | 0:00.32   | 76.1        | 0:01.42   | 205.8       | 0:03.54    | 431.1        |
| padded-sub-alignment-html | 0:00.02   | 40.8        | 0:00.11   | 109.0       | 0:00.18    | 205.9        |
