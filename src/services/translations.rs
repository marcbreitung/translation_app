use yew::callback::Callback;
use yew::services::fetch::FetchTask;

use super::Requests;
use crate::error::Error;
use crate::types::*;

pub struct Translations {
    requests: Requests,
}

impl Default for Translations {
    fn default() -> Self {
        Self::new()
    }
}

impl Translations {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    pub fn all(&mut self, callback: Callback<Result<Vec<TranslationInfo>, Error>>) -> FetchTask {
        self.requests
            .get::<Vec<TranslationInfo>>("/translations".to_owned(), callback)
    }

    pub fn get(
        &mut self,
        id: String,
        callback: Callback<Result<TranslationInfo, Error>>,
    ) -> FetchTask {
        self.requests
            .get::<TranslationInfo>(format!("/translations/{}", id), callback)
    }

    pub fn create(
        &mut self,
        translation: TranslationCrateUpdateInfo,
        callback: Callback<Result<TranslationInfo, Error>>,
    ) -> FetchTask {
        self.requests
            .post::<TranslationCrateUpdateInfo, TranslationInfo>(
                "/translations".to_owned(),
                translation,
                callback,
            )
    }

    pub fn update(
        &mut self,
        id: String,
        translation: TranslationInfo,
        callback: Callback<Result<TranslationInfo, Error>>,
    ) -> FetchTask {
        self.requests.put::<TranslationInfo, TranslationInfo>(
            format!("/translations/{}", id),
            translation,
            callback,
        )
    }

    pub fn delete(
        &mut self,
        id: String,
        callback: Callback<Result<TranslationDelete, Error>>,
    ) -> FetchTask {
        self.requests
            .delete::<TranslationDelete>(format!("/translations/{}", id), callback)
    }
}
