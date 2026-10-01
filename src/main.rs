mod align;
mod alignmentplot;
mod alignmentstat;
mod alignmerge;
mod args;
mod astruct;
mod colorcoded;
mod filesearch;
mod filterblock;
mod filtersame;
mod filtersite;
mod indelreplace;
mod motifsearch;
mod proteincolourcounter;
mod proteinstat;
mod samealignment;
mod sitealignment;
mod sitereplace;
mod updownstream;
mod view;
mod viewspliced;
use crate::align::alignmerge;
use crate::alignmentplot::plotter;
use crate::alignmentstat::alignmentstats;
use crate::alignmerge::alignmergeall;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::colorcoded::readcodecolor;
use crate::filterblock::filterblockalignment;
use crate::filtersame::filtersiteall;
use crate::filtersite::filtersiteremoval;
use crate::indelreplace::substitute;
use crate::motifsearch::motifsearchall;
use crate::proteincolourcounter::proteomecolor;
use crate::proteinstat::proteomestats;
use crate::samealignment::dealignment;
use crate::sitealignment::sitespecific;
use crate::sitereplace::sitereplacenuc;
use crate::updownstream::upstreamdownstream;
use crate::view::alignment_embedded_common;
use crate::viewspliced::splicedalignment;
use clap::Parser;
mod alignhtml;
use alignhtml::alignmenthtml;
mod subseq;
use crate::subseq::alignsubseq;
mod fastaconvert;
use crate::fastaconvert::convertfasta;
mod padded;
use crate::padded::alignpadded;
use vornix_banner::{rgb, Banner, BuiltinFont, Style};

/*
Gaurav Sablok,
gsablok@proton.me
*/
fn main() {
    let style = Style::new().fg(rgb(255, 100, 20)).bold();

    let mut banner = Banner::new("phyevkit")
        .with_builtin_font(BuiltinFont::Block)
        .with_style(style)
        .centered(true);

    banner.display().unwrap();

    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::Alignmerge {
            alignment,
            mergeheader,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = alignmerge(alignment, mergeheader).unwrap();
                println!("The merged alignment have been written: {:?}", command);
            });
        }
        Commands::Alignmergeinterval {
            alignment,
            start,
            end,
            header,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = alignmergeall(alignment, start, end, header).unwrap();
                println!(
                    "The values after applying the score filter are:{:?}",
                    command
                );
            })
        }
        Commands::SameAlignment { alignment, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = dealignment(alignment).unwrap();
                println!("The file has been written:{:?}", command);
            });
        }
        Commands::Alignmentstats { alignment, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = alignmentstats(alignment).unwrap();
                println!("The alignment stats for the given file is: {:?}", command);
            });
        }
        Commands::FilterSite {
            alignment,
            base,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = filtersiteremoval(alignment, base).unwrap();
                println!(
                    "The results after the filtering have been written:{:?}",
                    command
                );
            });
        }
        Commands::FilterAll { alignment, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = filtersiteall(alignment).unwrap();
                println!("The filtered bases have been written, {:?}", command);
            });
        }
        Commands::FilterBlock {
            alignment,
            block,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = filterblockalignment(alignment, block).unwrap();
                println!("The alignment block has been filtered, {:?}", command);
            });
        }
        Commands::AlignmentView { alignment, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = alignment_embedded_common(alignment);
                println!(
                    "The printed alignment for the give alignment is as follows: {:?}",
                    command
                );
            });
        }
        Commands::AlignmentClipview {
            alignment,
            start,
            end,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = splicedalignment(alignment, *start, *end);
                println!(
                    "The printed alignment for the specific region are:{:?}",
                    command
                );
            });
        }
        Commands::Sitereplace {
            alignment,
            letter,
            replacement,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = sitereplacenuc(alignment, letter, replacement).unwrap();
                println!("The site have been repalced:{:?}", command)
            });
        }
        Commands::ProteinStat { alignment, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = proteomestats(alignment).unwrap();
                println!("The stats have been written for the file: {:?}", command);
            });
        }
        Commands::Indelreplace {
            alignment,
            indel,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = substitute(alignment, indel).unwrap();
                println!("The indel replacement have been written: {:?}", command);
            });
        }
        Commands::MotifSearch {
            alignment,
            motif,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = motifsearchall(alignment, motif).unwrap();
                println!("The search motifs has been listed:{:?}", command);
            });
        }
        Commands::UpDownStream {
            alignment,
            focal,
            upstream,
            downstream,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = upstreamdownstream(alignment, focal, upstream, downstream).unwrap();
                println!(
                    "The upstream downstream regions from the alignment have been written {:?}",
                    command
                );
            });
        }
        Commands::SiteAlignment {
            alignment,
            header,
            base,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = sitespecific(alignment, header, base).unwrap();
                println!(
             "The specific site information from all those alignments have been written:{:?}",
             command
         );
            });
        }
        Commands::Plotter { alignment, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = plotter(alignment).unwrap();
                println!("The alignment plots have been written:{:?}", command);
            });
        }
        Commands::Nucleotidecolour { alignment, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = readcodecolor(alignment).unwrap();
                println!("The colour coded alignment are as follows:{:?}", command);
            });
        }
        Commands::Proteomecolour { alignment, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = proteomecolor(alignment).unwrap();
                println!(
                    "The color coded protein alignment protein table: {:?}",
                    command
                );
            })
        }
        Commands::AlignmentHtml {
            inputfasta,
            outputpath,
            thread,
        } => {
            let value = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .expect("file failed");
            value.install(|| {
                let _ = alignmenthtml(inputfasta, outputpath);
                println!("The command has finished");
            })
        }
        Commands::SubAlignmentHtml {
            inputfasta,
            start,
            end,
            prefix,
            thread,
        } => {
            let value = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .expect("file failed");
            value.install(|| {
                let _ = alignsubseq(inputfasta, start, end, prefix);
                println!("The command has finished")
            });
        }
        Commands::AlignmentConvert { input, thread } => {
            let value = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .expect("file failed");
            value.install(|| {
                let _ = convertfasta(input);
                println!("The command has finished")
            });
        }
        Commands::PaddedSubAlignmentHtml {
            inputfasta,
            start,
            end,
            prefix,
            thread,
        } => {
            let value = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .expect("file failed");
            value.install(|| {
                let _ = alignpadded(inputfasta, start, end, prefix);
                println!("The command has finished")
            });
        }
    }
}
