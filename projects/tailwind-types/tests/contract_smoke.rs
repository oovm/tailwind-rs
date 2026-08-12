use tailwind_types::{CandidateInput, CompileRequest, SourceRef};

#[test]
fn contract_roundtrips_through_json() {
    let request = CompileRequest {
        candidates: vec![CandidateInput::new("block", SourceRef::new("opaque-ref"))],
        ..Default::default()
    };
    let json = serde_json::to_string_pretty(&request).expect("serialize");
    assert!(!json.contains("html"));
    assert!(!json.contains("vmz"));
    assert!(!json.contains("doki"));
    assert!(!json.contains("designs"));
    let back: CompileRequest = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.candidates[0].token, "block");
    assert_eq!(back.candidates[0].source.as_str(), "opaque-ref");
}
