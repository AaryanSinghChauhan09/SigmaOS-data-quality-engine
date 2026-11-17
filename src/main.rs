//! SigmaOS-data-quality-engine — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_data_quality_engine_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-data-quality-engine: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-data-quality-engine: ok");
}
