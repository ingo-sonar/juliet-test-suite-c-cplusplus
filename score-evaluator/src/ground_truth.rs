use std::path::PathBuf;
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

use crate::formats::ScanLocation;

pub fn parse_ground_truth(manifest_xml: PathBuf) -> GroundTruth {
    let contents = std::fs::read_to_string(manifest_xml).expect("could not read manifest xml");
    let juliet_ground_truth: JulietGroundTruth = from_str(&contents).expect("could not parse manifest xml");
    juliet_ground_truth.into()
}

#[derive(Default)]
pub struct GroundTruth {
    pub positive_tests: Vec<PositiveTest>,
    pub negative_tests: Vec<NegativeTest>,
}
pub struct PositiveTest {
    pub locations: Vec<ScanLocation>
}

pub struct NegativeTest {
    pub file: String,
}

impl GroundTruth {
    pub fn keep_test_cases(&mut self, test_cases: Vec<String>) {
        self.positive_tests.retain_mut(|test| {
            test.locations.retain(|loc| {
                test_cases.iter().any(|x| loc.file.starts_with(x))
            });
            !test.locations.is_empty()
        });
        self.negative_tests.retain(|test| {
            test_cases.iter().any(|x| test.file.starts_with(x))
        });
    }
}

impl From<JulietGroundTruth> for GroundTruth {
    fn from(value: JulietGroundTruth) -> Self {
        let mut result = GroundTruth::default();
        for test_case in value.test_cases {
            for juliet_file in test_case.files {
                if let Some(flaws) = juliet_file.flaws {
                    let mut locations: Vec<ScanLocation> = vec!();
                    for flaw in flaws {
                        let loc = ScanLocation {
                            file: juliet_file.path.clone(),
                            line: flaw.line.parse().expect("line parse"),
                        };
                        locations.push(loc);
                    }
                    result.positive_tests.push(PositiveTest { locations })
                } else {
                    result.negative_tests.push(NegativeTest { file: juliet_file.path.clone() });
                }
            }
        }
        result
    }
}

/// juliet manifest.xml
/// ```xml
/// <container>
///  <testcase>
///    <file path="CWE114_Process_Control__w32_char_connect_socket_01.c">
///      <flaw line="121" name="CWE-114: Process Control"/>
///    </file>
///  </testcase>
/// </container
/// ```

#[derive(Serialize, Deserialize)]
pub struct JulietGroundTruth {
    #[serde(rename = "testcase")]
    pub test_cases: Vec<JulietTestCase>,
}
impl JulietGroundTruth {
    pub fn keep_test_cases(&mut self, test_cases: Vec<String>) {
        self.test_cases.retain_mut(|test_case| {
            test_case.files.retain(|file| {
                test_cases.iter().any(|name| file.path.starts_with(name))
            });
            !test_case.files.is_empty()
        })
    }

    pub fn write_to(&self, manifest_xml: PathBuf) {
        let str: String = quick_xml::se::to_string(&self).expect("serialize");
        std::fs::write(manifest_xml, str);
    }
}
#[derive(Serialize, Deserialize)]
pub struct JulietTestCase {
    #[serde(rename = "file")]
    pub files: Vec<JulietFile>,
}
#[derive(Serialize, Deserialize)]
pub struct JulietFile {
    pub path: String,
    #[serde[rename = "flaw"]]
    pub flaws: Option<Vec<JulietFlaw>>,
}
#[derive(Serialize, Deserialize)]
pub struct JulietFlaw {
    pub name: String,
    pub line: String,
}
