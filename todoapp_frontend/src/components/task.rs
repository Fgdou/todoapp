use yew::prelude::*;

use crate::{components::{checkbox::Checkbox, hidden_input::HiddenInput}, models::tasks::Task};


#[derive(Properties, PartialEq)]
pub struct ItemProps {
    pub item: Task,
    #[prop_or_default]
    pub update_item_callback: Option<Callback<Task>>,
    #[prop_or_default]
    pub delete_callback: Option<Callback<i32>>,
}

#[component]
pub fn TaskItem(props: &ItemProps) -> Html {
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
            <HiddenInput value={title} strikethrough={done} callback={callback} label={"Title"} />
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
            <HiddenInput multiline={true} value={description} callback={callback} label={"Description"} />
        )
    };


    let delete_handler = {
        let delete_callback = props.delete_callback.clone();
        let id = item.id;
        Callback::from(move |_| {
            delete_callback.as_ref().map(|c| c.emit(id));
        })
    };

    html!(
        <div class="flex gap-2 border-1 rounded-xl px-3 py-2 border-amber-400">
            <div>{checkbox}</div>
            <div class="flex-grow flex flex-col">
                <div class="flex">
                    <span class="flex-grow">{title}</span>
                    <span><button onclick={delete_handler} class="cursor-pointer text-red-800 font-mono">{"Delete"}</button></span>
                </div>
                <span>{description}</span>
            </div>
        </div>
    )
}
