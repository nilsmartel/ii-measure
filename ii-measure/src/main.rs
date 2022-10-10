mod util;
use util::*;

mod inverted_index;
mod measure;
mod table_lake;
use log::Logger;
use structopt::StructOpt;
use table_lake::*;

mod algorithm;
mod cli;
mod db;
mod log;

use measure::measure_logging;

use crate::cli::Config;

use jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn basename(s: &str) -> String {
    s.rsplit('/').next().unwrap().to_owned()
}
fn main() {
    let Config {
        bintable,
        algorithm,
        table,
        header,
        header_only,
        mut factor,
    } = cli::Config::from_args();

    if header_only {
        log::print_header();
        std::process::exit(1);
    }

    if factor == Some(1.0) {
        factor = None;
    }

    if let Some(f) = factor {
        let percentage = f * 100.;
        eprintln!(
            "benchmarking {} on {}% {}",
            algorithm.str(),
            percentage,
            table
        );
    } else {
        eprintln!("benchmarking {} on {}", algorithm.str(), table);
    }

    // init information logger
    let log = Logger::new(algorithm.str(), basename(&table), header);

    let receiver = if bintable {
        indices_from_bintable(&table, factor)
    } else {
        indices_sqlx(&table, factor)
    };

    // Select Compression Algorithm and perfom
    use cli::CompressionAlgorithm::*;
    match algorithm {
        Baseline => measure_logging(algorithm::baseline, receiver, log),
        DedupHash => measure_logging(algorithm::dedup_hash, receiver, log),
        DedupBTree => measure_logging(algorithm::dedup_btree, receiver, log),
        NS => measure_logging(algorithm::ns_4_wise, receiver, log),
    }
}
