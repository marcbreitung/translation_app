use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

use crate::error::Error;
use crate::routes::AppRoute;
use crate::services::Languages;
use crate::types::{LanguageDelete, LanguageInfo};

pub enum Msg {
    Response(Result<Vec<LanguageInfo>, Error>),
    ResponseDelete(Result<LanguageDelete, Error>),
    DeleteLanguage(String),
}

pub struct LanguageList {
    languages: Languages,
    language_list: Option<Vec<LanguageInfo>>,
    response: Callback<Result<Vec<LanguageInfo>, Error>>,
    response_delete: Callback<Result<LanguageDelete, Error>>,
    task: Option<FetchTask>,
    link: ComponentLink<Self>,
}

impl Component for LanguageList {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            languages: Languages::new(),
            language_list: None,
            response: link.callback(Msg::Response),
            response_delete: link.callback(Msg::ResponseDelete),
            task: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Response(Ok(language_list)) => {
                self.language_list = Some(language_list);
                self.task = None;
            }
            Msg::Response(Err(_)) => {
                self.task = None;
            }
            Msg::ResponseDelete(Ok(_language_deleted)) => {
                self.task = Some(self.languages.all(self.response.clone()))
            }
            Msg::ResponseDelete(Err(_)) => {
                self.task = None;
            }
            Msg::DeleteLanguage(id) => {
                self.task = Some(self.languages.delete(id, self.response_delete.clone()));
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if let Some(language_list) = &self.language_list {
            html! {
                <div>
            <div class="collection">
            {for language_list.iter().map(|language| {
                let language_to_delete = language.clone();
                let onclick_delete = self.link.callback(move |ev| Msg::DeleteLanguage(language_to_delete.id.to_string()));
                html! {
                    <div class="collection-item">
                        <RouterAnchor<AppRoute> route=AppRoute::LanguageShow(language.id.to_string())> {&language.name} </RouterAnchor<AppRoute>>
                        <RouterAnchor<AppRoute> route=AppRoute::LanguageEdit(language.id.to_string())> {"edit"} </RouterAnchor<AppRoute>>
                        <a href="#" onclick=onclick_delete>{"delete"}</a>
                    </div>
                }
                })}
                </div>
            <div>
                <RouterAnchor<AppRoute> route=AppRoute::LanguageCreate classes="waves-effect waves-light btn"> {"add new languages"} </RouterAnchor<AppRoute>>
            </div>
            </div>
            }
        } else {
            html! {}
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.task = Some(self.languages.all(self.response.clone()))
        }
    }
}
