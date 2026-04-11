//! PeTTa CLI - Command-line interface for MeTTa execution
//!
//! Usage: petta [file.metta] [file2.metta] ...
//!
//! If no arguments are given, runs a small demo.

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // Determine project root: look for src/main.pl relative to the binary or CWD
    let project_root = find_project_root();

    if args.is_empty() {
        // No arguments: run a small demo
        run_demo(&project_root);
        return;
    }

    let verbose = args.iter().any(|a| a == "-v" || a == "--verbose");
    let files: Vec<&String> = args
        .iter()
        .filter(|a| *a != "-v" && *a != "--verbose")
        .collect();

    if files.is_empty() {
        eprintln!("PeTTa: MeTTa language implementation (Rust + SWI-Prolog)");
        eprintln!("Usage: petta [-v] <file.metta> [file2.metta] ...");
        eprintln!("       petta              (run demo)");
        std::process::exit(1);
    }

    run_files(&project_root, &files, verbose);
}

fn find_project_root() -> std::path::PathBuf {
    // Strategy 1: Check current directory
    let cwd = std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
    if cwd.join("src").join("main.pl").exists() {
        return cwd;
    }

    // Strategy 2: Check directory of the executable
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            if dir.join("src").join("main.pl").exists() {
                return dir.to_path_buf();
            }
            // Check parent of bin dir
            if let Some(parent) = dir.parent() {
                if parent.join("src").join("main.pl").exists() {
                    return parent.to_path_buf();
                }
            }
        }
    }

    // Strategy 3: Check PETTA_PATH env variable
    if let Ok(path) = std::env::var("PETTA_PATH") {
        let p = Path::new(&path);
        if p.join("src").join("main.pl").exists() {
            return p.to_path_buf();
        }
    }

    // Strategy 4: Walk up from CWD
    let mut current = cwd.clone();
    loop {
        if current.join("src").join("main.pl").exists() {
            return current;
        }
        if !current.pop() {
            break;
        }
    }

    // Default to CWD
    cwd
}

fn run_demo(project_root: &Path) {
    use petta::PettaEngine;

    match PettaEngine::new(project_root, true) {
        Ok(engine) => {
            println!("PeTTa Demo");
            println!("===========");

            // Simple identity (avoid "id" — it's a built-in predicate)
            match engine.process_metta_string("(= (myid $x) $x) !(myid 42)") {
                Ok(results) => {
                    println!("\nIdentity function:");
                    for r in &results {
                        println!("  Result: {}", r.value);
                    }
                }
                Err(e) => eprintln!("  Error: {}", e),
            }

            // Arithmetic
            match engine.process_metta_string("!(+ 1 2)") {
                Ok(results) => {
                    println!("\nArithmetic (+ 1 2):");
                    for r in &results {
                        println!("  Result: {}", r.value);
                    }
                }
                Err(e) => eprintln!("  Error: {}", e),
            }

            // Boolean
            match engine.process_metta_string("!(and true false)") {
                Ok(results) => {
                    println!("\nBoolean (and true false):");
                    for r in &results {
                        println!("  Result: {}", r.value);
                    }
                }
                Err(e) => eprintln!("  Error: {}", e),
            }

            println!("\nDone.");
        }
        Err(e) => {
            eprintln!("Failed to initialize PeTTa engine: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_files(project_root: &Path, files: &[&String], verbose: bool) {
    use petta::PettaEngine;

    let engine = match PettaEngine::new(project_root, verbose) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to initialize PeTTa engine: {}", e);
            std::process::exit(1);
        }
    };

    let mut had_failure = false;

    for file_path in files {
        let path = Path::new(file_path);
        if !path.exists() {
            eprintln!("Error: file not found: {}", path.display());
            had_failure = true;
            continue;
        }

        match engine.load_metta_file(path) {
            Ok(results) => {
                for r in &results {
                    println!("{}", r.value);
                }
            }
            Err(e) => {
                eprintln!("Error processing {}: {}", path.display(), e);
                had_failure = true;
            }
        }
    }

    if had_failure {
        std::process::exit(1);
    }
}
