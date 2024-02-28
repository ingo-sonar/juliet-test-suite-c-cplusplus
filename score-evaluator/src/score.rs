use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use serde::Serialize;

use crate::{formats::ScanResult, ground_truth::JulietGroundTruth};
use crate::formats::ScanLocation;
use crate::ground_truth::GroundTruth;

pub fn calculate_score(ground_truth: GroundTruth, scan_results: ScanResult) -> ScoreResults {
    let required = {
        let mut required: HashSet<ScanLocation> = HashSet::new();
        let mut disallowed: HashSet<String> = HashSet::new();
        for test in ground_truth.positive_tests {
            required.extend(test.locations);
        }
        for test in ground_truth.negative_tests {
            disallowed.insert(test.file);
        }

        // sanity check
        for req in required.iter().map(|x| &x.file) {
            assert!(!disallowed.contains(req));
        }

        required
    };

    let found: HashSet<ScanLocation> = scan_results.locations.into_iter().collect();
    let true_positives: HashSet<ScanLocation> = found.intersection(&required).cloned().collect();
    let false_positives: HashSet<ScanLocation> = found.difference(&required).cloned().collect();
    let false_negatives: HashSet<ScanLocation> = required.difference(&found).cloned().collect();

    let mut true_positives: Vec<String> = true_positives.into_iter().map(|x| x.to_string()).collect();
    let mut false_positives: Vec<String> = false_positives.into_iter().map(|x| x.to_string()).collect();
    let mut false_negatives: Vec<String> = false_negatives.into_iter().map(|x| x.to_string()).collect();

    true_positives.sort();
    false_positives.sort();
    false_negatives.sort();

    let precision = true_positives.len() as f32 / (true_positives.len() as f32 + false_positives.len() as f32);
    let recall = true_positives.len() as f32 / (true_positives.len() as f32 + false_negatives.len() as f32);
    let f1_rate = 2.0 * precision * recall / (precision + recall);

    ScoreResults {
        metrics: Metrics {
            num_true_positives: true_positives.len(),
            num_false_positives: false_positives.len(),
            num_false_negatives: false_negatives.len(),
            precision,
            recall,
            f1_rate,
        },
        true_positives,
        false_positives,
        false_negatives,
    }
}

#[derive(Debug, Serialize)]
pub struct ScoreResults {
    metrics: Metrics,
    true_positives: Vec<String>,
    false_positives: Vec<String>,
    false_negatives: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Metrics {
    num_true_positives: usize,
    num_false_positives: usize,
    num_false_negatives: usize,
    precision: f32,
    recall: f32,
    f1_rate: f32,
}

impl ScoreResults {
    pub fn log(&self) {
        log::info!("Results: {:#?}", self.metrics);
    }
    pub fn write_to(&self, file: PathBuf) {
        let str = serde_json::to_string(&self).expect("could not serialize results");
        std::fs::write(file, str).expect("could not write results");
    }
}