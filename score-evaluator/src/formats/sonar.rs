use std::collections::HashSet;
use std::path::PathBuf;

use serde::Deserialize;

use super::{ScanLocation, ScanResult};

const CWE415: bool = true;
const CWE416: bool = false;

pub fn parse_sonar_results(input: PathBuf) -> ScanResult {
    let contents: String = std::fs::read_to_string(input).expect("invalid file");
    let sonar_issues: Vec<Vec<SonarIssue>> = serde_json::from_str(&contents).expect("invalid json");
    let mut results = ScanResult::default();
    for issue in sonar_issues.into_iter().flatten() {
        if CWE416 {
            if !issue.message.contains("Use of memory after it is freed") { continue; }
        }
        if CWE415 {
            if !issue.message.contains("Attempt to free released memory") { continue; }
        }
        if !issue.tags.contains("cwe") { continue; }
        let location = ScanLocation {
            file: issue.component.split('/').last().expect("prefix").to_string(),
            line: issue.line,
        };
        results.locations.push(location);
    }
    results
}

#[derive(Deserialize)]
struct SonarIssue {
    line: usize,
    component: String,
    severity: String,
    tags: HashSet<String>,
    message: String,
}