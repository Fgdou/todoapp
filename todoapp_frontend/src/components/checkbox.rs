use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CheckboxProps {
    pub checked: bool,
    #[prop_or_default]
    pub on_change: Option<Callback<bool>>,
}

#[component]
pub fn Checkbox(props: &CheckboxProps) -> Html {
    let on_change = props.on_change.clone();

    let handle_change = Callback::from(move |e: Event| {
        let target: HtmlInputElement = e.target_unchecked_into();
        if let Some(on_change) = on_change.as_ref() {
            on_change.emit(target.checked());
        }
    });

    html!(
        <input type="checkbox" checked={props.checked} onchange={handle_change}/>
    )
}