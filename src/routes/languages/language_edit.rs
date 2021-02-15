use yew::services::fetch::FetchTask;
use yew::{
    html, Callback, Component, ComponentLink, FocusEvent, Html, InputData, Properties, ShouldRender,
};
use yew_base_components::components::form::{input::Input, button::Button};

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
    UpdateId(String),
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
            Msg::UpdateId(id) => {
                self.request.id = id;
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
        let oninput_id = self
            .link
            .callback(|value: String| Msg::UpdateId(value));
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
                <Input label="Id".to_owned() value=&self.request.id.to_owned() onupdate=oninput_id/> 
                <Input label="Name".to_owned() value=&self.request.name.to_owned() onupdate=oninput_name/> 
                <Input label="Language".to_owned() value=&self.request.lang.to_owned() onupdate=oninput_lang/> 
                <Input label="Territory".to_owned() value=&self.request.territory.to_owned() onupdate=oninput_territory/> 
                <Button label="Update".to_owned()/>
                </form>
                </div>
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
