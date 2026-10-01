use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "phyloEVOLVE",
    version = "1.0",
    about = "phylogenomics made easy for gene and marker alignment
    ************************************************
    Author Gaurav Sablok,
    Email: gsablok@proton.me
    ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// merge all the alignment into a single string.
    Alignmerge {
        /// provide the path to the alignment file
        alignment: String,
        /// provide the header to the alignment
        mergeheader: String,
        /// threads for the analysis
        thread: String,
    },
    /// alignmerge the specific region of the alignment
    Alignmergeinterval {
        /// provide the path to the alignment file
        alignment: String,
        /// provide the start range
        start: String,
        /// provide the end range
        end: String,
        /// provide the header for the merge alignment
        header: String,
        /// threads for the analysis
        thread: String,
    },
    /// remove same alignment
    SameAlignment {
        /// path to the alignment file
        alignment: String,
        /// threads for the analysis
        thread: String,
    },
    /// alignmentstats
    Alignmentstats {
        /// path  to the alignment file
        alignment: String,
        /// threads for the analysis
        thread: String,
    },
    /// filter the sites with the given base
    FilterSite {
        /// provide the path the alignment file
        alignment: String,
        /// provide the base that needs to be removed from the alignment
        base: String,
        /// threads for the analysis
        thread: String,
    },
    /// removes same bases across all the alignment
    FilterAll {
        /// provide the path to the alignment file
        alignment: String,
        /// threads for the analysis
        thread: String,
    },
    /// collineratiy block based alignment filtering
    FilterBlock {
        /// path to the alignment file
        alignment: String,
        /// provide the block size
        block: String,
        /// threads for the analysis
        thread: String,
    },
    /// allows for the view of the alignment
    AlignmentView {
        /// path to the alignment file
        alignment: String,
        /// threads for the analysis
        thread: String,
    },
    /// allows for the embedded view of the clipped Alignment
    AlignmentClipview {
        /// path to the alignment file
        alignment: String,
        /// start of the alignment
        start: usize,
        /// end of the alignment
        end: usize,
        /// threads for the analysis
        thread: String,
    },
    /// replace the specific sites based on the site probability
    Sitereplace {
        /// path to the alignment file
        alignment: String,
        /// path to the letter that you want to replace
        letter: String,
        /// path ot the replacement
        replacement: String,
        /// threads for the analysis
        thread: String,
    },
    /// estimate the protein stats
    ProteinStat {
        /// path to the protein file
        alignment: String,
        /// threads for the analysis
        thread: String,
    },
    /// indel substituter
    Indelreplace {
        /// path to the alignment file
        alignment: String,
        /// base sustitute for the indel
        indel: String,
        /// threads for the analysis
        thread: String,
    },
    /// search for the specific protein and the nucleotide motifs
    MotifSearch {
        /// path to the alignment file
        alignment: String,
        /// motif for the search
        motif: String,
        /// threads for the analysis
        thread: String,
    },
    /// specific part of the alignment and the upstream and the downstream of alignment
    UpDownStream {
        /// path to the alignment
        alignment: String,
        /// focal site
        focal: String,
        /// upstream alignment
        upstream: String,
        /// downstream alignment
        downstream: String,
        /// threads for the analysis
        thread: String,
    },
    /// extract the places of the specified base for LD analysis
    SiteAlignment {
        /// path to the alignment file
        alignment: String,
        /// path to the header
        header: String,
        /// specific base
        base: String,
        /// threads for the analysis
        thread: String,
    },
    /// frequency plot for the alignments
    Plotter {
        /// path to the alignment file
        alignment: String,
        /// threads for the analysis
        thread: String,
    },
    /// color coded map
    Nucleotidecolour {
        /// path to the alignment file
        alignment: String,
        /// threads for the analysis
        thread: String,
    },
    /// proteome heatmap
    Proteomecolour {
        /// path to the protein alignment
        alignment: String,
        /// threads for the analysis
        thread: String,
    },
    /// alignment to html
    AlignmentHtml {
        /// input path to the fasta
        inputfasta: String,
        /// output path to the html
        outputpath: String,
        /// threads for the analysis
        thread: String,
    },
    /// sub alignment to html
    SubAlignmentHtml {
        /// input path to the fasta
        inputfasta: String,
        /// start of the alignment
        start: String,
        /// end of the alignment
        end: String,
        /// prefix for the alignment
        prefix: String,
        /// threads for the analysis
        thread: String,
    },
    /// convert alignment
    AlignmentConvert {
        /// path to the input fasta
        input: String,
        /// threads for the analysis
        thread: String,
    },
    /// padded sub alignment to html
    PaddedSubAlignmentHtml {
        /// input path to the fasta
        inputfasta: String,
        /// start of the alignment
        start: String,
        /// end of the alignment
        end: String,
        /// prefix for the alignment
        prefix: String,
        /// threads for the analysis
        thread: String,
    },
}
