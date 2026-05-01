//! Advanced PeTTa usage examples
//!
//! This file demonstrates advanced usage patterns for the PeTTa runtime.

use petta::{PeTTaEngine, EngineConfig};
use std::path::Path;

/// Example: Batch processing
pub fn batch_processing_example() {
 let queries = vec![
  "!(+ 1 2)",
  "!(+ 2 2)",
  "!(+ 3 2)",
 ];
 
 let config = EngineConfig::new(Path::new("."));
 let mut engine = PeTTaEngine::with_config(&config).unwrap();
 
 for query in queries {
  match engine.eval(query) {
   Ok(result) => println!("{} = {}", query, result),
   Err(e) => {
    eprintln!("Error executing {}: {}", query, e);
    // Continue with next query
   }
  }
 }
}

fn main() {
 println!("Advanced usage examples are available as tests.");
 println!("Run with: cargo test --example advanced_usage");
 
 // Or run the batch processing example
 batch_processing_example();
}

#[cfg(test)]
mod advanced_examples {
 use super::*;

 /// Example: Using multiple backends
 #[test]
 #[ignore] // Requires backend setup
 fn example_multiple_backends() {
  // SWI-Prolog backend
  let swipl_config = EngineConfig::builder()
   .verbose(false)
   .build();
  let mut swipl_engine = PeTTaEngine::with_config(&swipl_config).unwrap();
  let result1 = swipl_engine.eval("!(+ 1 2)").unwrap();
  assert_eq!(result1, "3");

  // MORK backend (if available)
  #[cfg(feature = "mork")]
  {
   let mork_config = EngineConfig::builder()
    .verbose(false)
    .build();
   let mut mork_engine = PeTTaEngine::with_config(&mork_config).unwrap();
   let result2 = mork_engine.eval("!(+ 2 2)").unwrap();
   assert_eq!(result2, "4");
  }
 }

 /// Example: Error handling
 #[test]
 fn example_error_handling() {
  let config = EngineConfig::builder().verbose(false).build();
  let mut engine = PeTTaEngine::with_config(&config).unwrap();

  // Handle errors gracefully
  match engine.eval("!(undefined-function)") {
   Ok(_) => panic!("Should have failed"),
   Err(e) => {
    eprintln!("Expected error: {}", e);
   }
  }
 }

 /// Example: Batch processing
 #[test]
 fn example_batch_processing() {
  let config = EngineConfig::builder().verbose(false).build();
  let mut engine = PeTTaEngine::with_config(&config).unwrap();

  let queries = vec!["!(+ 1 2)", "!(+ 2 2)", "!(+ 3 2)"];
  for query in queries {
   let result = engine.eval(query).unwrap();
   assert!(!result.is_empty());
  }
 }
}
