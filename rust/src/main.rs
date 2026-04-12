//! PeTTa CLI - Command-line interface for MeTTa execution

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let project_root = find_project_root();

    if args.is_empty() {
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
    let cwd = std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
    if cwd.join("src").join("main.pl").exists() {
        return cwd;
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            if dir.join("src").join("main.pl").exists() {
                return dir.to_path_buf();
            }
            if let Some(parent) = dir.parent() {
                if parent.join("src").join("main.pl").exists() {
                    return parent.to_path_buf();
                }
            }
        }
    }
    if let Ok(path) = std::env::var("PETTA_PATH") {
        let p = Path::new(&path);
        if p.join("src").join("main.pl").exists() {
            return p.to_path_buf();
        }
    }
    let mut current = cwd.clone();
    loop {
        if current.join("src").join("main.pl").exists() {
            return current;
        }
        if !current.pop() {
            break;
        }
    }
    cwd
}

fn run_demo(project_root: &Path) {
    use petta::PeTTaEngine;
    match PeTTaEngine::new(project_root, false) {
        Ok(engine) => {
            println!("PeTTa Demo\n===========");
            let cases = [
                ("Identity", "(= (myid $x) $x) !(myid 42)"),
                ("Arithmetic", "!(+ 1 2)"),
                ("Boolean", "!(and true false)"),
            ];
            for (name, code) in &cases {
                match engine.process_metta_string(code) {
                    Ok(results) => {
                        println!("\n{}:", name);
                        for r in &results {
                            println!("  Result: {}", r.value);
                        }
                    }
                    Err(e) => eprintln!("  Error: {}", e),
                }
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
    use petta::PeTTaEngine;

    // Validate all files exist first before spawning any subprocess
    let paths: Vec<&Path> = files
        .iter()
        .map(|f| Path::new(f))
        .filter(|p| {
            if !p.exists() {
                eprintln!("Error: file not found: {}", p.display());
                false
            } else {
                true
            }
        })
        .collect();

    if paths.is_empty() {
        std::process::exit(1);
    }

    // Batch all files into a single subprocess
    let engine = match PeTTaEngine::new(project_root, verbose) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to initialize PeTTa engine: {}", e);
            std::process::exit(1);
        }
    };

    match engine.load_metta_files(&paths) {
        Ok(results) => {
            for r in &results {
                println!("{}", r.value);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
