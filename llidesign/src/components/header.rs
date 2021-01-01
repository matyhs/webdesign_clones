use yew::prelude::*;

pub struct Header {
    toggle: bool,
    onmenuclick: Callback<bool>,
    link: ComponentLink<Self>
}

pub enum Msg {
    MenuToggle
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(false)]
    pub toggle: bool,
    pub onmenuclick: Callback<bool>
}

impl Component for Header {
    type Properties = Props;
    type Message = Msg;
    
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Header { toggle: props.toggle, onmenuclick: props.onmenuclick, link:  link }
    }

    fn view(&self) -> Html {
        let burger_animation = "h-0.5 w-5 rounded mt-1 transition duration-500 ease-in-out";
        let cross_forward = if self.toggle {
            "transform rotate-45 translate-x-0.5 translate-y-1.5"
        } else {
            ""
        };
        let cross_backward = if self.toggle {
            "transform -rotate-45 translate-x-0.5 -translate-y-1.5"
        } else {
            ""
        };
        let middle_color  = if self.toggle {
            "bg-transparent"
        } else {
            "bg-gray-400"
        };
        let menuclick = self.link.callback(|_| Msg::MenuToggle);

        html! {
            <div class="absolute top-10 w-screen">
                <svg class="bg-white w-24 h-24 relative left-12 inline cursor-pointer">    
                    <text fill="gray" class="text-5xl font-mono" x="10" y="55">{"MNQ"}</text>
                    <text fill="gray" class="text-md text-gray-400 font-thin font-mono" x="25" y="75">{"CLONE"}</text>
                </svg>
                <div class="bg-white w-24 h-8 absolute right-12 inline cursor-pointer border border-solid border-gray-400" onclick=menuclick>
                    <span class="text-gray-400 uppercase text-xl font-thin relative inline p-1">
                        {"Menu"}
                    </span>
                    <div class="inline-block w-5">
                        <div class=("bg-gray-400", burger_animation, cross_forward)>
                        </div>
                        <div class=(burger_animation, middle_color)>
                        </div>
                        <div class=("bg-gray-400", burger_animation, cross_backward)>
                        </div>  
                    </div>
                </div>
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match  msg {
            Msg::MenuToggle => {
                self.toggle = !self.toggle;
                self.onmenuclick.emit(self.toggle);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.toggle = props.toggle;
        true
    }
}