use yew::prelude::*;

pub struct Alert;

impl Component for Alert {
    type Message = ();
    type Properties = ();

    fn crate(_: Self::Properties, _: ComponentLink<Self>) -> Self {
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
            <div>Message</div>
        }
    }
}
