use clap::Parser;
use petta::MettaEngine;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The MeTTa file to execute
    file: String,
}

fn main() {
    let args = Args::parse();

    let mut engine = MettaEngine::new();
    match engine.run_metta_file(&args.file) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}
