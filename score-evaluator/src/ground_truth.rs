use std::path::PathBuf;

pub fn parse_ground_truth(manifest_xml: PathBuf) -> JulietGroundTruth {
    todo!()
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
pub struct JulietGroundTruth {
    pub container: JulietContainer
}
pub struct JulietContainer {
    pub test_case: Vec<JulietTestCase>,
}
pub struct JulietTestCase {
    pub file: Vec<JulietFile>,
}
pub struct JulietFile {
    pub path: String,
    pub flaw: Vec<JulietFlaw>,
}
pub struct JulietFlaw {
    pub name: String,
    pub line: String,
}
