use yew::{platform::spawn_local, prelude::*};

use crate::{api::get_todos, components::{checkbox::Checkbox, hidden_input::HiddenInput}, models::todos::Todo};

#[derive(Properties, PartialEq)]
struct ItemProps {
    item: Todo,
}

#[component]
fn Item(props: &ItemProps) -> Html {
    let item = &props.item;
    html!(
        <div>
            <span><Checkbox checked={item.done}/></span>
            <span><HiddenInput value={item.title.clone()} strikethrough={item.done} /></span>
            <span><HiddenInput value={item.description.clone()} /></span>
        </div>
    )
}

#[component]
pub fn TodoList() -> Html {
    let todos = use_state(|| Vec::new());

    {
        let todos = todos.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let res = get_todos().await;
                todos.set(res);
            });

            || ()
        });
    }

    html!(
        <div  class="flex gap-3 flex-col">
            for todo in todos.iter() {
                <Item item={todo.clone()}/>
            }
        </div>
    )
}