use yew::prelude::*;

pub struct Overlay {
    show_component: bool,
    onmenuoptionclick: Callback<Msg>,
    link: ComponentLink<Self>
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(false)]
    pub show_component: bool,
    pub onmenuoptionclick: Callback<Msg>
}

pub enum Msg {
    OurService,
    AboutUs,
    HowWeWork,
    BeSpokeFurniture,
    ContactUs,
    Blog,
    Careers
}

impl Component for Overlay {
    type Properties = Props;
    type Message = Msg;

    fn  create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Overlay { show_component: props.show_component, onmenuoptionclick: props.onmenuoptionclick, link: link }
    }

    fn view(&self) -> Html {
        let visibility = if self.show_component {
            "opacity-100 transform scale-y-100"
        } else {
            "opacity-0 transform scale-y-0"
        };

        html! {
            <div class=("absolute top-0 left-0 w-screen h-screen flex flex-col flex-wrap content-center justify-center bg-white bg-opacity-80 text-center uppercase text-3xl font-thin transition duration-500 ease-in-out", visibility)>
                <div onclick=self.link.callback(|_| Msg::OurService) class="cursor-pointer my-3">{"Our Services"}</div>
                <div class="cursor-pointer my-3">{"About Us"}</div>
                <div class="cursor-pointer my-3">{"How We Work"}</div>
                <div class="cursor-pointer my-3">{"Bespoke Furniture"}</div>
                <div class="cursor-pointer my-3">{"Contact Us"}</div>
                <div class="cursor-pointer my-3">{"Blog"}</div>
                <div class="cursor-pointer my-3">{"Careers"}</div>
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.onmenuoptionclick.emit(msg);
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.show_component = props.show_component;
        true
    }
}