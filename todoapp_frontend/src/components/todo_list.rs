use yew::{platform::spawn_local, prelude::*};

use crate::{api::{get_todos, new_todo, update_todo}, components::todo::Item, models::todos::{Todo, TodoInsert}};

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

    let new_item = {
        let todos = todos.clone();
        Callback::from(move |_| {
            let todos = todos.clone();
            spawn_local(async move {
                let mut new_todos = (*todos).clone();
                let item = TodoInsert {
                    description: String::new(),
                    title: String::new(),
                };
                let res = new_todo(item).await;
                new_todos.push(res);
                todos.set(new_todos);
            });
        })
    };

    html!(
        <div  class="flex gap-3 flex-col">
            for todo in todos.iter() {
                <Item item={todo.clone()} update_item_callback={callback.clone()}/>
            }

            <div><button onclick={new_item}>{"New Item"}</button></div>
        </div>
    )
}