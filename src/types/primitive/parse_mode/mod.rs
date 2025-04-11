use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

const ESCAPE_MARKDOWN: [char; 4] = ['_', '*', '`', '['];
const ESCAPE_MARKDOWN_V2: [char; 18] = [
    '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.', '!',
];

/// Represents a mode for parsing entities in a text.
///
/// See [formatting options][1] for more details.
///
/// [1]: https://core.telegram.org/bots/api#formatting-options
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
    /// Escapes HTML or Markdown special characters.
    ///
    /// # Arguments
    ///
    /// * `input` - String that will be escaped.
    ///
    /// For MarkdownV2 you must escape characters with `\` by yourself in the following cases:
    ///
    /// - all \` and `\` characters in 'pre' and 'code'.
    /// - `(...)` - part of inline link definition, all `)` and `\` characters.
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
