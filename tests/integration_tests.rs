use petta::MettaEngine;

#[test]
fn test_basic_metta_loading() {
    let mut engine = MettaEngine::new();
    let _ = engine.run_metta_file("examples/hello.metta");
}
