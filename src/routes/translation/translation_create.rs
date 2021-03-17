use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink, FocusEvent, Html, Properties,
    ShouldRender,
};
use yew_base_components::components::form::{button::Button, input::Input};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::error::Error;
use crate::routes::AppRoute;
use crate::services::Translations;
use crate::types::{TranslationCrateUpdateInfo, TranslationInfo};

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub id: Option<String>,
}

pub enum Msg {
    Request,
    Response(Result<TranslationInfo, Error>),
    UpdateKey(String),
    UpdateLanguage(String),
    UpdateTarget(String),
    Ignore,
}

pub struct TranslationCreate {
    translations: Translations,
    response: Callback<Result<TranslationInfo, Error>>,
    request: TranslationCrateUpdateInfo,
    task: Option<FetchTask>,
    props: Props,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
}

impl Component for TranslationCreate {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            translations: Translations::new(),
            response: link.callback(Msg::Response),
            request: TranslationCrateUpdateInfo::default(),
            task: None,
            props,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Response(Ok(translation)) => {
                self.task = None;
                self.router_agent.send(ChangeRoute(
                    AppRoute::TranslationEdit(translation.id).into(),
                ));
            }
            Msg::Response(Err(_)) => {
                self.task = None;
            }
            Msg::Request => {
                self.task = Some(
                    self.translations
                        .create(self.request.clone(), self.response.clone()),
                );
            }
            Msg::UpdateKey(key) => {
                self.request.key = key;
            }
            Msg::UpdateLanguage(language) => {
                self.request.language = language;
            }
            Msg::UpdateTarget(target) => {
                self.request.target = target;
            }
            Msg::Ignore => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onsubmit = self.link.callback(|event: FocusEvent| {
            event.prevent_default();
            Msg::Request
        });
        let oninput_key = self.link.callback(|value: String| Msg::UpdateKey(value));
        let oninput_language = self
            .link
            .callback(|value: String| Msg::UpdateLanguage(value));
        let oninput_target = self.link.callback(|value: String| Msg::UpdateTarget(value));
        html! {
            <div class="p-5 m-5 bg-white shadow-sm rounded">
                <form onsubmit=onsubmit>
                 <Input label="Key:".to_owned() value=&self.request.key.to_owned() onupdate=oninput_key/>
                 <Input label="Language:".to_owned() value=&self.request.language.to_owned() onupdate=oninput_language/>
                <Input label="Target:".to_owned() value=&self.request.target.to_owned() onupdate=oninput_target/>
                <Button label="Update".to_owned()/>
                </form>
            </div>
        }
    }
}
