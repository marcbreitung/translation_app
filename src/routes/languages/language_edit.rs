use yew::services::fetch::FetchTask;
use yew::{
    html, Callback, Component, ComponentLink, FocusEvent, Html, InputData, Properties, ShouldRender,
};

use crate::error::Error;
use crate::services::Languages;
use crate::types::LanguageInfo;

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub id: Option<String>,
}

pub enum Msg {
    Request,
    RequestUpdate,
    Response(Result<LanguageInfo, Error>),
    UpdateName(String),
    UpdateLang(String),
    UpdateTerritory(String),
}

pub struct LanguageEdit {
    languages: Languages,
    response: Callback<Result<LanguageInfo, Error>>,
    request: LanguageInfo,
    task: Option<FetchTask>,
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for LanguageEdit {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            languages: Languages::new(),
            response: link.callback(Msg::Response),
            request: LanguageInfo::default(),
            task: None,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Response(Ok(language)) => {
                self.request = language;
                self.task = None;
            }
            Msg::Response(Err(_)) => {
                self.task = None;
            }
            Msg::Request => {
                if let Some(id) = &self.props.id {
                    self.task = Some(self.languages.get(id.clone(), self.response.clone()))
                }
            }
            Msg::RequestUpdate => {
                if let Some(id) = &self.props.id {
                    self.task = Some(self.languages.update(
                        id.to_string(),
                        self.request.clone(),
                        self.response.clone(),
                    ));
                }
            }
            Msg::UpdateName(name) => {
                self.request.name = name;
            }
            Msg::UpdateLang(lang) => {
                self.request.lang = lang;
            }
            Msg::UpdateTerritory(terrirory) => {
                self.request.territory = terrirory;
            }
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
            Msg::RequestUpdate
        });
        let oninput_name = self.link.callback(|ev: InputData| Msg::UpdateName(ev.value));
        let oninput_lang = self
            .link
            .callback(|ev: InputData| Msg::UpdateLang(ev.value));
        let oninput_territory = self
            .link
            .callback(|ev: InputData| Msg::UpdateTerritory(ev.value));
        html! {
            <article>
                <form onsubmit=onsubmit>
                <div><label>{"Id:"}</label><input type="text" value={&self.request.id} readonly=true/></div>
                <div><label>{"Name:"}</label><input type="text" value={&self.request.name} oninput=oninput_name/></div>
                <div><label>{"Lang:"}</label><input type="text" value={&self.request.lang} oninput=oninput_lang/></div>
                <div><label>{"Territory:"}</label><input type="text" value={&self.request.territory} oninput=oninput_territory/></div>

                <div><button type="submit" class="waves-effect waves-light btn">{"Update"}</button></div>
                </form>
                </article>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(id) = &self.props.id {
                self.task = Some(self.languages.get(id.clone(), self.response.clone()))
            }
        }
    }
}
