use yew_router::prelude::*;
use yew_router::switch::Permissive;

pub mod about;
pub mod home;
pub mod translation;
pub mod languages;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/translation/{id}/edit"]
    TranslationEdit(String),
    #[to = "/translation/{id}"]
    TranslationShow(String),
    #[to = "/translations/create"]
    TranslationCreate,
    #[to = "/translations"]
    TranslationList,
    #[to = "/languages/{id}/edit"]
    LanguageEdit(String),
    #[to = "/languagelanguages/{id}"]
    LanguageShow(String),
    #[to = "/languages/create"]
    LanguageCreate,
    #[to = "/languages"]
    LanguageList,
    #[to = "/about"]
    About,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
    #[to = "/"]
    Home,
}
