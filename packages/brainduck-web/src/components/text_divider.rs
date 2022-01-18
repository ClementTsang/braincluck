use yew::{classes, function_component, html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct TextDividerProps {
    pub text: &'static str,
}

#[function_component(TextDivider)]
pub fn text_divider(props: &TextDividerProps) -> Html {
    html! {
        <div class={classes!("relative", "flex", "py-5", "px-6", "items-center")}>
            <div class={classes!("flex-grow", "border-t", "border-gray-300", "dark:border-gray-500")}></div>
            <span class={classes!("flex-shrink", "text-lg", "mx-6", "text-gray-300", "dark:text-gray-500")}>{ &props.text }</span>
            <div class={classes!("flex-grow", "border-t", "border-gray-300", "dark:border-gray-500")}></div>
        </div>
    }
}
