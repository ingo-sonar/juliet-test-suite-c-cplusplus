use std::path::PathBuf;

use serde::Deserialize;

use super::{ScanLocation, ScanResult};

pub fn parse_sarif_results(input: PathBuf) -> ScanResult {
    let contents: String = std::fs::read_to_string(input).expect("invalid file");
    let multi_sarif: Result<Vec<Sarif>, _> = serde_json::from_str(&contents);
    let multi_sarif: Vec<Sarif> = match multi_sarif {
        Ok(multi_sarif) => multi_sarif,
        Err(_) => vec!(serde_json::from_str(&contents).expect("invalid json"))
    };
    let mut results = ScanResult::default();
    for sarif in multi_sarif {
        for run in sarif.runs {
            for result in run.results {
                for location in result.locations {
                    let file = location.physical_location.artifact_location.uri;
                    let line = location.physical_location.region.start_line as usize;
                    results.locations.push(ScanLocation {
                        file: file.split('/').last().expect("prefix").to_string(),
                        line
                    })
                }
            }
        }
    }
    results
}

#[derive(Deserialize)]
struct Sarif {
    runs: Vec<SarufRun>
}

#[derive(Deserialize)]
struct SarufRun {
    results: Vec<SarifResult>
}

#[derive(Deserialize)]
struct SarifResult {
    level: String,
    locations: Vec<SarifLocation>,
    message: SarifText,
    #[serde(rename = "ruleId")]
    rule_id: String,
}

#[derive(Deserialize)]
struct SarifText {
    text: String,
}

#[derive(Deserialize)]
struct SarifLocation {
    #[serde(rename = "physicalLocation")]
    physical_location: PhysicalLocation
}

#[derive(Deserialize)]
struct PhysicalLocation {
    #[serde(rename = "artifactLocation")]
    artifact_location: ArtifactLocation,
    region: Region
}

#[derive(Deserialize)]
struct ArtifactLocation {
    uri: String
}

#[derive(Deserialize)]
struct Region {
    #[serde(rename = "startColumn")]
    start_column: i32,
    #[serde(rename = "startLine")]
    start_line: i32,
}


