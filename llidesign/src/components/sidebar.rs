use yew::prelude::*;

pub struct Sidebar {
    show_component: bool,
    onclose: Callback<()>,
    link: ComponentLink<Self>
}

pub enum Msg {
    CloseSideMenu
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(false)]
    pub show_component: bool,
    pub onclose: Callback<()>
}

impl Component for Sidebar {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Sidebar { show_component: props.show_component, onclose: props.onclose, link: link }
    }

    fn view(&self) -> Html {
        if self.show_component {
            html! {
                <div class="absolute top-0 right-0 w-1/4 h-screen flex flex-col bg-white p-10">
                    <div>
                        <h1 class="text-3xl w-3/5">{"Lorem Ipsum"}</h1>
                    </div>
                    <div class="overflow-y-scroll text-justify my-10 pr-2 text-sm leading-loose">
                        <p>
                            {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris convallis, elit ac lobortis porta, orci sapien semper leo, vel semper massa leo et ligula. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas sodales nibh vel ornare faucibus. Quisque vel egestas urna, ut auctor nisi. Nulla at facilisis massa, ut tincidunt velit. Sed tempus sapien at lectus accumsan, nec blandit tortor ultrices. Nulla a finibus urna. Vivamus arcu quam, malesuada non lorem in, lacinia aliquet justo. Aenean in tincidunt ipsum. Praesent et nulla a massa facilisis consequat nec vitae massa. "}
                        </p>
                        <br />
                        <p>
                            {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris scelerisque blandit pharetra. Integer ex felis, euismod ac porttitor a, rhoncus et ex. Donec in fringilla turpis. Aliquam sodales sollicitudin tellus vel posuere. Sed lorem lectus, semper ut pellentesque nec, iaculis at est. Cras sollicitudin sagittis elit, non aliquet ligula vestibulum vitae. Vivamus libero turpis, feugiat elementum urna eget, sodales tempor dolor. Vestibulum in nisl volutpat, vulputate odio sit amet, condimentum nunc. Nulla vestibulum tincidunt ipsum eget volutpat."}
                        </p>
                        <br />
                        <p>
                            {"Sed sed velit at ante vulputate maximus. Proin rhoncus auctor sem, sit amet interdum mi posuere finibus. Quisque metus lacus, pulvinar non elementum non, suscipit in metus. Ut placerat lectus placerat, dapibus ligula a, lacinia erat. Praesent pharetra blandit ex sit amet aliquam. Praesent sit amet porttitor nisl. Suspendisse ac neque odio. "}
                        </p>
                        <br />
                        <p>
                            {"In placerat, lorem ac congue dictum, magna nisl malesuada ipsum, id lacinia tortor nulla in tortor. Morbi volutpat bibendum libero eu efficitur. Mauris quis neque porta, hendrerit nisi non, bibendum nibh. Integer non massa auctor, cursus purus vel, commodo augue. Ut in elementum metus, ac molestie sem. Aliquam a blandit felis. Maecenas convallis massa ut dui interdum, quis luctus elit tincidunt."}
                        </p>
                        <br />
                        <p>
                            {"Phasellus ac lectus auctor libero porttitor suscipit. Ut interdum eleifend dolor. Integer finibus ipsum justo, consectetur egestas risus commodo non. Sed venenatis neque ex, non semper elit faucibus id. Etiam dignissim lorem ac condimentum rutrum. Vivamus quis convallis magna. Mauris non felis hendrerit, rhoncus turpis at, posuere sem. Ut vulputate sapien a quam tincidunt sagittis. Fusce enim nibh, maximus eu facilisis non, aliquet vitae nulla. Quisque convallis risus nisi, non condimentum nibh scelerisque nec. Sed nec blandit justo. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas."}
                        </p>
                    </div>
                    <div onclick=self.link.callback(|_| Msg::CloseSideMenu) class="text-sm cursor-pointer">
                        {"Close"}
                    </div>
                </div>
            }
        } else {
            html! {}
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CloseSideMenu => {
                self.onclose.emit(());
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.show_component = props.show_component;
        true
    }
}