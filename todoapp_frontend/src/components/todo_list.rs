use yew::{platform::spawn_local, prelude::*};

use crate::{api::{get_todos, update_todo}, components::{checkbox::Checkbox, hidden_input::HiddenInput}, models::todos::Todo};

#[derive(Properties, PartialEq)]
struct ItemProps {
    item: Todo,
    #[prop_or_default]
    update_item_callback: Option<Callback<Todo>>,
}

#[component]
fn Item(props: &ItemProps) -> Html {
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


    html!(
        <div>
            <span>{checkbox}</span>
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

    let callback = {
        let todos = todos.clone();
        Callback::from(move |todo: Todo| {
            {    
                let todo = todo.clone();
                spawn_local(async move {
                    update_todo(todo.clone()).await;
                });
            }

            let mut new_todos = (*todos).clone();
            let found = new_todos.iter_mut().find(|i| i == &&todo).unwrap();
            *found = todo;

            todos.set(new_todos);
        })
    };

    html!(
        <div  class="flex gap-3 flex-col">
            for todo in todos.iter() {
                <Item item={todo.clone()} update_item_callback={callback.clone()}/>
            }
        </div>
    )
}