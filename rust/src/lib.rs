//! PeTTa - MeTTa language implementation in Rust + SWI-Prolog
//!
//! This crate provides a Rust wrapper around SWI-Prolog to execute
//! MeTTa (Metalanguage for Transformation) programs.
//!
//! Each call to `load_metta_file` or `process_metta_string` spawns a
//! fresh SWI-Prolog subprocess. For multi-call state persistence,
//! collect all MeTTa code into a single call.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

/// Global counter for unique temp filenames across parallel tests.
static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Represents a result from executing MeTTa code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MettaResult {
    /// The string representation of the result (S-expression format).
    pub value: String,
}

/// The main PeTTa engine wrapper.
pub struct PeTTaEngine {
    project_root: PathBuf,
    verbose: bool,
}

impl PeTTaEngine {
    /// Create a new PeTTa engine.
    ///
    /// # Arguments
    /// * `project_root` - The root directory of the PeTTa project (where src/ lives).
    /// * `verbose` - Whether to enable verbose output (debug info from Prolog).
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

        if self.verbose {
            // No extra args needed -- metta.pl defaults to non-silent
        } else {
            cmd.arg("--silent");
        }

        let output = cmd
            .output()
            .map_err(|e| PeTTaError::SpawnSwipl(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PeTTaError::SwiplError(format!(
                "swipl exited with status {}: {}",
                output.status,
                stderr.trim()
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(parse_output(&stdout, self.verbose))
    }

    /// Process a MeTTa code string.
    pub fn process_metta_string(
        &self,
        metta_code: &str,
    ) -> Result<Vec<MettaResult>, PeTTaError> {
        let main_pl = self.project_root.join("src").join("main.pl");

        // Write MeTTa code to a temp file to avoid shell injection
        let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let tmp_dir = std::env::temp_dir();
        let tmp_path = tmp_dir.join(format!("petta_{}_{}.metta", std::process::id(), counter));

        std::fs::write(&tmp_path, metta_code)
            .map_err(|e| PeTTaError::WriteError(e.to_string()))?;

        let tmp_str = tmp_path.to_string_lossy();
        // Escape single quotes in the path for Prolog
        let tmp_escaped = tmp_str.replace('\'', "''");

        // Call read_file_to_string + process_metta_string directly via -g.
        // This bypasses main.pl's main/0 predicate and gives us clean results.
        let silent_flag = if self.verbose { "false" } else { "true" };
        let query = format!(
            "assertz(working_dir('{}')), assertz(silent({})), \
             read_file_to_string('{}', Code, []), \
             process_metta_string(Code, Results), \
             maplist(swrite, Results, Strings), \
             (Strings == [] -> true ; maplist(writeln, Strings)), \
             delete_file('{}'), halt.",
            self.project_root.to_string_lossy().replace('\\', "\\\\"),
            silent_flag,
            tmp_escaped,
            tmp_escaped
        );

        let output = Command::new("swipl")
            .arg("-q")
            .arg("-s")
            .arg(&main_pl)
            .arg("-g")
            .arg(&query)
            .output()
            .map_err(|e| PeTTaError::SpawnSwipl(e.to_string()))?;

        if !output.status.success() {
            let _ = std::fs::remove_file(&tmp_path);
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PeTTaError::SwiplError(format!(
                "swipl exited with status {}: {}",
                output.status,
                stderr.trim()
            )));
        }

        let _ = std::fs::remove_file(&tmp_path);

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(parse_output(&stdout, self.verbose))
    }
}

/// Parse SWI-Prolog output into MettaResult values.
fn parse_output(output: &str, _verbose: bool) -> Vec<MettaResult> {
    output
        .lines()
        .filter_map(|line| {
            // Strip ANSI codes first, then filter
            let cleaned = strip_ansi(line.trim());
            if cleaned.is_empty()
                || cleaned.starts_with('%')
                || cleaned.starts_with("?-")
                || cleaned.starts_with(":-")
                || cleaned.starts_with("-->")
                || cleaned.starts_with("^^^")
                || cleaned.starts_with('!')  // MeTTa runnable forms (debug echo)
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

/// Strip ANSI escape sequences.
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
        } else {
            result.push(ch);
        }
    }
    result.trim().to_string()
}

/// Errors that can occur during PeTTa execution.
#[derive(Debug)]
pub enum PeTTaError {
    FileNotFound(PathBuf),
    SpawnSwipl(String),
    PathError(String),
    WriteError(String),
    ReadError(String),
    SwiplError(String),
}

impl std::fmt::Display for PeTTaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PeTTaError::FileNotFound(p) => write!(f, "File not found: {}", p.display()),
            PeTTaError::SpawnSwipl(e) => write!(f, "Failed to spawn swipl: {}", e),
            PeTTaError::PathError(e) => write!(f, "Path error: {}", e),
            PeTTaError::WriteError(e) => write!(f, "Write error: {}", e),
            PeTTaError::ReadError(e) => write!(f, "Read error: {}", e),
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
        Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
    }

