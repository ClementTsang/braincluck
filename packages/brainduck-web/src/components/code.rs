use web_sys::HtmlTextAreaElement;
use yew::{classes, function_component, functional::*, html};

#[function_component(Code)]
pub fn code() -> Html {
    let code_classes = classes!(
        "font-mono",
        "text-black",
        "dark:text-slate-50",
        "p-6",
        "border-2",
        "focus:border-0",
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

    let textarea_ref = use_node_ref();
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
            <textarea ref={textarea_ref} class={code_classes} autofocus=true style="resize: none;"></textarea>
        </div>
    }
}
