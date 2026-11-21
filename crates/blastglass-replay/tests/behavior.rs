use blastglass_evidence::seal;
use blastglass_persistence::EvidenceStore;
use blastglass_replay::replay;
#[test]
fn replays_saved_event() {
    let p = std::env::temp_dir().join(format!("blastglass-replay-{}.jsonl", std::process::id()));
    let s = EvidenceStore::open(&p).unwrap();
    s.append(&seal("1", "c", "done", "genesis").unwrap())
        .unwrap();
    assert_eq!(replay(&s, "c").unwrap().events, 1);
    let _ = std::fs::remove_file(p);
}
