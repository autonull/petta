//! Utility functions for PeTTa
//!
//! This module provides common utilities used throughout the codebase:
//! - ANSI color formatting
//! - Duration formatting
//! - String manipulation
//! - Fuzzy matching

// ============================================================================
// ANSI Color Formatting
// ============================================================================

/// Format string with green ANSI color
#[inline]
pub fn green(s: &str) -> String { format!("\x1b[32m{s}\x1b[0m") }

/// Format string with red ANSI color
#[inline]
pub fn red(s: &str) -> String { format!("\x1b[31m{s}\x1b[0m") }

/// Format string with yellow ANSI color
#[inline]
pub fn yellow(s: &str) -> String { format!("\x1b[33m{s}\x1b[0m") }

/// Format string with cyan ANSI color
#[inline]
pub fn cyan(s: &str) -> String { format!("\x1b[36m{s}\x1b[0m") }

/// Format string with blue ANSI color
#[inline]
pub fn blue(s: &str) -> String { format!("\x1b[34m{s}\x1b[0m") }

/// Format string with bold ANSI style
#[inline]
pub fn bold(s: &str) -> String { format!("\x1b[1m{s}\x1b[0m") }

// ============================================================================
// Time Formatting
// ============================================================================

/// Format duration in human-readable form
///
/// Automatically chooses appropriate units:
/// - nanoseconds (ns) for values < 1μs
/// - microseconds (μs) for values < 1ms
/// - milliseconds (ms) for values < 1s
/// - seconds (s) for larger values
///
/// # Example
///
/// ```
/// use petta::utils::format_duration_ms;
///
/// assert!(format_duration_ms(0.5).contains("μs"));
/// assert!(format_duration_ms(123.45).contains("ms"));
/// assert!(format_duration_ms(1234.5).contains("s"));
/// ```
#[inline]
pub fn format_duration_ms(ms: f64) -> String {
    if ms < 1e-3 { format!("{:.2}ns", ms * 1e6) }
    else if ms < 1.0 { format!("{:.2}μs", ms * 1000.0) }
    else if ms < 1000.0 { format!("{:.2}ms", ms) }
    else { format!("{:.2}s", ms / 1000.0) }
}

// ============================================================================
// String Utilities
// ============================================================================

/// Truncate string with ellipsis if it exceeds maximum length
#[inline]
pub fn truncate(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len { s } else { &s[..max_len.saturating_sub(3)] }
}

// ============================================================================
// Fuzzy Matching (Levenshtein Distance)
// ============================================================================

/// Calculate Levenshtein distance between two strings
///
/// The Levenshtein distance is the minimum number of single-character edits
/// (insertions, deletions, or substitutions) required to change one word
/// into the other.
///
/// # Example
///
/// ```
/// use petta::utils::levenshtein;
///
/// assert_eq!(levenshtein("kitten", "sitting"), 3);
/// assert_eq!(levenshtein("", "abc"), 3);
/// ```
#[allow(clippy::needless_range_loop)]
pub fn levenshtein(a: &str, b: &str) -> usize {
    let (a, b): (Vec<_>, Vec<_>) = (a.chars().collect(), b.chars().collect());
    let (alen, blen) = (a.len(), b.len());

    if alen == 0 { return blen; }
    if blen == 0 { return alen; }

    let mut dp = vec![vec![0; blen + 1]; alen + 1];
    for i in 0..=alen { dp[i][0] = i; }
    for j in 0..=blen { dp[0][j] = j; }

    for (i, a_char) in a.iter().enumerate() {
        for (j, b_char) in b.iter().enumerate() {
            let cost = if a_char == b_char { 0 } else { 1 };
            dp[i+1][j+1] = (dp[i][j+1] + 1).min(dp[i+1][j] + 1).min(dp[i][j] + cost);
        }
    }
    dp[alen][blen]
}

/// Find best fuzzy match from a list of candidates
///
/// Returns the candidate with the smallest Levenshtein distance to the target,
/// but only if the distance is within an acceptable threshold (max of string length or 3).
///
/// # Example
///
/// ```
/// use petta::utils::find_best_match;
///
/// let candidates = ["apple", "banana", "cherry"];
/// let best = find_best_match("appple", &candidates);
/// assert_eq!(best, Some("apple"));
/// ```
pub fn find_best_match<'a>(target: &str, candidates: &'a [&str]) -> Option<&'a str> {
    let max_dist = target.len().max(3);
    candidates.iter()
        .filter_map(|&c| {
            let dist = levenshtein(target, c);
            if dist <= max_dist { Some((c, dist)) } else { None }
        })
        .min_by_key(|(_, d)| *d)
        .map(|(c, _)| c)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration_ns() {
        assert!(format_duration_ms(0.000001).contains("ns"));
    }

    #[test]
    fn test_format_duration_us() {
        let result = format_duration_ms(0.5);
        assert!(result.contains("μs"));
    }

    #[test]
    fn test_format_duration_ms() {
        let result = format_duration_ms(123.45);
        assert!(result.contains("ms"));
    }

    #[test]
    fn test_format_duration_s() {
        let result = format_duration_ms(1234.5);
        assert!(result.contains("s"));
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("hello", 10), "hello");
        assert_eq!(truncate("hello", 3), "");
    }

    #[test]
    fn test_levenshtein() {
        assert_eq!(levenshtein("", ""), 0);
        assert_eq!(levenshtein("abc", ""), 3);
        assert_eq!(levenshtein("", "abc"), 3);
        assert_eq!(levenshtein("kitten", "sitting"), 3);
        assert_eq!(levenshtein("abc", "abc"), 0);
    }

    #[test]
    fn test_find_best_match() {
        let candidates = ["apple", "banana", "cherry"];
        assert_eq!(find_best_match("appple", &candidates), Some("apple"));
        assert_eq!(find_best_match("banna", &candidates), Some("banana"));
    }

    #[test]
    fn test_color_functions() {
        assert!(green("test").contains("\x1b[32m"));
        assert!(red("test").contains("\x1b[31m"));
        assert!(yellow("test").contains("\x1b[33m"));
        assert!(cyan("test").contains("\x1b[36m"));
        assert!(blue("test").contains("\x1b[34m"));
        assert!(bold("test").contains("\x1b[1m"));
    }
}
