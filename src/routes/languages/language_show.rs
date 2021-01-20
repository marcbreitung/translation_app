use crate::error::Error;
use crate::services::Languages;
use crate::types::LanguageInfo;
use log::debug;
use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub id: Option<String>,
}

pub enum Msg {
    Response(Result<LanguageInfo, Error>),
}

pub struct LanguageShow {
    languages: Languages,
    language: Option<LanguageInfo>,
    response: Callback<Result<LanguageInfo, Error>>,
    task: Option<FetchTask>,
    props: Props,
}

impl Component for LanguageShow {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            languages: Languages::new(),
            language: None,
            response: link.callback(Msg::Response),
            task: None,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Response(Ok(language)) => {
                self.language = Some(language);
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
        if let Some(language) = &self.language {
            html! {
                <article>
                    <h1>{&language.id}</h1>
                    <h2>{&language.name}</h2>
                    <p>{&language.lang}</p>
                    <p>{&language.territory}</p>
                </article>
            }
        } else {
            html! {}
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(id) = &self.props.id {
                debug!("{}", id);
                self.task = Some(self.languages.get(id.clone(), self.response.clone()))
            }
        }
    }
}
