use yew::prelude::*;

pub struct Gallery {}

impl Component for Gallery {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Gallery {}
    }

    fn view(&self) -> Html {
        html! {
            <div class="w-full h-full m-8">
                <img />
            </div>
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        true
    }
}