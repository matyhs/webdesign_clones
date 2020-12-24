use yew::prelude::*;

use crate::components::*;

pub struct App {}

impl Component for App {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn view(&self) -> Html {
        html!{
            <main_layout::MainLayout />
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        true
    }
}