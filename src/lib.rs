mod config;
mod error;

use config::NeviraideConfig;
use nvim_oxi::plugin;

use crate::error::NeviraideResult;

#[plugin]
fn neviraide_core() -> NeviraideResult<()> {
    let config = NeviraideConfig::new();
    config.apply()?;

    Ok(())
}
