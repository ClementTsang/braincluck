use crate::components::*;
use yew::{classes, function_component, html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct OutputProps {
    #[prop_or_default]
    pub hidden: bool,

    #[prop_or_default]
    pub text: String,
}

#[function_component(Output)]
pub fn output(props: &OutputProps) -> Html {
    let output_classes = classes!(
        "flex",
        "flex-col",
        "p-2",
        "border-2",
        "rounded",
        "border-slate-200",
        "dark:border-slate-800",
        "border-solid",
        "bg-white",
        "dark:bg-slate-800",
        "relative",
        if props.hidden {
            vec!["flex-none"]
        } else {
            vec!["flex-1"]
        }
    );

    let code_classes = classes!(
        "font-mono",
        "text-black",
        "dark:text-slate-50",
        "p-6",
        "w-full",
        "h-full",
        "bg-white",
        "dark:bg-slate-800",
    );

    html! {
        if !props.hidden {
            <div class={output_classes}>
                <div class={classes!("place-content-center")}>
                    <p class={classes!("text-center", "text-xl", "text-black", "dark:text-slate-50")}>
                        {"Execution"}
                    </p>
                </div>
                <TextDivider text="Output"/>
                if !props.text.is_empty() {
                    <pre class={code_classes}>
                        { props.text.clone() } // TODO: Escape?
                    </pre>
                }
            </div>
        }
    }
}
