use yew::{classes, function_component, html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub text: &'static str,
}

#[function_component(TextDivider)]
pub fn text_divider(props: &Props) -> Html {
    html! {
        <div class={classes!("relative", "flex", "py-5", "px-6", "items-center")}>
            <div class={classes!("flex-grow", "border-t"," border-gray-300")}></div>
            <span class={classes!("flex-shrink", "text-lg", "mx-6", "text-gray-300")}>{ &props.text }</span>
            <div class={classes!("flex-grow", "border-t", "border-gray-300")}></div>
        </div>
    }
}
