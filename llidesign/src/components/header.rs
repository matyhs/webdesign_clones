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
        let menuclick = self.link.callback(|_| Msg::MenuToggle);   
        html! {
            <div class="absolute top-10 w-screen">
                <svg class="bg-white w-24 h-24 relative left-12 inline cursor-pointer">    
                    <text fill="gray" class="text-5xl font-mono" x="10" y="55">{"MNQ"}</text>
                    <text fill="gray" class="text-md text-gray-400 font-thin font-mono" x="25" y="75">{"CLONE"}</text>
                </svg>
                <div class="bg-white w-24 h-8 absolute right-12 inline cursor-pointer border border-solid border-gray-400" onclick=menuclick>
                    <svg class="w-16 h-8 relative inline">
                        <text fill="gray" class="text-lg font-mono inline" x="10" y="22">{"Menu"}</text>
                    </svg>
                    <svg class="w-4 h-4 inline mt-1" viewBox="0 0 384 384">
                        <path d="m368 154.667969h-352c-8.832031 0-16-7.167969-16-16s7.167969-16 16-16h352c8.832031 0 16 7.167969 16 16s-7.167969 16-16 16zm0 0"/>
                        <path d="m368 32h-352c-8.832031 0-16-7.167969-16-16s7.167969-16 16-16h352c8.832031 0 16 7.167969 16 16s-7.167969 16-16 16zm0 0"/>
                        <path d="m368 277.332031h-352c-8.832031 0-16-7.167969-16-16s7.167969-16 16-16h352c8.832031 0 16 7.167969 16 16s-7.167969 16-16 16zm0 0"/>
                    </svg>
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