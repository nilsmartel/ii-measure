mod util;
use util::*;

mod inverted_index;
mod kinds;
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
        database,
        algorithm,
        table,
        header,
        header_only,
        mut factor,
        multi_proc,
    } = cli::Config::from_args();

    if header_only {
        log::print_header();
        std::process::exit(1);
    }

    if factor == Some(1.0) {
        factor = None;
    }

    if let Some(f) = factor {
        if f < 0.00001 {
            eprintln!("to small");
            std::process::exit(0);
        }

        if f >= 0.5
            && &table == "main_tokenized"
            && (algorithm == Baseline || algorithm == Smaz || algorithm == NS)
        {
            eprintln!(
                "using {} on more than 50% of corpus {table} is not expected to work",
                algorithm.str()
            );
            eprintln!("exiting, because this will crash anyway");
            std::process::exit(0);
        }

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
    let log = Logger::new(
        algorithm.str().to_string(),
        basename(&table),
        header,
        multi_proc,
    );

    let receiver = if database {
        indices_sqlx(&table, factor)
    } else {
        indices_from_bintable(&table, factor)
    };

    // Select Compression Algorithm and perfom
    use kinds::CompressionAlgorithm::*;
    match algorithm {
        Baseline => measure_logging(algorithm::baseline, receiver, log),
        DedupHash => measure_logging(algorithm::dedup_hash, receiver, log),
        DedupBTree => measure_logging(algorithm::dedup_btree, receiver, log),

        NSDedup => measure_logging(algorithm::ns_4_wise, receiver, log),
        NS => measure_logging(algorithm::ns_raw, receiver, log),
        NSArena => measure_logging(algorithm::ns_arena::ns_arena, receiver, log),

        SmazDedup => measure_logging(algorithm::smaz, receiver, log),
        Smaz => measure_logging(algorithm::smaz_raw, receiver, log),

        SmazNSDedup => measure_logging(algorithm::smaz_ns, receiver, log),

        // FastPforDedup => measure_logging(algorithm::pfor::pfor, receiver, log),
        FastPforExactDedup => measure_logging(algorithm::pfor_x::pfor, receiver, log),

        _ => panic!("algorithm {} not yet implemented", algorithm.str()),
    }
}
