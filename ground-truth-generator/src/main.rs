use clap::Parser;
use log::LevelFilter;
use score_evaluator::ground_truth::{JulietFile, JulietFlaw, JulietGroundTruth, JulietTestCase};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::collections::HashMap;
use std::path::PathBuf;
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
 
    #[arg(long)]
    analysis_type: TAType,
}
 
fn main() {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("could not initialize logger");
 
    let args = Args::parse();
    if args.stats {
        print_stats();
    }
    let ground_truth = generate_ground_truth(args.analysis_type);
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
                let dir = entry
                    .path()
                    .to_str()
                    .expect("path")
                    .strip_prefix("../testcases")
                    .expect("x")
                    .strip_prefix(std::path::MAIN_SEPARATOR)
                    .expect("prefix")
                    .split(std::path::MAIN_SEPARATOR)
                    .into_iter()
                    .next()
                    .expect("")
                    .to_string();
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
 
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum TAType {
    UseAfterFree = 0,
    DoubleFree = 1,
}
 
impl TAType {
    fn strings(&self) -> &'static [(&'static str, usize)] {
        match self {
            TAType::DoubleFree => &[
                ("POTENTIAL FLAW: Possibly freeing memory twice", 1),
                ("POTENTIAL FLAW: Possibly deleting memory twice", 1),
            ],
            TAType::UseAfterFree => &[
                ("POTENTIAL FLAW: Use of data that may have been freed", 1),
                ("POTENTIAL FLAW: Use of data that may have been deleted", 1),
                (
                    "FLAW: Freeing a memory block and then returning a pointer to the freed memory",
                    2,
                ),
            ],
        }
    }
    fn source_path(&self) -> &'static str {
        match self {
            TAType::UseAfterFree => "../testcases/CWE416_Use_After_Free",
            TAType::DoubleFree => "../testcases/CWE415_Double_Free",
        }
    }
    fn name(&self) -> &'static str {
        match self {
            TAType::UseAfterFree => "CWE-416: Use After Free",
            TAType::DoubleFree => "CWE-415: Double Free",
        }
    }
}
 
fn generate_ground_truth(tpe: TAType) -> JulietGroundTruth {
    let mut test_cases: Vec<JulietTestCase> = vec![];
 
    for entry in WalkDir::new(tpe.source_path())
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
 
                let contains_line = tpe
                    .strings()
                    .iter()
                    .filter_map(|(x, offset)| {
                        if line.contains(*x) {
                            Some(*offset)
                        } else {
                            None
                        }
                    })
                    .next();
                if let Some(offset) = contains_line {
                    
                    let mut comment_len = 0;
                    while !lines[num].ends_with("*/") {
                        num += 1;
                        comment_len += 1;
                        if comment_len > 3 { panic!("check long comment in line {}", num + 1); }
                    }
                    
                    let reported_line = num + 1 + offset; //  line numbers start at 1, and add specific offset (e.g. report next line)
                    test_cases.push(JulietTestCase {
                        files: vec![JulietFile {
                            path: file.to_string(),
                            flaws: Some(vec![JulietFlaw {
                                name: tpe.name().to_string(),
                                line: reported_line.to_string(),
                            }]),
                        }],
                    })
                }
            }
        }
    }
 
    JulietGroundTruth { test_cases }
}
 