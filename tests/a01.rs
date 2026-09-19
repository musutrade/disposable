#[test]
fn a01_marker() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("a01-smoke.txt");
    let bytes = std::fs::read(path).expect("read a01-smoke.txt");
    assert_eq!(bytes, b"CodexSymphony real A01\n");
}
