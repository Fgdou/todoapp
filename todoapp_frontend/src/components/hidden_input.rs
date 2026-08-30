use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HiddenInputProperties {
    pub value: String,
    #[prop_or_default]
    pub callback: Option<Callback<String>>,
    #[prop_or_default]
    pub strikethrough: bool,
}

#[component]
pub fn HiddenInput(props: &HiddenInputProperties) -> Html {

    let callback = props.callback.clone();

    let handler = Callback::from(move |e: Event| {
        let target: HtmlInputElement = e.target_unchecked_into();
        if let Some(callback) = &callback {
            callback.emit(target.value());
        }
    });

    html!(
        <input type="text" value={props.value.clone()} onchange={handler} class={
            classes!(
                props.strikethrough.then(|| Some("line-through"))
            )
        } />
    )
}