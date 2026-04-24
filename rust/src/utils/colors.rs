//! ANSI color utilities for terminal output

/// ANSI color codes
pub mod codes {
    pub const RESET: u8 = 0;
    pub const BOLD: u8 = 1;
    pub const RED: u8 = 31;
    pub const GREEN: u8 = 32;
    pub const YELLOW: u8 = 33;
    pub const CYAN: u8 = 36;
}

/// Apply ANSI color code to a string
#[inline]
pub fn ansi_color(s: &str, code: u8) -> String {
    format!("\x1b[{}m{}\x1b[{}m", code, s, codes::RESET)
}

/// Apply bold formatting
#[inline]
pub fn bold(s: &str) -> String {
    format!("\x1b[{}m{}\x1b[{}m", codes::BOLD, s, codes::RESET)
}

/// Green colored string (code 32)
#[inline]
pub fn green(s: &str) -> String {
    ansi_color(s, codes::GREEN)
}

/// Red colored string (code 31)
#[inline]
pub fn red(s: &str) -> String {
    ansi_color(s, codes::RED)
}

/// Yellow colored string (code 33)
#[inline]
pub fn yellow(s: &str) -> String {
    ansi_color(s, codes::YELLOW)
}

/// Cyan colored string (code 36)
#[inline]
pub fn cyan(s: &str) -> String {
    ansi_color(s, codes::CYAN)
}

/// Styled string builder for ergonomic terminal output
pub struct StyledString {
    content: String,
}

impl StyledString {
    pub fn new(s: &str) -> Self {
        Self {
            content: s.to_string(),
        }
    }

    pub fn bold(self) -> Self {
        Self {
            content: format!("\x1b[{}m{}\x1b[{}m", codes::BOLD, self.content, codes::RESET),
        }
    }

    pub fn green(self) -> Self {
        Self {
            content: green(&self.content),
        }
    }

    pub fn red(self) -> Self {
        Self {
            content: red(&self.content),
        }
    }

    pub fn yellow(self) -> Self {
        Self {
            content: yellow(&self.content),
        }
    }

    pub fn cyan(self) -> Self {
        Self {
            content: cyan(&self.content),
        }
    }

    pub fn bg(self, code: u8) -> Self {
        Self {
            content: format!("\x1b[{}m{}\x1b[{}m", code + 10, self.content, codes::RESET),
        }
    }
}

impl From<&str> for StyledString {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for StyledString {
    fn from(s: String) -> Self {
        Self { content: s }
    }
}

impl std::fmt::Display for StyledString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_functions() {
        assert!(green("test").contains("test"));
        assert!(red("test").contains("test"));
        assert!(yellow("test").contains("test"));
        assert!(cyan("test").contains("test"));
    }

    #[test]
    fn test_ansi_color_codes() {
        assert!(green("test").starts_with("\x1b[32m"));
        assert!(red("test").starts_with("\x1b[31m"));
        assert!(yellow("test").starts_with("\x1b[33m"));
        assert!(cyan("test").starts_with("\x1b[36m"));
    }
}
