use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LspConfig {
    pub format_before_save: bool,
    pub inlay_hints:        bool,
    pub code_lenses:        bool,
    pub completion:         CompletionConfig,
    pub diagnostic:         DiagnosticConfig
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CompletionConfig {
    pub auto: bool
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DiagnosticConfig {
    pub enable_d_signs: bool,
    pub virtual_text:   bool,
    pub show_on_hover:  bool
}

impl LspConfig {
    pub fn default() -> Self {
        LspConfig {
            format_before_save: false,
            inlay_hints:        true,
            code_lenses:        true,
            completion:         CompletionConfig {
                auto: true
            },
            diagnostic:         DiagnosticConfig {
                enable_d_signs: true,
                virtual_text:   false,
                show_on_hover:  false
            }
        }
    }
}

impl Display for CompletionConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "CompletionConfig {{ auto: {} }}", self.auto)
    }
}

impl Display for DiagnosticConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "DiagnosticConfig {{ enable_d_signs: {}, virtual_text: {}, show_on_hover: {} }}",
            self.enable_d_signs, self.virtual_text, self.show_on_hover
        )
    }
}

impl Display for LspConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "LspConfig {{ format_before_save: {}, inlay_hints: {}, code_lenses: {}, completion: {}, diagnostic: {} }}",
            self.format_before_save,
            self.inlay_hints,
            self.code_lenses,
            self.completion,
            self.diagnostic
        )
    }
}
