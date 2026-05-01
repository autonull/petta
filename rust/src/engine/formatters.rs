//! Output formatters for PeTTa

use crate::values::MettaResult;

/// Trait for output formatting
pub trait OutputFormatter: Send + Sync {
    fn format(&self, results: &[MettaResult]) -> String;
    fn format_single(&self, result: &MettaResult) -> String;
}

/// Pretty formatter
pub struct PrettyFormatter { use_color: bool }
impl PrettyFormatter {
    pub fn new(use_color: bool) -> Self { Self { use_color } }
}
impl OutputFormatter for PrettyFormatter {
    fn format(&self, results: &[MettaResult]) -> String {
        results.iter().map(|r| self.format_single(r)).collect::<Vec<_>>().join("\n")
    }
    fn format_single(&self, result: &MettaResult) -> String {
        if self.use_color {
            format!("{} {}", crate::utils::green("✓"), crate::utils::cyan(&result.value))
        } else {
            format!(" {}", result.value)
        }
    }
}

/// Compact formatter
#[derive(Default)]
pub struct CompactFormatter;
impl CompactFormatter { pub fn new() -> Self { Self } }
impl OutputFormatter for CompactFormatter {
    fn format(&self, results: &[MettaResult]) -> String {
        results.iter().map(|r| r.value.as_str()).collect::<Vec<_>>().join(" ")
    }
    fn format_single(&self, result: &MettaResult) -> String { result.value.clone() }
}

/// JSON formatter
#[derive(Default)]
pub struct JsonFormatter;
impl JsonFormatter { pub fn new() -> Self { Self } }
impl OutputFormatter for JsonFormatter {
    fn format(&self, results: &[MettaResult]) -> String {
        let values: Vec<&str> = results.iter().map(|r| r.value.as_str()).collect();
        serde_json::to_string(&values).unwrap_or_else(|_| "[]".to_string())
    }
    fn format_single(&self, result: &MettaResult) -> String {
        serde_json::to_string(&result.value).unwrap_or_else(|_| "\"\"".to_string())
    }
}

/// S-expression formatter
#[derive(Default)]
pub struct SExprFormatter;
impl SExprFormatter { pub fn new() -> Self { Self } }
impl OutputFormatter for SExprFormatter {
    fn format(&self, results: &[MettaResult]) -> String {
        results.iter().map(|r| self.format_single(r)).collect::<Vec<_>>().join("\n")
    }
    fn format_single(&self, result: &MettaResult) -> String {
        let v = &result.value;
        if v.starts_with('(') || v.starts_with('[') { v.clone() }
        else { format!("({})", v) }
    }
}

pub fn create_formatter(format: &str, use_color: bool) -> Box<dyn OutputFormatter> {
    match format {
        "compact" => Box::new(CompactFormatter::new()),
        "json" => Box::new(JsonFormatter::new()),
        "sexpr" => Box::new(SExprFormatter::new()),
        _ => Box::new(PrettyFormatter::new(use_color)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn make_result(value: &str) -> MettaResult { MettaResult { value: value.into() } }
    
    #[test]
    fn test_compact_formatter() {
        let formatter = CompactFormatter::new();
        let results = vec![make_result("1"), make_result("2"), make_result("3")];
        assert_eq!(formatter.format(&results), "1 2 3");
    }
}
