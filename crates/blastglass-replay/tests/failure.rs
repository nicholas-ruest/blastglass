use blastglass_persistence::EvidenceStore;
use blastglass_replay::replay;
#[test]
fn missing_rehearsal_fails() {
    let p = std::env::temp_dir().join(format!("blastglass-none-{}.jsonl", std::process::id()));
    let s = EvidenceStore::open(&p).unwrap();
    assert!(replay(&s, "missing").is_err());
    let _ = std::fs::remove_file(p);
}
