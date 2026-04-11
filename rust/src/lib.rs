//! PeTTa - MeTTa language implementation in Rust + SWI-Prolog
//!
//! This crate provides a Rust wrapper around SWI-Prolog to execute
//! MeTTa (Metalanguage for Transformation) programs.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Represents a result from executing MeTTa code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MettaResult {
    /// The string representation of the result (S-expression format).
    pub value: String,
}

/// The main PeTTa engine wrapper.
///
/// Manages the project root and provides methods to load MeTTa files
/// and process MeTTa code strings via SWI-Prolog subprocesses.
pub struct PettaEngine {
    project_root: PathBuf,
    verbose: bool,
}

impl PettaEngine {
    /// Create a new PeTTa engine.
    ///
    /// # Arguments
    /// * `project_root` - The root directory of the PeTTa project (where src/ lives).
    /// * `verbose` - Whether to enable verbose output.
    pub fn new(project_root: &Path, verbose: bool) -> Result<Self, PeTTaError> {
        let main_pl = project_root.join("src").join("main.pl");
        if !main_pl.exists() {
            return Err(PeTTaError::FileNotFound(main_pl));
        }

        let abs_root = project_root
            .canonicalize()
            .map_err(|e| PeTTaError::PathError(e.to_string()))?;

        Ok(Self {
            project_root: abs_root,
            verbose,
        })
    }

    /// Load and execute a MeTTa file.
    ///
    /// # Arguments
    /// * `file_path` - Path to the .metta file.
    pub fn load_metta_file(&self, file_path: &Path) -> Result<Vec<MettaResult>, PeTTaError> {
        let abs_path = file_path
            .canonicalize()
            .map_err(|e| PeTTaError::PathError(e.to_string()))?;

        if !abs_path.exists() {
            return Err(PeTTaError::FileNotFound(abs_path));
        }

        let main_pl = self.project_root.join("src").join("main.pl");
        let rel_path = abs_path
            .strip_prefix(&self.project_root)
            .unwrap_or(&abs_path);

        let mut cmd = Command::new("swipl");
        cmd.arg("-q")
            .arg("-s")
            .arg(&main_pl)
            .arg("--")
            .arg(rel_path);

        if !self.verbose {
            cmd.arg("--silent");
        }

        let output = cmd
            .output()
            .map_err(|e| PeTTaError::SpawnSwipl(e.to_string()))?;

        if !output.stderr.is_empty() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if self.verbose {
                eprintln!("[PeTTa] SWI-Prolog stderr: {}", stderr.trim());
            }
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(parse_output(&stdout))
    }

    /// Process a MeTTa code string.
    ///
    /// # Arguments
    /// * `metta_code` - MeTTa source code as a string.
    pub fn process_metta_string(
        &self,
        metta_code: &str,
    ) -> Result<Vec<MettaResult>, PeTTaError> {
        let main_pl = self.project_root.join("src").join("main.pl");

        // Escape single quotes and backslashes for Prolog string
        let escaped = metta_code.replace('\\', "\\\\").replace('\'', "\\'");

        // Build the query: set working dir, enable silent mode, process code, output results
        let query = format!(
            "assertz(working_dir('{}')), assertz(silent(true)), process_metta_string('{}', Results), maplist(swrite, Results, Strings), (Strings == [] -> true ; maplist(writeln, Strings)), halt.",
            self.project_root.to_string_lossy().replace('\\', "\\\\"),
            escaped
        );

        let output = Command::new("swipl")
            .arg("-q")
            .arg("-s")
            .arg(&main_pl)
            .arg("-g")
            .arg(&query)
            .output()
            .map_err(|e| PeTTaError::SpawnSwipl(e.to_string()))?;

        if !output.stderr.is_empty() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if self.verbose {
                eprintln!("[PeTTa] SWI-Prolog stderr: {}", stderr.trim());
            }
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(parse_output(&stdout))
    }
}

/// Parse SWI-Prolog output (one result per line) into MettaResult values.
fn parse_output(output: &str) -> Vec<MettaResult> {
    output
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            // Skip empty lines, debug output, and SWI prompts
            if trimmed.is_empty()
                || trimmed.starts_with('%')
                || trimmed.starts_with("?-")
                || trimmed.starts_with(":-")
                || trimmed.starts_with("-->")
                || trimmed.starts_with("^^^")
            {
                return None;
            }
            // Strip ANSI escape codes
            let cleaned = strip_ansi(trimmed);
            if cleaned.is_empty()
                || cleaned.starts_with("-->")
                || cleaned.starts_with("^^^")
                || cleaned.contains("metta function")
                || cleaned.contains("metta runnable")
                || cleaned.contains("prolog clause")
                || cleaned.contains("prolog goal")
            {
                return None;
            }
            Some(MettaResult { value: cleaned })
        })
        .collect()
}

