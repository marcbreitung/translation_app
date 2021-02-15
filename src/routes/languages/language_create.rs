use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink, FocusEvent, Html, InputData,
    Properties, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use yew_base_components::components::form::{input::Input, button::Button};

use crate::error::Error;
use crate::routes::AppRoute;
use crate::services::Languages;
use crate::types::{LanguageCrateUpdateInfo, LanguageInfo};

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub id: Option<String>,
}

pub enum Msg {
    Request,
    Response(Result<LanguageInfo, Error>),
    UpdateName(String),
    UpdateLang(String),
    UpdateTerritory(String),
    Ignore,
}

pub struct LanguageCreate {
    languages: Languages,
    response: Callback<Result<LanguageInfo, Error>>,
    request: LanguageCrateUpdateInfo,
    task: Option<FetchTask>,
    props: Props,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
}

impl Component for LanguageCreate {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            languages: Languages::new(),
            response: link.callback(Msg::Response),
            request: LanguageCrateUpdateInfo::default(),
            task: None,
            props,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Response(Ok(language)) => {
                self.task = None;
                self.router_agent
                    .send(ChangeRoute(AppRoute::LanguageEdit(language.id).into()));
            }
            Msg::Response(Err(_)) => {
                self.task = None;
            }
            Msg::Request => {
                self.task = Some(
                    self.languages
                        .create(self.request.clone(), self.response.clone()),
                );
            }
            Msg::UpdateName(name) => {
                self.request.name = name;
            }
            Msg::UpdateLang(lang) => {
                self.request.lang = lang;
            }
            Msg::UpdateTerritory(territory) => {
                self.request.territory = territory;
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
        let oninput_name = self
            .link
            .callback(|value: String| Msg::UpdateName(value));
        let oninput_lang = self
            .link
            .callback(|value: String| Msg::UpdateLang(value));
        let oninput_territory = self
            .link
            .callback(|value: String| Msg::UpdateTerritory(value));
        html! {
            <div class="p-5 m-5 bg-white shadow-sm rounded">
                <form onsubmit=onsubmit>
                <Input label="Name".to_owned() value=&self.request.name.to_owned() onupdate=oninput_name/>
                <Input label="Language".to_owned() value=&self.request.lang.to_owned() onupdate=oninput_lang/>
                <Input label="Territory".to_owned() value=&self.request.territory.to_owned() onupdate=oninput_territory/>
                <Button label="Add".to_owned()/>
                </form>
            </div>
        }
    }
}
