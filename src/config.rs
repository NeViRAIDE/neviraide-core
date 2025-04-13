mod basic;
mod git;
mod lsp;
mod ui;

use std::fmt::Debug;

use git::GitConfig;
use lsp::LspConfig;
use nvim_oxi::api::{get_var, set_var};
use serde::Serialize;
use ui::UiConfig;

use crate::{
    config::basic::BasicConfig,
    error::{NeviraideError, NeviraideResult}
};

#[derive(Debug, Default, Serialize)]
pub struct NeviraideConfig {
    pub basic: BasicConfig,
    pub git:   GitConfig,
    pub lsp:   LspConfig,
    pub ui:    UiConfig
}

impl NeviraideConfig {
    pub fn new() -> Self {
        NeviraideConfig {
            basic: BasicConfig::default(),
            git:   GitConfig::default(),
            lsp:   LspConfig::default(),
            ui:    UiConfig::default()
        }
    }

    pub fn apply(&self) -> NeviraideResult<()> {
        self.apply_basic()?;
        self.apply_git()?;
        self.apply_lsp()?;
        self.apply_ui()?;
        Ok(())
    }

    fn apply_basic(&self) -> NeviraideResult<()> {
        self.set_var("language", &self.basic.language)?;
        self.set_var("latest_stable_plugins", self.basic.latest_stable_plugins)?;
        self.set_var("programming", self.basic.programming.join(","))?;
        Ok(())
    }

    fn apply_git(&self) -> NeviraideResult<()> {
        self.set_var("git.enable_g_signs", self.git.gitsigns.enable_g_signs)?;
        self.set_var(
            "git.enable_current_line_blame",
            self.git.gitsigns.enable_current_line_blame
        )?;
        Ok(())
    }

    fn apply_lsp(&self) -> NeviraideResult<()> {
        self.set_var("lsp.format_before_save", self.lsp.format_before_save)?;
        self.set_var("lsp.inlay_hints", self.lsp.inlay_hints)?;
        self.set_var("lsp.code_lenses", self.lsp.code_lenses)?;
        self.set_var("lsp.completion.auto", self.lsp.completion.auto)?;
        self.set_var(
            "lsp.diagnostic.enable_d_signs",
            self.lsp.diagnostic.enable_d_signs
        )?;
        self.set_var(
            "lsp.diagnostic.virtual_text",
            self.lsp.diagnostic.virtual_text
        )?;
        self.set_var(
            "lsp.diagnostic.show_on_hover",
            self.lsp.diagnostic.show_on_hover
        )?;
        Ok(())
    }

    fn apply_ui(&self) -> NeviraideResult<()> {
        self.set_var("ui.hyde", self.ui.hyde)?;
        self.set_var("ui.theme", &self.ui.theme)?;
        self.set_var("ui.notify", self.ui.notify)?;
        self.set_var("ui.nonicons", self.ui.nonicons)?;
        self.set_var("ui.cursor_line", self.ui.cursor_line)?;
        self.set_var("ui.cursor_column", self.ui.cursor_column)?;
        self.set_var("ui.indents", self.ui.indents)?;
        self.set_var(
            "ui.line_numbers.auto_switch_relative",
            self.ui.line_numbers.auto_switch_relative
        )?;
        self.set_var(
            "ui.line_numbers.numbers_enabled",
            self.ui.line_numbers.numbers_enabled
        )?;
        self.set_var(
            "ui.line_numbers.relative_numbers",
            self.ui.line_numbers.relative_numbers
        )?;
        self.set_var("ui.font.family", &self.ui.font.family)?;
        self.set_var("ui.font.size", self.ui.font.size)?;
        Ok(())
    }

    fn set_var<T: ToString + Debug>(&self, key: &str, value: T) -> NeviraideResult<()> {
        set_var(key, value.to_string()).map_err(|e| {
            eprintln!(
                "Failed to set var: {} with value: {:?}, error: {:?}",
                key, value, e
            );
            NeviraideError::OxiApiError(format!("Failed to set var: {:?}", e))
        })?;
        Ok(())
    }
}

#[nvim_oxi::test]
fn test_neviraide_config() -> NeviraideResult<()> {
    let config = NeviraideConfig::new();
    config.apply()?;
    println!("config: {config:?}");

    let language: String = get_var("language")?;
    assert_eq!(language, "ru");

    let latest_stable_plugins: String = get_var("latest_stable_plugins")?;
    let latest_stable_plugins = latest_stable_plugins == "true";
    assert_eq!(latest_stable_plugins, true);

    let programming: String = get_var("programming")?;
    assert_eq!(programming, "lua,rust");

    let git_enabled: String = get_var("git.enable_g_signs")?;
    let git_enabled = git_enabled == "true";
    assert_eq!(git_enabled, true);

    let lsp_format_before_save: String = get_var("lsp.format_before_save")?;
    let lsp_format_before_save = lsp_format_before_save == "true";
    assert_eq!(lsp_format_before_save, false);

    let font_family: String = get_var("ui.font.family")?;
    assert_eq!(font_family, "JetBrainsMono Nerd Font");

    let font_size_str: String = get_var("ui.font.size")?;
    let font_size: i32 = font_size_str
        .parse()
        .map_err(|e| NeviraideError::OxiApiError(format!("Failed to parse font size: {:?}", e)))?;
    assert_eq!(font_size, 11);

    Ok(())
}
