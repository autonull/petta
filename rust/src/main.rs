//! PeTTa CLI - Production MeTTa Runtime

#[cfg(feature = "clap")]
use clap::Parser;

#[cfg(feature = "clap")]
use petta::Cli;

#[cfg(feature = "clap")]
use petta::{
    Backend, EngineConfig, PeTTaEngine, ReplConfig, run_repl,
    utils::red,
};
use petta::utils::cyan;
#[cfg(feature = "clap")]
use std::path::Path;

fn main() {
    #[cfg(feature = "clap")]
    {
        let args = Cli::parse();

        if args.files.is_empty() && !args.interactive {
            print_banner(&args);
            run_demo();
            return;
        }

        if args.interactive {
            run_repl_mode(&args);
            return;
        }

        run_files(&args);
    }

    #[cfg(not(feature = "clap"))]
    {
        println!("{} {}", cyan("⚡"), cyan("PeTTa v0.5.0"));
        println!("Compiled without clap feature. CLI arguments disabled.");
        run_demo();
    }
}

#[cfg(not(feature = "clap"))]
fn run_demo() {
    println!("\n{} {}", cyan("⚡"), cyan("PeTTa Demo"));
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

#[cfg(feature = "clap")]
fn print_banner(args: &Cli) {
    println!("{} {}", cyan("⚡"), cyan("PeTTa v0.5.0"));
    println!("{}", cyan("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"));
    println!("{} {}", cyan("🧠"), cyan("Production MeTTa Runtime"));
    if args.verbose {
        println!("{} {}", cyan("Backend:"), args.backend);
        println!("{} {}", cyan("Output:"), args.output_format);
    }
}

#[cfg(feature = "clap")]
fn run_demo() {
    println!("\n{} {}", cyan("⚡"), cyan("PeTTa Demo"));
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

#[cfg(feature = "clap")]
fn run_repl_mode(args: &Cli) {
    let config = ReplConfig::new(".")
        .verbose(args.verbose)
        .backend(parse_backend(&args.backend.to_string()));
    run_repl(&config);
}

#[cfg(feature = "clap")]
fn run_files(args: &Cli) {
    eprintln!("[DBG] run_files called with files={:?}", args.files);
    let timing = args.time.then(std::time::Instant::now);
    let backend = parse_backend(&args.backend.to_string());
    eprintln!("[DBG] backend={:?}", backend);

    // Separate actual files from key=value config overrides
    let mut files = Vec::new();
    let mut extra_args = Vec::new();
    for arg in &args.files {
        if arg.contains('=') && !Path::new(arg).exists() {
            extra_args.push(arg.clone());
        } else {
            files.push(arg.clone());
        }
    }
    eprintln!("[DBG] files after filter={:?}, extra_args={:?}", files, extra_args);

    let mut config = EngineConfig::new(Path::new(".")).verbose(args.verbose).backend(backend);
    config.extra_args = extra_args;
    eprintln!("[DBG] config created, src_dir={:?}", config.src_dir);

    eprintln!("[DBG] calling PeTTaEngine::with_config...");
    let mut engine = match PeTTaEngine::with_config(&config) {
        Ok(e) => {
            eprintln!("[DBG] PeTTaEngine::with_config succeeded, backend={}", e.backend_name());
            e
        }
        Err(e) => {
            eprintln!("{} Failed to initialize engine: {}", red("✗"), e);
            std::process::exit(1);
        }
    };
    eprintln!("[DBG] engine ready");

    let mut exit_code = 0;
    for file in &files {
        eprintln!("[DBG] processing file: {}", file);
        let path = Path::new(file);
        if !path.exists() {
            eprintln!("{} File not found: {}", red("✗"), file);
            exit_code = 1;
            continue;
        }
        eprintln!("[DBG] file exists, calling load_metta_file...");

        match engine.load_metta_file(path) {
            Ok(results) => {
                eprintln!("[DBG] load_metta_file returned {} results", results.len());
                eprintln!("[DBG] stderr from prolog: {}", engine.stderr_output());
                for r in &results {
                    println!("{}", r.value);
                }
            }
            Err(e) => {
                eprintln!("{} Error loading {}: {}", red("✗"), file, e);
                eprintln!("[DBG] stderr from prolog: {}", engine.stderr_output());
                exit_code = 1;
            }
        }
    }

    if let Some(start) = timing {
        eprintln!("\n⏱ Timing: {:.3}ms", start.elapsed().as_millis() as f64);
    }

    eprintln!("[DBG] run_files exit_code={}", exit_code);
    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}

#[cfg(feature = "clap")]
fn parse_backend(s: &str) -> Backend {
    match s.to_lowercase().as_str() {
        "mork" => Backend::Mork,
        "swipl" | "prolog" => Backend::Swipl,
        _ => Backend::Swipl,
    }
}
