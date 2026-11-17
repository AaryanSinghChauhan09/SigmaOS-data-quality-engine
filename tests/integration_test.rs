//! Integration tests for SigmaOS-data-quality-engine.

use sigmaos_data_quality_engine_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
