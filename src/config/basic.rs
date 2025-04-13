use std::fmt::{Display, Formatter, Result as FmtResult};

use nvim_oxi::{
    Object,
    conversion::{Error as ConversionError, FromObject, ToObject},
    serde::{Deserializer, Serializer}
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BasicConfig {
    pub language:              String,
    pub latest_stable_plugins: bool,
    pub programming:           Vec<String>
}

impl BasicConfig {
    pub fn default() -> Self {
        BasicConfig {
            language:              "ru".to_string(),
            latest_stable_plugins: true,
            programming:           vec!["lua".to_string(), "rust".to_string()]
        }
    }
}

impl FromObject for BasicConfig {
    fn from_object(obj: Object) -> Result<Self, ConversionError> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

impl ToObject for BasicConfig {
    fn to_object(self) -> Result<Object, ConversionError> {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}

impl Display for BasicConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "BasicConfig {{ language: {}, latest_stable_plugins: {}, programming: {:?} }}",
            self.language, self.latest_stable_plugins, self.programming
        )
    }
}
