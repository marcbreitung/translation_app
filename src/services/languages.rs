
use yew::callback::Callback;
use yew::services::fetch::FetchTask;

use super::Requests;
use crate::error::Error;
use crate::types::*;

pub struct Languages {
    requests: Requests,
}

impl Default for Languages {
    fn default() -> Self {
        Self::new()
    }
}

impl Languages {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    pub fn all(&mut self, callback: Callback<Result<Vec<LanguageInfo>, Error>>) -> FetchTask {
        self.requests
            .get::<Vec<LanguageInfo>>("/languages".to_owned(), callback)
    }

    pub fn get(
        &mut self,
        id: String,
        callback: Callback<Result<LanguageInfo, Error>>,
    ) -> FetchTask {
        self.requests
            .get::<LanguageInfo>(format!("/languages/{}", id), callback)
    }

    pub fn create(
        &mut self,
        language: LanguageCrateUpdateInfo,
        callback: Callback<Result<LanguageInfo, Error>>,
    ) -> FetchTask {
        self.requests
            .post::<LanguageCrateUpdateInfo, LanguageInfo>(
                "/languages".to_owned(),
                language,
                callback,
            )
    }

    pub fn update(
        &mut self,
        id: String,
        language: LanguageInfo,
        callback: Callback<Result<LanguageInfo, Error>>,
    ) -> FetchTask {
        self.requests.put::<LanguageInfo, LanguageInfo>(
            format!("/languages/{}", id),
            language,
            callback,
        )
    }

    pub fn delete(
        &mut self,
        id: String,
        callback: Callback<Result<LanguageDelete, Error>>,
    ) -> FetchTask {
        self.requests
            .delete::<LanguageDelete>(format!("/languages/{}", id), callback)
    }
}
