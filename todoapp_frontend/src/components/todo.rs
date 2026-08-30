use yew::prelude::*;

use crate::{components::{checkbox::Checkbox, hidden_input::HiddenInput}, models::todos::Todo};


#[derive(Properties, PartialEq)]
pub struct ItemProps {
    pub item: Todo,
    #[prop_or_default]
    pub update_item_callback: Option<Callback<Todo>>,
}

#[component]
pub fn Item(props: &ItemProps) -> Html {
    let item = &props.item;

    let checkbox = {
        let item = item.clone();
        let done = item.done;
        let update_item_callback = props.update_item_callback.clone();
        let callback = Callback::from(move |value| {
            let mut item = item.clone();
            let update_item_callback = update_item_callback.clone();
            item.done = value;
            update_item_callback.map(|c| c.emit(item));
        });

        html!(
            <Checkbox checked={done} on_change={callback}/>
        )
    };

    let title = {
        let item = item.clone();
        let title = item.title.clone();
        let done = item.done;
        let update_item_callback = props.update_item_callback.clone();

        let callback = Callback::from(move |value| {
            let mut item = item.clone();
            let update_item_callback = update_item_callback.clone();
            item.title = value;
            update_item_callback.map(|c| c.emit(item));
        });

        html!(
            <HiddenInput value={title} strikethrough={done} callback={callback} />
        )
    };

    let description = {
        let item = item.clone();
        let description = item.description.clone();
        let update_item_callback = props.update_item_callback.clone();

        let callback = Callback::from(move |value| {
            let mut item = item.clone();
            let update_item_callback = update_item_callback.clone();
            item.description = value;
            update_item_callback.map(|c| c.emit(item));
        });

        html!(
            <HiddenInput value={description} callback={callback} />
        )
    };


    html!(
        <div>
            <span>{checkbox}</span>
            <span>{title}</span>
            <span>{description}</span>
        </div>
    )
}
