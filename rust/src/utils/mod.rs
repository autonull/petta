//! Utility functions for PeTTa
//!
//! Provides common utilities for formatting, string manipulation, and ANSI output

/// Format a string with bold
pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

/// Format a string with green color
pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

/// Format a string with red color
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

/// Format a string with yellow color
pub fn yellow(s: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", s)
}

/// Format a string with cyan color
pub fn cyan(s: &str) -> String {
    format!("\x1b[36m{}\x1b[0m", s)
}

/// Format a string with blue color
pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}

/// Format duration in human-readable form
pub fn format_duration_ms(ms: f64) -> String {
    if ms < 1e-3 {
        format!("{:.2}ns", ms * 1e6)
    } else if ms < 1.0 {
        format!("{:.2}μs", ms * 1000.0)
    } else if ms < 1000.0 {
        format!("{:.2}ms", ms)
    } else {
        format!("{:.2}s", ms / 1000.0)
    }
}

/// Truncate string to max length with ellipsis
pub fn truncate(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len {
        s
    } else {
        &s[..max_len.saturating_sub(3) + 3]
    }
}

/// Word wrap text to specified width
pub fn word_wrap(text: &str, width: usize) -> String {
    if width == 0 {
        return text.to_string();
    }

    let mut result = String::with_capacity(text.len());
    let mut line_len = 0;

    for word in text.split_whitespace() {
        let word_len = word.len();

        if line_len + word_len + 1 > width {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(word);
            line_len = word_len;
        } else {
            if line_len > 0 {
                result.push(' ');
                line_len += 1;
            }
            result.push_str(word);
            line_len += word_len;
        }
    }

    result
}

/// Indent text by specified number of spaces
pub fn indent(text: &str, spaces: usize) -> String {
    let prefix = " ".repeat(spaces);
    text.lines()
        .map(|line| format!("{}{}", prefix, line))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Calculate Levenshtein distance for suggestions
pub fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

    for i in 0..=a_len {
        matrix[i][0] = i;
    }
    for j in 0..=b_len {
        matrix[0][j] = j;
    }

    for i in 1..=a_len {
        for j in 1..=b_len {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }

    matrix[a_len][b_len]
}

/// Find best match from candidates based on Levenshtein distance
pub fn find_best_match<'a>(target: &str, candidates: &'a [&'a str]) -> Option<&'a str> {
    if candidates.is_empty() {
        return None;
    }

    let max_distance = target.len().max(3);
    let mut best_match = None;
    let mut best_distance = max_distance + 1;

    for &candidate in candidates {
        let distance = levenshtein(target, candidate);
        if distance < best_distance && distance <= max_distance {
            best_match = Some(candidate);
            best_distance = distance;
        }
    }

    best_match
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("hello", 10), "hello");
        assert_eq!(truncate("hello", 5), "hello");
        let result = truncate("hello", 3);
        assert!(result.len() <= 3);
    }

    #[test]
    fn test_levenshtein() {
        assert_eq!(levenshtein("kitten", "sitting"), 3);
        assert_eq!(levenshtein("", "abc"), 3);
        assert_eq!(levenshtein("abc", ""), 3);
        assert_eq!(levenshtein("same", "same"), 0);
    }

    #[test]
    fn test_find_best_match() {
        let candidates = ["foo", "bar", "baz", "qux"];
        assert_eq!(find_best_match("fo", &candidates), Some("foo"));
        assert_eq!(find_best_match("ba", &candidates), Some("bar"));
    }

    #[test]
    fn test_format_duration() {
        assert!(format_duration_ms(0.000001).contains("ns"));
        assert!(format_duration_ms(0.5).contains("μs"));
        assert!(format_duration_ms(500.0).contains("ms"));
        assert!(format_duration_ms(1500.0).contains("s"));
    }

    #[test]
    fn test_word_wrap() {
        let wrapped = word_wrap("hello world foo bar", 10);
        assert!(wrapped.contains('\n'));
        assert!(wrapped.contains("hello"));
    }
}
