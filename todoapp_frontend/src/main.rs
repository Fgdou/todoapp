pub mod models;
pub mod api;

use yew::{platform::spawn_local, prelude::*};

use crate::{api::get_todos, models::todos::Todo};

#[derive(Properties, PartialEq)]
struct ItemProps {
    item: Todo,
}

#[component]
fn Item(props: &ItemProps) -> Html {
    let item = &props.item;
    html!(
        <div>
            <span>{item.done}</span>
            <span>{item.title.clone()}</span>
            <span>{item.description.clone()}</span>
        </div>
    )
}

#[component]
fn List() -> Html {
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
        <div>
            <h1>{"Todo List"}</h1>
            for todo in todos.iter() {
                <Item item={todo.clone()}/>
            }
        </div>
    )
}

#[component]
fn App() -> Html {
    html!(
        <List/>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
