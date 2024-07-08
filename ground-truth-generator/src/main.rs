use std::collections::HashMap;
use std::path::PathBuf;
use clap::Parser;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TerminalMode, TermLogger};
use score_evaluator::ground_truth::{JulietFile, JulietFlaw, JulietGroundTruth, JulietTestCase};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Stats about the benchmark
    #[arg(short, long)]
    stats: bool,

    /// Generated Ground Truth manifest.xml
    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)
        .expect("could not initialize logger");

    let args = Args::parse();
    if args.stats {
        print_stats();
    }
    let ground_truth = generate_ground_truth();
    ground_truth.write_to(args.output);
}

fn print_stats() {
    let mut found: HashMap<String, usize> = HashMap::new();
    let mut total = 0usize;
    for entry in WalkDir::new("../testcases")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file = entry.file_name().to_string_lossy();
        if file.ends_with(".c") || file.ends_with(".cpp") {
            let text = std::fs::read_to_string(entry.path()).expect("read");
            if text.contains("int main(") {
                let dir = entry.path().to_str().expect("path").strip_prefix("../testcases").expect("x")
                    .strip_prefix(std::path::MAIN_SEPARATOR).expect("prefix")
                    .split(std::path::MAIN_SEPARATOR).into_iter().next().expect("").to_string();
                *found.entry(dir).or_default() += 1;
                total += 1;
            }
        }
    }

    log::info!("found total of {} tests", total);
    for (dir, num) in found {
        log::info!("in {}: {} tests", dir, num);
    }
}

fn generate_ground_truth() -> JulietGroundTruth {

    let mut test_cases: Vec<JulietTestCase> = vec!();

    for entry in WalkDir::new("../testcases/CWE476_NULL_Pointer_Dereference")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file = entry.file_name().to_string_lossy();

        if file.ends_with(".c") || file.ends_with(".cpp") {
            let text = std::fs::read_to_string(entry.path()).expect("read");
            let lines: Vec<&str> = text.lines().collect();
            for (mut num, line) in lines.iter().enumerate() {
                if line.contains("#ifndef OMITGOOD") {
                    // We assume that the bad tests always come before the good tests.
                    // This seems to hold as the test cases are auto-generated.
                    break;
                }
                if line.contains("FLAW:") {
                    let mut comment_len = 0;
                    while !lines[num].ends_with("*/") {
                        num += 1;
                        comment_len += 1;
                        if comment_len > 3 { panic!("check long comment?!"); }
                    }
                    
                    let mut reported_line = num + 2; // +1 (Report next line) +1 (line numbers start at 1)
                    
                    test_cases.push(JulietTestCase {
                        files: vec!(
                            JulietFile {
                                path: file.to_string(),
                                flaws: Some(vec!(
                                    JulietFlaw {
                                        name: "CWE476: NULL Pointer Dereference".to_string(),
                                        line: reported_line.to_string(),
                                    }
                                ))
                            }
                        )
                    })
                }
            }
        }
    }

    JulietGroundTruth { test_cases }
}