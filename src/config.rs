mod basic;
mod git;
mod lsp;
mod ui;

use std::fmt::Debug;

use git::GitConfig;
use lsp::LspConfig;
use nvim_oxi::api::{get_var, set_var};
use serde::Serialize;
use serde_json::{Value, from_str, to_value};
use ui::UiConfig;

use crate::{
    config::basic::BasicConfig,
    error::{NeviraideError, NeviraideResult}
};

#[derive(Debug, Serialize)]
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
        self.apply_recursive("", self)?;
        Ok(())
    }

    fn apply_recursive<T: Serialize>(&self, prefix: &str, config: &T) -> NeviraideResult<()> {
        for (key, value) in self.get_fields(config) {
            let full_key = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{}.{}", prefix, key)
            };

            if let Some(_) = value.as_object() {
                self.apply_recursive(&full_key, &value)?;
            } else {
                self.set_var(&full_key, value)?;
            }
        }
        Ok(())
    }

    fn get_fields<T: Serialize>(&self, config: &T) -> Vec<(String, Value)> {
        let mut fields = Vec::new();

        if let Ok(serialized) = to_value(config) {
            if let Some(object) = serialized.as_object() {
                for (key, value) in object {
                    fields.push((key.clone(), value.clone()));
                }
            }
        }

        fields
    }

    fn set_var<T: ToString + Debug>(&self, key: &str, value: T) -> NeviraideResult<()> {
        let value_str = value.to_string();

        let value_str = if let Ok(vec) = from_str::<Vec<String>>(&value_str) {
            let lua_table = format!(
                "{{{}}}",
                vec.iter()
                    .map(|s| format!("\"{}\"", s))
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            lua_table
        } else {
            value_str
        };

        let value_str = if value_str.starts_with("\"") && value_str.ends_with("\"") {
            value_str[1..value_str.len() - 1].to_string()
        } else {
            value_str
        };

        set_var(key, value_str).map_err(|e| {
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

    let language: String = get_var("basic.language")?;
    assert_eq!(language, "ru");

    let latest_stable_plugins: String = get_var("basic.latest_stable_plugins")?;
    let latest_stable_plugins = latest_stable_plugins == "true";
    assert_eq!(latest_stable_plugins, true);

    let programming: String = get_var("basic.programming")?;
    assert_eq!(programming, "{\"lua\", \"rust\"}");

    let git_enabled: String = get_var("git.gitsigns.enable_g_signs")?;
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
