use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

use yew_base_components::components::color_scheme::ColorScheme;
use yew_base_components::components::message::message::Message;

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
    error_message: Option<String>,
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
            error_message: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Response(Ok(language_list)) => {
                self.language_list = Some(language_list);
                self.task = None;
            }
            Msg::Response(Err(error)) => {
                self.task = None;
                let message: String = format!("An error occurred: {:?}", error);
                self.error_message = Some(message);
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
                <div class="p-5 m-5 bg-white shadow-sm rounded">
                    <div class="p-2.5 pb-5 border-b mb-7 font-semibold text-gray-700">
                        {"Languages"}
                    </div>
            <table class="min-w-full divide-y divide-gray-700">
                <thead>
                    <tr class="bg-gray-50">
                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">{"Name"}</th>
                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">{"Language"}</th>
                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">{"Territory"}</th>
                        <th scope="col" class="relative px-6 py-a3"><span class="sr-only">{"Edit"}</span></th>
                    </tr>
                </thead>
                <tbody class="bg-white divide-y divide-gray-200">
            {for language_list.iter().map(|language| {
                let language_to_delete = language.clone();
                let onclick_delete = self.link.callback(move |ev| Msg::DeleteLanguage(language_to_delete.id.to_string()));
                html! {
                    <tr>
                        <td class="px-6 py-2 whitespace-nowrap text-sm">
                            <RouterAnchor<AppRoute> route=AppRoute::LanguageShow(language.id.to_string())> {&language.name} </RouterAnchor<AppRoute>>
                        </td>
                        <td class="px-6 py-2 whitespace-nowrap text-sm">
                            <RouterAnchor<AppRoute> route=AppRoute::LanguageShow(language.id.to_string())> {&language.lang} </RouterAnchor<AppRoute>>
                        </td>
                        <td class="px-6 py-2 whitespace-nowrap text-sm">
                            <RouterAnchor<AppRoute> route=AppRoute::LanguageShow(language.id.to_string())> {&language.territory} </RouterAnchor<AppRoute>>
                        </td>
                        <td class="px-6 py-2 whitespace-nowrap text-right text-sm">
                            <RouterAnchor<AppRoute> route=AppRoute::LanguageEdit(language.id.to_string()) classes="text-indigo-600 inline-block p-2 hover:text-indigo-900"> {"edit"} </RouterAnchor<AppRoute>>
                            <a href="#" class="text-indigo-600 inline-block p-2 hover:text-indigo-900" onclick=onclick_delete>{"delete"}</a>
                        </td>
                    </tr>
                }
                })}
                    </tbody>
                </table>
            <div class="p-5 text-right">
                <RouterAnchor<AppRoute> route=AppRoute::LanguageCreate classes="border shadow-sm text-white text-sm rounded p-2 bg-indigo-700"> {"add new languages"} </RouterAnchor<AppRoute>>
            </div>
            </div>
            }
        } else {
            if let Some(error_message) = &self.error_message {
                html! {
                    <Message text={error_message} color_scheme=ColorScheme::Error/>
                }
            } else {
                html! {}
            }
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.task = Some(self.languages.all(self.response.clone()))
        }
    }
}
