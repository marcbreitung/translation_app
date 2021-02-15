use yew::prelude::*;
use yew_router::switch::Permissive;
use yew_router::{prelude::*, route::Route};

use crate::components::{header::Header, nav::Nav};
use crate::routes::{
    about::About, home::Home, languages::language_create::LanguageCreate,
    languages::language_edit::LanguageEdit, languages::language_list::LanguageList,
    languages::language_show::LanguageShow, translation::translation_create::TranslationCreate,
    translation::translation_edit::TranslationEdit, translation::translation_list::TranslationList,
    translation::translation_show::TranslationShow, AppRoute,
};

/// Root component
pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Header />
                <main class="flex-1 flex">
                <div class="w-60 p-4 pt-8 shadow-sm">
                <Nav />
                </div>
                <div class="flex-1 bg-gray-50">
                <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute | {
                        match switch {
                            AppRoute::Home => html!{ <Home /> },
                            AppRoute::About => html!{ <About /> },
                            AppRoute::LanguageEdit(id) => html!{ <LanguageEdit id=Some(id.clone()) /> },
                            AppRoute::LanguageShow(id) => html!{ <LanguageShow id=Some(id.clone()) /> },
                            AppRoute::LanguageList => html!{ <LanguageList /> },
                            AppRoute::LanguageCreate => html!{ <LanguageCreate /> },
                            AppRoute::TranslationEdit(id) => html!{ <TranslationEdit id=Some(id.clone()) /> },
                            AppRoute::TranslationShow(id) => html!{ <TranslationShow id=Some(id.clone()) /> },
                            AppRoute::TranslationList => html!{ <TranslationList /> },
                            AppRoute::TranslationCreate => html!{ <TranslationCreate /> },
                            AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found"},
                            AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                        }
                    } )
                    redirect = Router::redirect(|route: Route<()>| {
                        AppRoute::PageNotFound(Permissive(Some(route.route)))
                    })
                />
                    </div>
                </main>
            </>
        }
    }
}
