use braincluck_interpreter::{bf_parse, Cells};
use std::io::{BufWriter, Cursor};
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[cfg(feature = "console_log")]
#[allow(unused_imports)]
use log::debug;

use crate::components::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Msg {
    Run,
    ToggleExecution,
    ToggleDarkMode,
    ClearOutput,
}

#[derive(Clone)]
pub struct App {
    dark_mode: bool,
    output_open: bool,
    text_ref: NodeRef,
    temp_output: String,
}

impl Component for App {
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
            output_open: false,
            text_ref: NodeRef::default(),
            temp_output: String::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Run => {
                self.output_open = true;
                self.temp_output = String::default();

                if let Some(input) = self.text_ref.cast::<HtmlTextAreaElement>() {
                    let code = input.value();

                    if let Ok(code) = bf_parse(&code) {
                        let mut cells = Cells::default();
                        let out = vec![];
                        let mut buf_out = BufWriter::new(out);
                        let input = vec![];
                        let mut cursor = Cursor::new(input);
                        if cells.interpret(&code, &mut buf_out, &mut cursor).is_ok() {
                            let output = String::from_utf8(
                                buf_out
                                    .into_inner()
                                    .expect("getting inner buffer should work"),
                            )
                            .expect("string should be valid utf8");
                            self.temp_output = output;
                        }
                    }
                }
                true
            }
            Msg::ToggleExecution => {
                self.output_open = !self.output_open;
                true
            }
            Msg::ToggleDarkMode => {
                self.dark_mode = !self.dark_mode;
                true
            }
            Msg::ClearOutput => {
                self.temp_output = String::default();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        let run_onclick = link.callback(|_| Msg::Run);
        let toggle_dark_mode_onclick = link.callback(|_| Msg::ToggleDarkMode);
        let toggle_output_onclick = link.callback(|_| Msg::ToggleExecution);
        let clear_output_onclick = link.callback(|_| Msg::ClearOutput);

        let body_classes = {
            classes!(if self.dark_mode {
                vec!["dark", "bg-slate-700"]
            } else {
                vec!["bg-slate-50"]
            })
        };

        let dark_classes = { classes!("w-12", "h-12", "flex-none") };

        html! {
            <body class={body_classes}>
                <div class={classes!("flex", "flex-col", "w-screen", "h-screen", "p-6", "space-y-3")}>
                    <div class={classes!("flex", "flex-none", "flex-row")}>
                        <div class={classes!("flex-1", "flex", "flex-row", "space-x-2")}>
                            <IconButton onclick={run_onclick} text={"Run"} class={classes!("flex-none")}>
                                <RunIcon />
                            </IconButton>
                            <IconButton onclick={toggle_dark_mode_onclick} class={dark_classes}>
                                if self.dark_mode {
                                    <SunIcon />
                                } else {
                                    <MoonIcon />
                                }
                            </IconButton>
                        </div>
                        <div class={classes!("flex-1", "flex", "flex-row", "justify-end", "space-x-2")}>
                            if self.output_open {
                                <IconButton class={classes!("flex-none")} onclick={clear_output_onclick} text={ "Clear Output" } />
                            }
                            <IconButton class={classes!("flex-none")} onclick={toggle_output_onclick} text={ if self.output_open { "Hide Output" } else { "Show Output" } } />
                        </div>
                    </div>
                    <div class={classes!("flex","flex-1")}>
                        <div class={classes!("flex", "flex-col", "lg:flex-row", "w-full", "h-full", "space-y-3", "lg:space-y-0", "lg:space-x-3")}>
                            <Code text_ref={self.text_ref.clone()} />
                            <Output hidden={!self.output_open} text={self.temp_output.clone()}/>
                        </div>
                    </div>
                </div>
            </body>
        }
    }
}
