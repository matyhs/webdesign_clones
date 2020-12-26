use yew::prelude::*;

static IMAGES: [&str; 4] = ["https://cdn.decorilla.com/online-decorating/wp-content/uploads/2018/10/modern-interior-design-grey-living-room2.png", 
                            "https://ulaburgiel.com/wp-content/uploads/2019/03/House-in-the-Woods-Interior-Design-Ula-Burgiel-1.jpg", 
                            "https://www.architectureartdesigns.com/wp-content/uploads/2017/03/16-16.jpg", 
                            "https://cdn.decorilla.com/online-decorating/wp-content/uploads/2018/07/minimalist-interior-design-bedroom.jpg"];

pub struct Gallery {
    current: String,
    index: usize,
    link: ComponentLink<Self>
}

pub enum Msg {
    Forward,
    Backward
}

impl Component for Gallery {
    type Properties = ();
    type Message = Msg;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Gallery { current: IMAGES[0].to_string(), index: 0, link: link }
    }

    fn view(&self) -> Html {
        let onforwardclick = self.link.callback(|_| Msg::Forward);
        let onbackwardclick = self.link.callback(|_| Msg::Backward);
        html! { 
            <div class="w-screen h-screen">
                <img class="w-full h-full p-8 top-0 left-0 absolute" src=self.current/>
                <svg onclick=onforwardclick class="fill-current text-white absolute top-1/2 right-8 w-20 h-20 cursor-pointer" viewBox="0 0 512 512" xmlns="http://www.w3.org/2000/svg">
                    <path d="m256 488a24 24 0 0 1 -16.971-40.971l191.03-191.029-191.03-191.029a24 24 0 0 1 33.942-33.942l208 208a24 24 0 0 1 0 33.942l-208 208a23.928 23.928 0 0 1 -16.971 7.029zm-191.029-7.029 208-208a24 24 0 0 0 0-33.942l-208-208a24 24 0 0 0 -33.942 33.942l191.03 191.029-191.03 191.029a24 24 0 0 0 33.942 33.942z"/>
                </svg>
                <svg onclick=onbackwardclick class="fill-current text-white absolute top-1/2 left-8 w-20 h-20 cursor-pointer" viewBox="0 0 512 512" xmlns="http://www.w3.org/2000/svg">
                    <path d="m256 488a23.928 23.928 0 0 1 -16.971-7.029l-208-208a24 24 0 0 1 0-33.942l208-208a24 24 0 0 1 33.942 33.942l-191.03 191.029 191.03 191.029a24 24 0 0 1 -16.971 40.971zm224.971-7.029a24 24 0 0 0 0-33.942l-191.03-191.029 191.03-191.029a24 24 0 0 0 -33.942-33.942l-208 208a24 24 0 0 0 0 33.942l208 208a24 24 0 0 0 33.942 0z"/>
                </svg>
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Forward => {
                if IMAGES.len() > self.index + 1 {
                    self.index += 1;
                } else {
                    self.index = 0;
                }
            },
            Msg::Backward => {
                if self.index > 0 {
                    self.index -= 1;
                } else {
                    self.index = IMAGES.len() - 1;
                }
            }
        };
        
        self.current = IMAGES[self.index].to_string();
        true
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        true
    }
}