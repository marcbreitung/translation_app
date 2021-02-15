use crate::error::Error;
use crate::services::Translations;
use crate::types::TranslationInfo;
use log::debug;
use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub id: Option<String>,
}

pub enum Msg {
    Response(Result<TranslationInfo, Error>),
}

pub struct TranslationShow {
    translations: Translations,
    translation: Option<TranslationInfo>,
    response: Callback<Result<TranslationInfo, Error>>,
    task: Option<FetchTask>,
    props: Props,
}

impl Component for TranslationShow {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            translations: Translations::new(),
            translation: None,
            response: link.callback(Msg::Response),
            task: None,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Response(Ok(translation)) => {
                self.translation = Some(translation);
                self.task = None;
            }
            Msg::Response(Err(_)) => {
                self.task = None;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        if let Some(translation) = &self.translation {
            html! {
                <div class="p-5 m-5 bg-white shadow-sm rounded">
                <article>
                    <h1>{&translation.id}</h1>
                    <h2>{&translation.key}</h2>
                    <p>{&translation.language}</p>
                    <p>{&translation.target}</p>
                </article>
                </div>
            }
        } else {
            html! {}
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(id) = &self.props.id {
                debug!("{}", id);
                self.task = Some(self.translations.get(id.clone(), self.response.clone()))
            }
        }
    }
}
