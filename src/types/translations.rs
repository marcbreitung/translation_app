use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TranslationInfo {
    pub id: String,
    pub key: String,
    pub target: String,
    pub language: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TranslationCrateUpdateInfo {
    pub key: String,
    pub target: String,
    pub language: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TranslationDelete {
    pub id: String,
    pub status: u32,
}
