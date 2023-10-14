use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

const ESCAPE_MARKDOWN: [char; 4] = ['_', '*', '`', '['];
const ESCAPE_MARKDOWN_V2: [char; 18] = [
    '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.', '!',
];

/// Send Markdown or HTML,
/// if you want Telegram apps to show
/// bold, italic, fixed-width text or
/// inline URLs in the media caption.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ParseMode {
    /// HTML
    #[serde(rename = "HTML")]
    Html,
    /// Markdown
    Markdown,
    /// MarkdownV2 style
    MarkdownV2,
}

impl ParseMode {
    /// Escape HTML or Markdown special characters
    ///
    /// For MarkdownV2 you must escape characters with `\` by yourself in the following cases:
    ///
    /// * all \` and `\` characters in 'pre' and 'code'.
    /// * `(...)` - part of inline link definition, all `)` and `\` characters.
    ///
    /// In all other places use this method to escape special characters.
    pub fn escape<T>(self, input: T) -> String
    where
        T: Into<String>,
    {
        let input = input.into();
        let mut result = String::new();
        match self {
            ParseMode::Html => {
                for i in input.chars() {
                    match i {
                        '<' => result += "&lt;",
                        '>' => result += "&gt;",
                        '&' => result += "&amp;",
                        _ => result.push(i),
                    }
                }
            }
            ParseMode::Markdown => {
                for i in input.chars() {
                    if ESCAPE_MARKDOWN.contains(&i) {
                        result.push('\\');
                    }
                    result.push(i);
                }
            }
            ParseMode::MarkdownV2 => {
                for i in input.chars() {
                    if ESCAPE_MARKDOWN_V2.contains(&i) {
                        result.push('\\');
                    }
                    result.push(i);
                }
            }
        }
        result
    }
}

impl fmt::Display for ParseMode {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(
            out,
            "{}",
            match self {
                ParseMode::Html => "HTML",
                ParseMode::Markdown => "Markdown",
                ParseMode::MarkdownV2 => "MarkdownV2",
            }
        )
    }
}
