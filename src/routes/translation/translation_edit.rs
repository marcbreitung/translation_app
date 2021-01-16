use yew::services::fetch::FetchTask;
use yew::{
    html, Callback, Component, ComponentLink, FocusEvent, Html, InputData, Properties, ShouldRender,
};

use crate::error::Error;
use crate::services::Translations;
use crate::types::TranslationInfo;

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub id: Option<String>,
}

pub enum Msg {
    Request,
    RequestUpdate,
    Response(Result<TranslationInfo, Error>),
    UpdateKey(String),
    UpdateLanguage(String),
    UpdateTarget(String),
}

pub struct TranslationEdit {
    translations: Translations,
    response: Callback<Result<TranslationInfo, Error>>,
    request: TranslationInfo,
    task: Option<FetchTask>,
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for TranslationEdit {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            translations: Translations::new(),
            response: link.callback(Msg::Response),
            request: TranslationInfo::default(),
            task: None,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Response(Ok(translation)) => {
                self.request = translation;
                self.task = None;
            }
            Msg::Response(Err(_)) => {
                self.task = None;
            }
            Msg::Request => {
                if let Some(id) = &self.props.id {
                    self.task = Some(self.translations.get(id.clone(), self.response.clone()))
                }
            }
            Msg::RequestUpdate => {
                if let Some(id) = &self.props.id {
                    self.task = Some(self.translations.update(
                        id.to_string(),
                        self.request.clone(),
                        self.response.clone(),
                    ));
                }
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
        let oninput_key = self.link.callback(|ev: InputData| Msg::UpdateKey(ev.value));
        let oninput_language = self
            .link
            .callback(|ev: InputData| Msg::UpdateLanguage(ev.value));
        let oninput_target = self
            .link
            .callback(|ev: InputData| Msg::UpdateTarget(ev.value));
        html! {
            <article>
                <form onsubmit=onsubmit>
                <div><label>{"Id:"}</label><input type="text" value={&self.request.id} readonly=true/></div>
                <div><label>{"Key:"}</label><input type="text" value={&self.request.key} oninput=oninput_key/></div>
                <div><label>{"Language:"}</label><input type="text" value={&self.request.language} oninput=oninput_language/></div>
                <div><label>{"Target:"}</label><input type="text" value={&self.request.target} oninput=oninput_target/></div>

                <div><button type="submit">{"Update"}</button></div>
                </form>
                </article>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(id) = &self.props.id {
                self.task = Some(self.translations.get(id.clone(), self.response.clone()))
            }
        }
    }
}
