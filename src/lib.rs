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
        let disable_warnings = ":- discontiguous throw_dcg_expansion_error/1.\n";
        machine.load_module_string("parser", format!("{}{}", disable_warnings, parser_pl));
        machine.load_module_string("translator", format!("{}{}", disable_warnings, translator_pl));
        machine.load_module_string("specializer", format!("{}{}", disable_warnings, specializer_pl));
        machine.load_module_string("filereader", format!("{}{}", disable_warnings, filereader_pl));
        machine.load_module_string("spaces", format!("{}{}", disable_warnings, spaces_pl));
        machine.load_module_string("metta", format!("{}{}", disable_warnings, metta_pl));

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
