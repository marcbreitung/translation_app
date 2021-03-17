use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

use yew_base_components::components::color_scheme::ColorScheme;
use yew_base_components::components::message::message::Message;

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
    error_message: Option<String>,
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
            error_message: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Response(Ok(translation_list)) => {
                self.translation_list = Some(translation_list);
                self.task = None;
            }
            Msg::Response(Err(error)) => {
                self.task = None;
                let message: String = format!("An error occurred: {:?}", error);
                self.error_message = Some(message);
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
                <div class="p-2.5 pb-5 border-b mb-7 font-semibold text-gray-700">
                    {"Translations"}
                </div>

            <table class="min-w-full divide-y divide-gray-700">
                <thead>
                    <tr class="bg-gray-50">
                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">{"Key"}</th>
                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">{"Language"}</th>
                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">{"Target"}</th>
                        <th scope="col" class="relative px-6 py-3"><span class="sr-only">{"Edit"}</span></th>
                    </tr>
                </thead>
                <tbody class="bg-white divide-y divide-gray-200">
            {for translation_list.iter().map(|translation| {
                let translation_to_delete = translation.clone();
                let onclick_delete = self.link.callback(move |ev| Msg::DeleteTranslation(translation_to_delete.id.to_string()));
                html! {
                    <tr>
                        <td class="px-6 py-2 whitespace-nowrap text-sm">
                            <RouterAnchor<AppRoute> route=AppRoute::TranslationShow(translation.id.to_string())> {&translation.key} </RouterAnchor<AppRoute>>
                        </td>
                        <td class="px-6 py-2 whitespace-nowrap text-sm">
                            <RouterAnchor<AppRoute> route=AppRoute::TranslationEdit(translation.id.to_string())> {&translation.language} </RouterAnchor<AppRoute>>
                        </td>
                        <td class="px-6 py-2 whitespace-nowrap text-sm">
                            <RouterAnchor<AppRoute> route=AppRoute::TranslationEdit(translation.id.to_string())> {&translation.target} </RouterAnchor<AppRoute>>
                        </td>
                        <td class="px-6 py-2 whitespace-nowrap text-right text-sm">
                            <RouterAnchor<AppRoute> route=AppRoute::TranslationEdit(translation.id.to_string()) classes="text-indigo-600 inline-block p-2 hover:text-indigo-900"> {"edit"} </RouterAnchor<AppRoute>>
                            <a href="#" class="text-indigo-600 inline-block p-2 hover:text-indigo-900" onclick=onclick_delete>{"delete"}</a>
                        </td>
                    </tr>
                }
                })}
                </tbody>
                </table>
            <div class="p-5 text-right">
                <RouterAnchor<AppRoute> route=AppRoute::TranslationCreate classes="border shadow-sm text-white text-sm rounded p-2 px-4 bg-indigo-700"> {"add new translations"} </RouterAnchor<AppRoute>>
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
            self.task = Some(self.translations.all(self.response.clone()))
        }
    }
}
