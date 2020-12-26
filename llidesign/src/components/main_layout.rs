use yew::prelude::*;

use crate::components::*;

pub struct MainLayout {}

impl Component for MainLayout {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        MainLayout {}
    }

    fn view(&self) -> Html {
        html!{
            <>
                <gallery::Gallery />
                <header::Header />
            </>
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        true
    }
}