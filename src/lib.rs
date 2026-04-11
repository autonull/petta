pub mod vector_space;

use scryer_prolog::{Machine, MachineBuilder};

pub struct MettaEngine {
    machine: Machine,
}

impl Default for MettaEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MettaEngine {
    pub fn new() -> Self {
        let machine = MachineBuilder::default().build();
        Self { machine }
    }

    pub fn run_metta_file(&mut self, filepath: &str) -> Result<(), String> {
        let query_str = format!(
            "load_metta_file('{}', Results), maplist(swrite, Results, ResultsR), maplist(format(\"~w~n\"), ResultsR).",
            filepath
        );

        let mut query = self.machine.run_query(query_str);
        if query.next().is_some() {
            Ok(())
        } else {
            Err("Query failed or returned no results".to_string())
        }
    }
}
