use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GitConfig {
    pub gitsigns: GitsignsConfig
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitsignsConfig {
    pub enable_g_signs:            bool,
    pub enable_current_line_blame: bool
}

impl GitConfig {
    pub fn default() -> Self {
        GitConfig {
            gitsigns: GitsignsConfig {
                enable_g_signs:            true,
                enable_current_line_blame: false
            }
        }
    }
}

impl Display for GitsignsConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "GitsignsConfig {{ enable_g_signs: {}, enable_current_line_blame: {} }}",
            self.enable_g_signs, self.enable_current_line_blame
        )
    }
}

impl Display for GitConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "GitConfig {{ gitsigns: {} }}", self.gitsigns)
    }
}
