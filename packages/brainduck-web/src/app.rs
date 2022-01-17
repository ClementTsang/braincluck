use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::components::{MoonIcon, RunIcon, SunIcon, TextDivider};

pub enum Msg {
    Run,
    OpenExecution,
    CloseExecution,
    ToggleDarkMode,
}

pub struct Model {
    dark_mode: bool,
    execution_open: bool,
    textarea_ref: NodeRef,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            dark_mode: false,
            execution_open: false,
            textarea_ref: NodeRef::default(),
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            // Autofocus
            if let Some(input) = self.textarea_ref.cast::<HtmlTextAreaElement>() {
                let _ = input.focus();
            }
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
            Msg::ToggleDarkMode => {
                self.dark_mode = !self.dark_mode;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        let code_classes = classes!(
            "font-mono",
            "p-6",
            "border-2",
            "rounded",
            "border-blue",
            "border-solid",
            "w-full",
            "h-full",
            "bg-white",
        );
        let output_classes = classes!(
            "flex",
            "flex-col",
            "flex-1",
            "p-2",
            "border-2",
            "rounded",
            "border-blue",
            "border-solid",
            "w-full",
            "h-full",
            "bg-white",
        );

        html! {
            <body class={classes!("bg-slate-50")}>
                <div class={classes!("flex", "flex-col", "w-screen", "h-screen", "p-6", "space-y-3")}>
                    <div class={classes!("flex", "flex-none")}>
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
                    <div class={classes!("flex","flex-1")}>
                        <div class={classes!("flex", "flex-col", "lg:flex-row", "w-full", "h-full", "lg:space-x-3", "space-y-3")}>
                            <div class={classes!("flex","flex-1")}>
                                <textarea ref={self.textarea_ref.clone()} class={code_classes} autofocus=true style="resize: none;"></textarea>
                            </div>
                            if self.execution_open {
                                <div class={output_classes}>
                                    <p class={classes!("text-center", "text-xl")}>
                                        {"Execution"}
                                    </p>
                                    <TextDivider text="Output"/>
                                </div>
                            }
                        </div>
                    </div>
                </div>
            </body>
        }
    }
}
