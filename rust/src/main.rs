//! PeTTa CLI - Production MeTTa Runtime

use clap::Parser;
use petta::{Backend, EngineConfig, PeTTaEngine};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "petta", version = "0.5.0", about = "PeTTa - Production MeTTa Runtime")]
struct Cli {
    #[arg(required = false)]
    files: Vec<String>,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    #[arg(short, long, default_value_t = false)]
    time: bool,

    #[arg(short, long, default_value = "swipl")]
    backend: String,

    #[arg(short = 'i', long, default_value_t = false)]
    interactive: bool,
}

fn main() {
    let args = Cli::parse();

    if args.files.is_empty() && !args.interactive {
        print_banner();
        run_demo();
        return;
    }

    if args.interactive {
        run_repl(&args);
        return;
    }

    run_files(&args);
}

fn print_banner() {
    println!("⚡ PeTTa v0.5.0");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🧠 Production MeTTa Runtime");
}

fn run_demo() {
    println!("\n⚡ PeTTa Demo");
    println!("Backend: Swipl");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let demos = [
        ("Identity", "(= (myid $x) $x) !(myid 42)"),
        ("Arithmetic", "!(+ 1 2)"),
        ("Boolean", "!(and true false)"),
    ];

    for (name, code) in &demos {
        println!("{name}: {code}");
    }
    println!("\n✓ Done");
}

fn run_files(args: &Cli) {
    let timing = args.time.then(std::time::Instant::now);
    let backend = parse_backend(&args.backend);

    let config = EngineConfig::new(Path::new(".")).verbose(args.verbose).backend(backend);

    let mut engine = match PeTTaEngine::with_config(&config) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("✗ Failed to initialize engine: {e}");
            std::process::exit(1);
        }
    };

    let mut exit_code = 0;
    for file in &args.files {
        let path = Path::new(file);
        if !path.exists() {
            eprintln!("✗ File not found: {file}");
            exit_code = 1;
            continue;
        }

        match engine.load_metta_file(path) {
            Ok(results) => {
                for r in &results {
                    println!("{}", r.value);
                }
            }
            Err(e) => {
                eprintln!("✗ Error loading {file}: {e}");
                exit_code = 1;
            }
        }
    }

    if let Some(start) = timing {
        eprintln!("\n⏱ Timing: {:.3}ms", start.elapsed().as_millis() as f64);
    }

    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}

fn run_repl(_args: &Cli) {
    println!("Interactive REPL not yet implemented in refactor");
}

fn parse_backend(s: &str) -> Backend {
    match s.to_lowercase().as_str() {
        "mork" => Backend::Mork,
        "swipl" | "prolog" => Backend::Swipl,
        _ => Backend::Swipl,
    }
}
