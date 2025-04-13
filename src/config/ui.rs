use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UiConfig {
    pub hyde:          bool,
    pub theme:         String,
    pub notify:        bool,
    pub nonicons:      bool,
    pub cursor_line:   bool,
    pub cursor_column: bool,
    pub indents:       usize,
    pub line_numbers:  LineNumbersConfig,
    pub font:          FontConfig
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LineNumbersConfig {
    pub auto_switch_relative: bool,
    pub numbers_enabled:      bool,
    pub relative_numbers:     bool
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FontConfig {
    pub family: String,
    pub size:   usize
}

impl UiConfig {
    pub fn default() -> Self {
        UiConfig {
            hyde:          false,
            theme:         "Catppuccin-Mocha".to_string(),
            notify:        true,
            nonicons:      true,
            cursor_line:   true,
            cursor_column: false,
            indents:       4,
            line_numbers:  LineNumbersConfig {
                auto_switch_relative: true,
                numbers_enabled:      true,
                relative_numbers:     true
            },
            font:          FontConfig {
                family: "JetBrainsMono Nerd Font".to_string(),
                size:   11
            }
        }
    }
}

impl Display for LineNumbersConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "LineNumbersConfig {{ auto_switch_relative: {}, numbers_enabled: {}, relative_numbers: {} }}",
            self.auto_switch_relative, self.numbers_enabled, self.relative_numbers
        )
    }
}

impl Display for FontConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "FontConfig {{ family: {}, size: {} }}",
            self.family, self.size
        )
    }
}

impl Display for UiConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "UiConfig {{ hyde: {}, theme: \"{}\", notify: {}, nonicons: {}, cursor_line: {}, cursor_column: {}, indents: {}, line_numbers: {}, font: {} }}",
            self.hyde,
            self.theme,
            self.notify,
            self.nonicons,
            self.cursor_line,
            self.cursor_column,
            self.indents,
            self.line_numbers,
            self.font
        )
    }
}
