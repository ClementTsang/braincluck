use yew::{classes, function_component, html, Callback, Children, Classes, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct IconButtonProps {
    pub onclick: Callback<()>,

    #[prop_or_default]
    pub text: Option<&'static str>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(IconButton)]
pub fn icon_button(props: &IconButtonProps) -> Html {
    let onclick = props.onclick.clone();
    let onclick = move |_| {
        onclick.emit(());
    };

    let button_classes = classes!(
        "bg-sky-600",
        "hover:bg-sky-700",
        "disabled:opacity-75",
        "disabled:hover:bg-sky-600",
        "rounded",
        "p-3",
        props.class.clone()
    );

    html! {
        <button {onclick} disabled={props.disabled} class={button_classes}>
            <div class={classes!("flex", "items-center", "justify-center", "space-x-1")}>
                if let Some(text) = props.text {
                    <p class={classes!("text-slate-50", "font-medium")}>
                        { text }
                    </p>
                }
                if !props.children.is_empty() {
                    <div class={classes!("h-4", "w-4", "text-white")}>
                        { for props.children.iter() }
                    </div>
                }
            </div>
        </button>
    }
}
