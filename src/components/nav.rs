use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

pub struct Nav;

impl Component for Nav {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Nav {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let link_class = "px-3 py-1 relative block hover:text-gray-900 text-gray-500 text-sm";
        html! {
            <div class="w-60 p-4 pt-8 shadow-sm">
            <nav>
                <p class="px-3 mb-3 uppercase tracking-wide font-semibold text-sm text-gray-700">{"Edit"}</p> 
                <ul> 
                    <li><RouterAnchor<AppRoute> route=AppRoute::LanguageList classes=link_class >{ "Languges" }</RouterAnchor<AppRoute>></li>
                    <li><RouterAnchor<AppRoute> route=AppRoute::TranslationList classes=link_class >{ "Translations" }</RouterAnchor<AppRoute>></li>
                    <li><RouterAnchor<AppRoute> route=AppRoute::About classes=link_class>{ "About" }</RouterAnchor<AppRoute>></li>
                </ul>
            </nav>
            </div>
        }
    }
}
