use yew::prelude::*;

pub struct Overlay {
    show_component: bool
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(false)]
    pub show_component: bool
}

impl Component for Overlay {
    type Properties = Props;
    type Message = ();

    fn  create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Overlay { show_component: props.show_component }
    }

    fn view(&self) -> Html {
        if self.show_component {
            html! {
                <div class="absolute top-0 left-0 w-screen h-screen flex flex-col flex-wrap content-center justify-center bg-white bg-opacity-80 text-center uppercase text-3xl font-thin">
                    <div class="cursor-pointer my-3">{"Our Services"}</div>
                    <div class="cursor-pointer my-3">{"About Us"}</div>
                    <div class="cursor-pointer my-3">{"How We Work"}</div>
                    <div class="cursor-pointer my-3">{"Bespoke Furniture"}</div>
                    <div class="cursor-pointer my-3">{"Contact Us"}</div>
                    <div class="cursor-pointer my-3">{"Blog"}</div>
                    <div class="cursor-pointer my-3">{"Careers"}</div>
                </div>
            }
        } else {
            html! {}
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.show_component = props.show_component;
        true
    }
}