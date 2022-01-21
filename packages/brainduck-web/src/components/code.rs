use web_sys::HtmlTextAreaElement;
use yew::{classes, function_component, functional::*, html, Properties, NodeRef};

#[derive(Clone, PartialEq, Properties)]
pub struct CodeProps {
    pub text_ref: NodeRef,
}

#[function_component(Code)]
pub fn code(props: &CodeProps) -> Html {
    let code_classes = classes!(
        "font-mono",
        "text-black",
        "dark:text-slate-50",
        "p-6",
        "border-2",
        "rounded",
        "border-slate-200",
        "dark:border-slate-800",
        "focus:border-sky-600",
        "border-solid",
        "w-full",
        "h-full",
        "bg-white",
        "dark:bg-slate-800",
    );

    let textarea_ref = props.text_ref.clone();
    {
        let textarea_ref = textarea_ref.clone();
        use_effect(move || {
            if let Some(input) = textarea_ref.cast::<HtmlTextAreaElement>() {
                let _ = input.focus();
            }
            || ()
        });
    }

    html! {
        <div class={classes!("flex","flex-1")}>
            <textarea ref={textarea_ref} class={code_classes} autofocus=true style="resize: none;" placeholder="Your code here"></textarea>
        </div>
    }
}
