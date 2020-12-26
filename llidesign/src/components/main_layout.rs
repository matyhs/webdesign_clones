use yew::prelude::*;

use crate::components::*;

pub struct MainLayout {
    show_overlay: bool,
    show_sidebar: bool,
    link: ComponentLink<Self>
}

pub enum Msg {
    AccessMenu(bool),
    AccessMenuOption(overlay::Msg),
    CloseSideMenu
}

impl Component for MainLayout {
    type Properties = ();
    type Message = Msg;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        MainLayout { show_overlay: false, show_sidebar: false, link: link }
    }

    fn view(&self) -> Html {
        html!{
            <>
                <gallery::Gallery />
                <overlay::Overlay show_component=self.show_overlay onmenuoptionclick=self.link.callback(Msg::AccessMenuOption)/>
                <sidebar::Sidebar show_component=self.show_sidebar onclose=self.link.callback(|_| Msg::CloseSideMenu)/>
                <header::Header toggle=self.show_overlay onmenuclick=self.link.callback(Msg::AccessMenu)/>
            </>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AccessMenu(toggle) => {
                self.show_overlay = toggle;
                self.show_sidebar = false;
                true
            },
            Msg::AccessMenuOption(option) => {
                self.show_sidebar = true;
                self.show_overlay = false;
                true
            },
            Msg::CloseSideMenu => {
                self.show_sidebar = false;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }
}