/// Strip ANSI escape sequences from a string.
fn strip_ansi(s: &str) -> String {
    let mut result = String::new();
    let mut in_escape = false;
    for ch in s.chars() {
        if ch == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if ch == 'm' || ch == 'H' || ch == 'J' || ch == 'K' {
                in_escape = false;
            }
            // Skip other chars within escape sequence (like [33m)
        } else {
            result.push(ch);
        }
    }
    result.trim().to_string()
}

/// Errors that can occur during PeTTa execution.
#[derive(Debug)]
pub enum PeTTaError {
    /// A required file was not found.
    FileNotFound(PathBuf),
    /// Failed to spawn swipl subprocess.
    SpawnSwipl(String),
    /// Path canonicalization error.
    PathError(String),
    /// SWI-Prolog returned an error (read from stderr).
    SwiplError(String),
}

impl std::fmt::Display for PeTTaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PeTTaError::FileNotFound(p) => write!(f, "File not found: {}", p.display()),
            PeTTaError::SpawnSwipl(e) => write!(f, "Failed to spawn swipl: {}", e),
            PeTTaError::PathError(e) => write!(f, "Path error: {}", e),
            PeTTaError::SwiplError(e) => write!(f, "SWI-Prolog error: {}", e),
        }
    }
}

impl std::error::Error for PeTTaError {}

/// Check if SWI-Prolog is available.
pub fn swipl_available() -> bool {
    Command::new("swipl")
        .arg("--version")
        .output()
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project_root() -> PathBuf {
        // CARGO_MANIFEST_DIR is the directory containing Cargo.toml,
        // which is the project root itself (Cargo.toml is at repo root)
        Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
    }

    #[test]
    fn test_engine_creation() {
        let root = project_root();
        let engine = PettaEngine::new(&root, false);
        assert!(engine.is_ok(), "Failed to create engine: {:?}", engine.err());
    }

    #[test]
    fn test_identity() {
        let root = project_root();
        let engine = PettaEngine::new(&root, false).unwrap();
        // Use a unique name to avoid conflicts with built-in predicates like id/2
        let results = engine.process_metta_string("(= (myid $x) $x) !(myid 42)").unwrap();
        assert!(!results.is_empty(), "Expected at least one result");
        assert_eq!(results[0].value, "42");
    }

    #[test]
    fn test_arithmetic() {
        let root = project_root();
        let engine = PettaEngine::new(&root, false).unwrap();
        let results = engine.process_metta_string("!(+ 1 2)").unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].value, "3");
    }

    #[test]
    fn test_load_identity_file() {
        let root = project_root();
        let engine = PettaEngine::new(&root, false).unwrap();
        let results = engine.load_metta_file(&root.join("examples/identity.metta")).unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_boolean() {
        let root = project_root();
        let engine = PettaEngine::new(&root, false).unwrap();
        let results = engine.process_metta_string("!(and true false)").unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].value, "false");
    }

    #[test]
    fn test_comparison() {
        let root = project_root();
        let engine = PettaEngine::new(&root, false).unwrap();
        let results = engine.process_metta_string("!(< 1 2)").unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].value, "true");
    }

    #[test]
    fn test_fibonacci() {
        let root = project_root();
        let engine = PettaEngine::new(&root, false).unwrap();
        let results = engine.load_metta_file(&root.join("examples/fib.metta")).unwrap();
        // fib.metta has a test that prints pass/fail
        assert!(!results.is_empty());
    }

    #[test]
    fn test_state() {
        let root = project_root();
        let engine = PettaEngine::new(&root, false).unwrap();
        let results = engine.load_metta_file(&root.join("examples/state.metta")).unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_if() {
        let root = project_root();
        let engine = PettaEngine::new(&root, false).unwrap();
        let results = engine.load_metta_file(&root.join("examples/if.metta")).unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_math() {
        let root = project_root();
        let engine = PettaEngine::new(&root, false).unwrap();
        let results = engine.load_metta_file(&root.join("examples/math.metta")).unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_parse_output() {
        let results = parse_output("42\n(a b c)\nhello");
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].value, "42");
        assert_eq!(results[1].value, "(a b c)");
        assert_eq!(results[2].value, "hello");
    }

    #[test]
    fn test_parse_output_filters_debug() {
        let results = parse_output("--> metta function -->\n42\n^^^^^^^^^^^^^^^^^^^");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].value, "42");
    }
}
