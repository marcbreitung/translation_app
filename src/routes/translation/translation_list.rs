use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

use crate::error::Error;
use crate::routes::AppRoute;
use crate::services::Translations;
use crate::types::{TranslationDelete, TranslationInfo};

pub enum Msg {
    Response(Result<Vec<TranslationInfo>, Error>),
    ResponseDelete(Result<TranslationDelete, Error>),
    DeleteTranslation(String),
}

pub struct TranslationList {
    translations: Translations,
    translation_list: Option<Vec<TranslationInfo>>,
    response: Callback<Result<Vec<TranslationInfo>, Error>>,
    response_delete: Callback<Result<TranslationDelete, Error>>,
    task: Option<FetchTask>,
    link: ComponentLink<Self>,
}

impl Component for TranslationList {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            translations: Translations::new(),
            translation_list: None,
            response: link.callback(Msg::Response),
            response_delete: link.callback(Msg::ResponseDelete),
            task: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Response(Ok(translation_list)) => {
                self.translation_list = Some(translation_list);
                self.task = None;
            }
            Msg::Response(Err(_)) => {
                self.task = None;
            }
            Msg::ResponseDelete(Ok(_translation_deleted)) => {
                self.task = Some(self.translations.all(self.response.clone()))
            }
            Msg::ResponseDelete(Err(_)) => {
                self.task = None;
            }
            Msg::DeleteTranslation(id) => {
                self.task = Some(self.translations.delete(id, self.response_delete.clone()));
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if let Some(translation_list) = &self.translation_list {
            html! {
                <div class="p-5 m-5 bg-white shadow-sm rounded">
            <div class="collection">
            {for translation_list.iter().map(|translation| {
                let translation_to_delete = translation.clone();
                let onclick_delete = self.link.callback(move |ev| Msg::DeleteTranslation(translation_to_delete.id.to_string()));
                html! {
                    <div class="collection-item">
                        <RouterAnchor<AppRoute> route=AppRoute::TranslationShow(translation.id.to_string())> {&translation.key} </RouterAnchor<AppRoute>>
                        <RouterAnchor<AppRoute> route=AppRoute::TranslationEdit(translation.id.to_string())> {"edit"} </RouterAnchor<AppRoute>>
                        <a href="#" onclick=onclick_delete>{"delete"}</a>
                    </div>
                }
                })}
                </div>
            <div>
                <RouterAnchor<AppRoute> route=AppRoute::TranslationCreate classes="waves-effect waves-light btn"> {"add new translations"} </RouterAnchor<AppRoute>>
            </div>
            </div>
            }
        } else {
            html! {}
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.task = Some(self.translations.all(self.response.clone()))
        }
    }
}
