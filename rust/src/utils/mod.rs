//! Utils module

/// ANSI color codes
pub fn green(s: &str) -> String { format!("\x1b[32m{s}\x1b[0m") }
pub fn red(s: &str) -> String { format!("\x1b[31m{s}\x1b[0m") }
pub fn yellow(s: &str) -> String { format!("\x1b[33m{s}\x1b[0m") }
pub fn cyan(s: &str) -> String { format!("\x1b[36m{s}\x1b[0m") }
pub fn blue(s: &str) -> String { format!("\x1b[34m{s}\x1b[0m") }
pub fn bold(s: &str) -> String { format!("\x1b[1m{s}\x1b[0m") }

/// Format duration in human-readable form
pub fn format_duration_ms(ms: f64) -> String {
    if ms < 1e-3 { format!("{:.2}ns", ms * 1e6) }
    else if ms < 1.0 { format!("{:.2}μs", ms * 1000.0) }
    else if ms < 1000.0 { format!("{:.2}ms", ms) }
    else { format!("{:.2}s", ms / 1000.0) }
}

/// Truncate string with ellipsis
pub fn truncate(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len { s } else { &s[..max_len.saturating_sub(3)] }
}

/// Calculate Levenshtein distance
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

/// Find best fuzzy match
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
