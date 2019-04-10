use structopt::StructOpt;
use bio::io::fasta;
use statrs::statistics::Statistics;
use std::str;
use std::process;

/// Filter out seqs with ambiguous bases and by length.
#[derive(StructOpt, Debug)]
pub struct CliArgs {
    /// The path to the fasta file
    #[structopt(parse(from_os_str))]
    infile: std::path::PathBuf,
}

#[derive(Debug)]
struct ReadLengthStats {
    mean: f64,
    sd: f64,
    min_allowed: f64,
    max_allowed: f64,
}

impl ReadLengthStats {
    fn get_read_lengths(path: &std::path::PathBuf) -> Vec<f64> {
        // Obtain reader or fail with error.
        let reader = fasta::Reader::from_file(path).unwrap_or_else(|err| {
            eprintln!("ERROR -- problem opening file {:?}: {}", path, err);
            process::exit(1);
        });

        let mut read_lengths = Vec::new();

        // First time through we need to get the mean and standard deviation of the read length.
        for item in reader.records() {
            // Obtain record or fail with error
            let record = item.unwrap_or_else(|err| {
                eprintln!("ERROR -- problem getting record from fasta file: {}", err);
                process::exit(1);
            });

            // mean is impl on [f64]
            read_lengths.push(record.seq().len() as f64);
        }

        if read_lengths.is_empty() {
            eprintln!("ERROR -- found no reads in file {:?}", path);
            process::exit(1);
        }

        read_lengths
    }

    fn get_read_length_stats(lengths: &[f64]) -> ReadLengthStats {
        let mean = lengths.mean();
        let sd = lengths.std_dev();
        let min_allowed = mean - (2.0 * sd);
        let max_allowed = mean + (2.0 * sd);

        ReadLengthStats {
            mean,
            sd,
            min_allowed,
            max_allowed,
        }
    }

    fn new(path: &std::path::PathBuf) -> ReadLengthStats {
        let read_lengths = ReadLengthStats::get_read_lengths(path);

        ReadLengthStats::get_read_length_stats(&read_lengths)
    }
}

fn has_ambiguous_bases(seq: bio::utils::TextSlice) -> bool {
    for &base in seq.iter() {
        if base == b'N' || base == b'n' {
            return true;
        }
    }

    false
}

fn format_record(rec: &fasta::Record) -> std::string::String {
    let id = rec.id();
    let seq = str::from_utf8(rec.seq()).unwrap();

    match rec.desc() {
        Some(desc) => format!(">{} {}\n{}", id, desc, seq),
        None => format!(">{}\n{}", id, seq),
    }
}

fn print_record(rec: &fasta::Record) {
    let str = format_record(rec);

    println!("{}", str);
}

fn seq_is_good(rec: &fasta::Record, len_stats: &ReadLengthStats) -> bool {
    let seq = rec.seq();
    let len = seq.len() as f64;

    len_stats.min_allowed <= len && len <= len_stats.max_allowed && !has_ambiguous_bases(seq)
}

pub fn run(config: CliArgs) {
    let read_len_stats = ReadLengthStats::new(&config.infile);

    eprintln!("DEBUG -- {:?}", read_len_stats);

    let reader = fasta::Reader::from_file(&config.infile).unwrap();

    for item in reader.records() {
        // Obtain record or fail with error
        let rec = item.unwrap();

        if seq_is_good(&rec, &read_len_stats) {
            print_record(&rec);
        }
    }
}
