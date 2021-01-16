use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Footer {}
    }

    fn update(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <footer>
                <nav>
                    <ul>
                        <li><RouterAnchor<AppRoute> route=AppRoute::Home classes="app-link">{ "Home" }</RouterAnchor<AppRoute>></li>
                        <li><RouterAnchor<AppRoute> route=AppRoute::About classes="app-link">{ "About" }</RouterAnchor<AppRoute>></li>
                    </ul>
                </nav>
            </footer>

        }
    }
}