    fn make_engine() -> PeTTaEngine {
        let root = project_root();
        PeTTaEngine::new(&root, false).expect("Failed to create engine")
    }

    #[test]
    fn test_engine_creation() {
        let root = project_root();
        assert!(PeTTaEngine::new(&root, false).is_ok());
    }

    #[test]
    fn test_identity() {
        let engine = make_engine();
        let results = engine
            .process_metta_string("(= (myid $x) $x) !(myid 42)")
            .unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].value, "42");
    }

    #[test]
    fn test_arithmetic() {
        let engine = make_engine();
        let results = engine.process_metta_string("!(+ 1 2)").unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].value, "3");
    }

    #[test]
    fn test_load_identity_file() {
        let engine = make_engine();
        let root = project_root();
        let results = engine
            .load_metta_file(&root.join("examples/identity.metta"))
            .unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_boolean() {
        let engine = make_engine();
        let results = engine
            .process_metta_string("!(and true false)")
            .unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].value, "false");
    }

    #[test]
    fn test_comparison() {
        let engine = make_engine();
        let results = engine.process_metta_string("!(< 1 2)").unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].value, "true");
    }

    #[test]
    fn test_fibonacci() {
        let engine = make_engine();
        let root = project_root();
        let results = engine
            .load_metta_file(&root.join("examples/fib.metta"))
            .unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_state() {
        let engine = make_engine();
        let root = project_root();
        let results = engine
            .load_metta_file(&root.join("examples/state.metta"))
            .unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_if() {
        let engine = make_engine();
        let root = project_root();
        let results = engine
            .load_metta_file(&root.join("examples/if.metta"))
            .unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_math() {
        let engine = make_engine();
        let root = project_root();
        let results = engine
            .load_metta_file(&root.join("examples/math.metta"))
            .unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_variable_renaming() {
        let engine = make_engine();
        let results = engine
            .process_metta_string("(= (fun ($a x)) ($b x)) !(fun (a x))")
            .unwrap();
        assert!(!results.is_empty());
        let val = &results[0].value;
        // Result should be ($_N x) or ($b x) — variable followed by x
        assert!(
            val.contains("$") && val.contains('x'),
            "Expected variable pattern, got: {}",
            val
        );
    }

    #[test]
    fn test_file_imports() {
        let engine = make_engine();
        let root = project_root();
        // Load identity.metta then call f in the same subprocess
        let identity_code =
            std::fs::read_to_string(root.join("examples/identity.metta")).unwrap();
        let combined = format!("{}\n!(f 5)", identity_code);
        let results = engine.process_metta_string(&combined).unwrap();
        // identity.metta includes !(test (f 1) 1) which produces "is 1, should 1..."
        // The last result should be from !(f 5) = 25
        assert!(
            results.iter().any(|r| r.value == "25"),
            "Expected '25' in results: {:?}",
            results
        );
    }

    #[test]
    fn test_verbose_mode() {
        let root = project_root();
        let engine = PeTTaEngine::new(&root, true).unwrap();
        let results = engine.process_metta_string("!(+ 1 2)").unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].value, "3");
    }

    #[test]
    fn test_parse_output() {
        let results = parse_output("42\n(a b c)\nhello", false);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].value, "42");
        assert_eq!(results[1].value, "(a b c)");
        assert_eq!(results[2].value, "hello");
    }

    #[test]
    fn test_parse_output_filters_debug() {
        // Both raw ANSI-coded and clean debug lines should be filtered
        let results = parse_output(
            "--> metta function -->\n42\n^^^^^^^^^^^^^^^^^^^\n\x1b[36m!(+ 1 2)\n\x1b[33m-->",
            false,
        );
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].value, "42");
    }
}
