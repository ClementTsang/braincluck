use yew::prelude::*;

enum Msg {
    Contents(String),
}

struct Model {
    input: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input: String::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Contents(input) => {
                self.input = input;
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div class={classes!("flex", "flex-col", "w-screen", "h-screen", "p-6", "space-y-3")}>
                <div class={classes!("flex", "flex-none")}>
                    <button class={classes!("bg-blue-400", "rounded", "p-3")}>{ "Run" }</button>
                </div>
                <div class={classes!("flex","flex-1")}>
                    <div class={classes!("flex", "flex-col", "lg:flex-row", "w-full", "h-full")}>
                        <div class={classes!("flex","flex-1")}>
                            <textarea class={classes!("rounded-md", "border-black", "border-solid", "w-full", "h-full")} style="resize: none;"></textarea>
                        </div>
                        <div class={classes!("flex","flex-1")}>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
