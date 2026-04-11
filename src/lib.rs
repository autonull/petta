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
        let mut machine = MachineBuilder::default().build();

        // Load the main metta prolog logic files required to parse and evaluate metta files
        let metta_pl = include_str!("../prolog/metta.pl");
        let parser_pl = include_str!("../prolog/parser.pl");
        let translator_pl = include_str!("../prolog/translator.pl");
        let specializer_pl = include_str!("../prolog/specializer.pl");
        let filereader_pl = include_str!("../prolog/filereader.pl");
        let spaces_pl = include_str!("../prolog/spaces.pl");

        // The order might matter. Let's just create a combined module or load them one by one.
        machine.load_module_string("parser", parser_pl.to_string());
        machine.load_module_string("translator", translator_pl.to_string());
        machine.load_module_string("specializer", specializer_pl.to_string());
        machine.load_module_string("filereader", filereader_pl.to_string());
        machine.load_module_string("spaces", spaces_pl.to_string());
        machine.load_module_string("metta", metta_pl.to_string());

        Self { machine }
    }

    pub fn run_metta_file(&mut self, filepath: &str) -> Result<(), String> {
        let query_str = format!("load_metta_file('{}', Results), maplist(swrite, Results, ResultsR), maplist(format(\"~w~n\"), ResultsR).", filepath);

        let mut query = self.machine.run_query(query_str);
        if query.next().is_some() {
            Ok(())
        } else {
            Err("Query failed or returned no results".to_string())
        }
    }
}
