use web_sys::Window;
use yew::prelude::*;

use crate::components::*;

pub enum Msg {
    Run,
    OpenExecution,
    CloseExecution,
    EnableDarkMode,
    DisableDarkMode,
}

pub struct Model {
    dark_mode: bool,
    execution_open: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            dark_mode: web_sys::window()
                .and_then(|window| window.match_media("(prefers-color-scheme: dark)").ok())
                .map(|res| match res {
                    Some(res) => res.matches(),
                    None => false,
                })
                .unwrap_or(false),
            execution_open: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Run => {
                self.execution_open = true;
                true
            }
            Msg::OpenExecution => {
                self.execution_open = true;
                true
            }
            Msg::CloseExecution => {
                self.execution_open = false;
                true
            }
            Msg::EnableDarkMode => {
                self.dark_mode = true;
                true
            }
            Msg::DisableDarkMode => {
                self.dark_mode = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();

        let body_classes = {
            classes!(if self.dark_mode {
                vec!["dark", "bg-slate-700"]
            } else {
                vec!["bg-slate-50"]
            })
        };

        html! {
            <body class={body_classes}>
                <div class={classes!("flex", "flex-col", "w-screen", "h-screen", "p-6", "space-y-3")}>
                    <div class={classes!("flex", "flex-none", "flex-row", "space-x-2")}>
                        <div class={classes!("flex-none")}>
                            <button onclick={link.callback(|_| Msg::Run)} class={classes!("bg-sky-600", "hover:bg-sky-700", "rounded", "p-3")}>
                                <div class={classes!("flex", "items-center", "justify-center", "space-x-1")}>
                                    <p class={classes!("text-slate-50", "font-medium")}>
                                        {"Run"}
                                    </p>
                                    <div class={classes!("h-4", "w-4")} style="color: white;">
                                        <RunIcon />
                                    </div>
                                </div>
                            </button>
                        </div>
                        <div class={classes!("flex-none")}>
                            if self.dark_mode {
                                <>
                                    <button onclick={link.callback(|_| Msg::DisableDarkMode)}  class={classes!("bg-sky-600", "hover:bg-sky-700", "rounded", "p-3")}>
                                        <div class={classes!("flex", "items-center", "justify-center", "space-x-1")}>
                                            <p class={classes!("text-slate-50", "font-medium")}>
                                                {"Light"}
                                            </p>
                                            <div class={classes!("h-4", "w-4")} style="color: white;">
                                                <SunIcon />
                                            </div>
                                        </div>
                                    </button>
                                </>
                            } else {
                                <>
                                    <button onclick={link.callback(|_| Msg::EnableDarkMode)}  class={classes!("bg-sky-600", "hover:bg-sky-700", "rounded", "p-3")}>
                                        <div class={classes!("flex", "items-center", "justify-center", "space-x-1")}>
                                            <p class={classes!("text-slate-50", "font-medium")}>
                                                {"Dark"}
                                            </p>
                                            <div class={classes!("h-4", "w-4")} style="color: white;">
                                                <MoonIcon />
                                            </div>
                                        </div>
                                    </button>
                                </>
                            }
                        </div>
                    </div>
                    <div class={classes!("flex","flex-1")}>
                        <div class={classes!("flex", "flex-col", "lg:flex-row", "w-full", "h-full", "space-y-3", "lg:space-y-0", "lg:space-x-3")}>
                            <Code/>
                            <Output hidden={!self.execution_open}/>
                        </div>
                    </div>
                </div>
            </body>
        }
    }
}
