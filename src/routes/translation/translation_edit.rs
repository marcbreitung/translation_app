use yew::services::fetch::FetchTask;
use yew::{
    html, Callback, Component, ComponentLink, FocusEvent, Html, Properties, ShouldRender,
};
use yew_base_components::components::form::{input::Input, button::Button};

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
    UpdateId(String),
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
            Msg::UpdateId(key) => {
                self.request.key = key;
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
        let oninput_id = self.link.callback(|value: String| Msg::UpdateId(value));
        let oninput_key = self.link.callback(|value: String| Msg::UpdateKey(value));
        let oninput_language = self
            .link
            .callback(|value: String| Msg::UpdateLanguage(value));
        let oninput_target = self
            .link
            .callback(|value: String| Msg::UpdateTarget(value));
        html! {
            <div class="p-5 m-5 bg-white shadow-sm rounded">
                <form onsubmit=onsubmit>
                <Input label="Id:".to_owned() value=&self.request.id.to_owned() onupdate=oninput_id/>
                <Input label="Key:".to_owned() value=&self.request.key.to_owned() onupdate=oninput_key/>
                <Input label="Language:".to_owned() value=&self.request.language.to_owned() onupdate=oninput_language/>
                <Input label="Target:".to_owned() value=&self.request.target.to_owned() onupdate=oninput_target/>
                <Button label="Update".to_owned()/>
                </form>
            </div>
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
