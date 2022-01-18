use crate::components::TextDivider;
use yew::{classes, function_component, html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct OutputProps {
    #[prop_or_default]
    pub hidden: bool,
}

#[function_component(Output)]
pub fn output(props: &OutputProps) -> Html {
    if props.hidden {
        html! {
        <div>
        </div>
        }
    } else {
        let output_classes = classes!(
            "flex",
            "flex-col",
            "flex-1",
            "p-2",
            "border-2",
            "rounded",
            "border-slate-200",
            "dark:border-slate-800",
            "border-solid",
            "w-full",
            "h-full",
            "bg-white",
            "dark:bg-slate-800"
        );

        html! {
            <div class={output_classes}>
                <p class={classes!("text-center", "text-xl", "text-black", "dark:text-slate-50")}>
                    {"Execution"}
                </p>
                <TextDivider text="Output"/>
            </div>
        }
    }
}
