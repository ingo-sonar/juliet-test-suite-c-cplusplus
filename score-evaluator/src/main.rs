use std::path::PathBuf;

use clap::Parser;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TerminalMode, TermLogger};

use formats::sonar::parse_sonar_results;
use ground_truth::parse_ground_truth;
use score::calculate_score;
use crate::formats::sarif::parse_sarif_results;

mod formats;
mod ground_truth;
mod score;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the results json file that the scanner produced.
    #[arg(short, long)]
    results_json: PathBuf,

    /// Results format (valid values: sonarqube)
    #[arg(short, long)]
    format: String,

    /// Active test cases, e.g. "CWE114_Process_Control" (omit to activate all test cases).
    #[arg(short, long)]
    test_cases: Vec<String>,

    /// Path to the "manifest.xml" file of the juliet test suite, which defines the ground truth.
    #[arg(short, long)]
    manifest_xml: PathBuf,

    /// Path to the "output.json" file to generate.
    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)
        .expect("could not initialize logger");

    let args = Args::parse();
    let mut ground_truth = parse_ground_truth(args.manifest_xml);
    log::info!("Loaded ground truth with {} tests.", ground_truth.positive_tests.len() + ground_truth.negative_tests.len());

    if !args.test_cases.is_empty() {
        ground_truth.keep_test_cases(args.test_cases);
        log::info!("Filtered ground truth has {} tests.", ground_truth.positive_tests.len() + ground_truth.negative_tests.len());
    }

    let results = match &args.format as &str {
        "sonarqube" => parse_sonar_results(args.results_json),
        "sarif" => parse_sarif_results(args.results_json),
        _ => panic!("invalid value for 'format'"),
    };
    log::info!("Loaded {} results.", results.locations.len());

    let score = calculate_score(ground_truth, results);
    score.log();
    score.write_to(args.output);
}

