use dotenv_codegen::dotenv;
use log::debug;
use serde::{Deserialize, Serialize};
use yew::callback::Callback;
use yew::format::{Json, Nothing, Text};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use crate::error::Error;
use crate::types::ErrorInfo;

const API_ROOT: &str = dotenv!("API_ROOT");

pub struct Requests {}

impl Default for Requests {
    fn default() -> Self {
        Self::new()
    }
}

impl Requests {
    pub fn new() -> Self {
        Self {}
    }

    pub fn builder<B, T>(
        &mut self,
        method: &str,
        url: String,
        body: B,
        callback: Callback<Result<T, Error>>,
    ) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
        B: Into<Text> + std::fmt::Debug,
    {
        let handler = move |response: Response<Text>| {
            if let (meta, Ok(data)) = response.into_parts() {
                debug!("Response: {:?}", data);

                if meta.status.is_success() {
                    let data: Result<T, _> = serde_json::from_str(&data);

                    if let Ok(data) = data {
                        callback.emit(Ok(data))
                    } else {
                        callback.emit(Err(Error::DeserializeError))
                    }
                } else {
                    match meta.status.as_u16() {
                        401 => callback.emit(Err(Error::Unauthorized)),
                        403 => callback.emit(Err(Error::Forbidden)),
                        404 => callback.emit(Err(Error::NotFound)),
                        500 => callback.emit(Err(Error::InternalServerError)),
                        422 => {
                            let data: Result<ErrorInfo, _> = serde_json::from_str(&data);
                            if let Ok(data) = data {
                                callback.emit(Err(Error::UnprocessableEntity(data)))
                            } else {
                                callback.emit(Err(Error::DeserializeError))
                            }
                        }
                        _ => callback.emit(Err(Error::RequestError)),
                    }
                }
            } else {
                callback.emit(Err(Error::RequestError))
            }
        };

        let url = format!("{}{}", API_ROOT, url);
        let builder = Request::builder()
            .method(method)
            .uri(url.as_str())
            .header("Content-Type", "application/json");

        let request = builder.body(body).unwrap();
        debug!("Requests: {:?}", request);

        FetchService::fetch(request, handler.into()).unwrap()
    }

    pub fn delete<T>(&mut self, url: String, callback: Callback<Result<T, Error>>) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
    {
        self.builder("DELETE", url, Nothing, callback)
    }

    pub fn get<T>(&mut self, url: String, callback: Callback<Result<T, Error>>) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
    {
        self.builder("GET", url, Nothing, callback)
    }

    pub fn post<B, T>(
        &mut self,
        url: String,
        body: B,
        callback: Callback<Result<T, Error>>,
    ) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
        B: Serialize,
    {
        let body: Text = Json(&body).into();
        self.builder("POST", url, body, callback)
    }

    pub fn put<B, T>(
        &mut self,
        url: String,
        body: B,
        callback: Callback<Result<T, Error>>,
    ) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
        B: Serialize,
    {
        let body: Text = Json(&body).into();
        self.builder("PUT", url, body, callback)
    }
}
