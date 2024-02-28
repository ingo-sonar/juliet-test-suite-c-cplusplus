mod scanners;
mod ground_truth;
mod score;

use std::path::PathBuf;
use clap::Parser;
use ground_truth::{parse_ground_truth, JulietGroundTruth};
use scanners::{sonar::parse_sonar_results, ScanResults};
use score::calculate_score;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the results json file that the scanner produced.
    #[arg(short, long)]
    results_json: PathBuf,

    /// Results format (valid values: sonarqube)
    #[arg(short, long)]
    format: String,
    
    /// Path to the "manifest.xml" file of the juliet test suite, which defines the ground truth.
    #[arg(short, long)]
    manifest_xml: PathBuf,

    /// Path to the "output.json" file to generate.
    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    let args = Args::parse();
    let results = match &args.format as &str {
        "sonarqube" => parse_sonar_results(args.results_json),
        _ => panic!("invalid value for 'format'"),
    };
    let ground_truth = parse_ground_truth(args.manifest_xml);
    let score = calculate_score(results, ground_truth);
    score.write_to(args.output);
}

