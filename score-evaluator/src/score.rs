use std::path::PathBuf;

use crate::{ground_truth::JulietGroundTruth, scanners::ScanResults};

pub fn calculate_score(format: ScanResults, results_json: JulietGroundTruth) -> ScoreResults {
    todo!()
}

pub struct ScoreResults {
    
}

impl ScoreResults {
    pub fn write_to(self, file: PathBuf) {
        todo!()
    }
}