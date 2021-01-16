mod translations;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use translations::{TranslationCrateUpdateInfo, TranslationDelete, TranslationInfo};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}
