use yew::prelude::*;

pub struct Header {

}

impl Component for Header {
    type Properties = ();
    type Message = ();
    
    fn create(_: Self::Properties, _:ComponentLink<Self>) -> Self {
        Header {}
    }

    fn view(&self) -> Html {
        html! {
            <div class="absolute top-10 w-screen">
                <svg class="bg-white w-24 h-24 relative left-12 inline cursor-pointer">    
                    <text fill="gray" class="text-5xl font-mono" x="10" y="55">{"MNQ"}</text>
                    <text fill="gray" class="text-md text-gray-400 font-thin font-mono" x="25" y="75">{"CLONE"}</text>
                </svg>
                <div class="bg-white w-24 h-8 absolute right-12 inline cursor-pointer border border-solid border-gray-400">
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

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        true
    }
}