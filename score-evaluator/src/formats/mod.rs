pub mod sonar;

#[derive(Default)]
pub struct ScanResult {
    pub locations: Vec<ScanLocation>
}

#[derive(Default, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ScanLocation {
    pub file: String,
    pub line: usize,
}
impl ToString for ScanLocation {
    fn to_string(&self) -> String {
        format!("{}:{}", self.file, self.line)
    }
}
