use yew::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct ButtonProperties {
    pub name: String,
    #[prop_or_default]
    pub on_click: Option<Callback<()>>,
    #[prop_or(false)]
    pub loading: bool,
}

#[component]
pub fn Button(props: &ButtonProperties) -> Html {
    html!(
        <button onclick={
                let on_click = props.on_click.clone();
                move |_| {
                    if let Some(f) = &on_click {
                        f.emit(());
                    }
                }
            } 
            type="submit" 
            class="disabled:cursor-progress disabled:bg-gray-100 cursor-pointer bg-amber-50 border-1 rounded-xl p-3 font-mono border-amber-400 font-bold text-lg" 
            disabled={props.loading}>
                {&props.name}
        </button>
    )
}