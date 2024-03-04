use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use clap::Parser;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TerminalMode, TermLogger};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Phasar folder to operate in; here there are .bc files for each test case
    #[arg(short, long)]
    folder: PathBuf,
}

fn main() {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)
        .expect("could not initialize logger");

    let args = Args::parse();
    run(args.folder);
}



fn run(folder: PathBuf) {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let read_dir = std::fs::read_dir(&folder).expect("directory?");
    for entry in   read_dir.into_iter().filter_map(|e| e.ok()) {
        let entry = entry.file_name();
        let file_name = entry.to_str().expect("file name?");
        if let Some(base_name) = file_name.strip_suffix(".bc") {
            let pos = base_name.rfind(|x: char| x.is_numeric()).expect("no number?");
            let group_name: String = base_name.chars().take(pos + 1).collect();
            map.entry(group_name).or_default().push(file_name.to_string());
        }
    }

    for (key, values) in map {
        if values.len() > 1 {
            log::info!("linking {:?} into {key}.bc", values);
            let status = Command::new("llvm-link-14")
                .arg("-o")
                .arg(format!("{key}.bc"))
                .args(&values).spawn().expect("spawn?").wait().expect("could not link?");
            assert!(status.success(), "could not link?");
            for value in values {
                std::fs::remove_file(folder.join(value)).expect("delete?");
            }
        }
    }
}