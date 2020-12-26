use yew::prelude::*;

use crate::components::*;

pub struct MainLayout {
    show_overlay: bool,
    link: ComponentLink<Self>
}

pub enum Msg {
    AccessMenu(bool)
}

impl Component for MainLayout {
    type Properties = ();
    type Message = Msg;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        MainLayout { show_overlay: false, link: link }
    }

    fn view(&self) -> Html {
        html!{
            <>
                <gallery::Gallery />
                <overlay::Overlay show_component=self.show_overlay/>
                <header::Header toggle=self.show_overlay onmenuclick=self.link.callback(Msg::AccessMenu)/>
            </>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AccessMenu(toggle) => {
                self.show_overlay = toggle;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }
}