use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::AppRoute;

pub struct Header;

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <header class="flex shadow-sm p-5 z-10">
              <div class="w-60">
                    <RouterAnchor<AppRoute> route=AppRoute::Home classes="brand-logo" >{ "Translation" }</RouterAnchor<AppRoute>>
          </div>
              <div class="flex flex-grow invisible md:visible">
                  <div class="flex-none">{"Suche"}</div>
                  <div class="flex-grow"></div>
                  <div class="flex-none">{"Account"}</div>
              </div>
          </header>  
        }
    }
}
