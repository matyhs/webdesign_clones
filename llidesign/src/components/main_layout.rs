use yew::prelude::*;

pub struct MainLayout {}

impl Component for MainLayout {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        MainLayout {}
    }

    fn view(&self) -> Html {
        html!{
            <div>
                
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