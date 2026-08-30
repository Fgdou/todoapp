use yew::{platform::spawn_local, prelude::*};

use crate::{api::{delete_task, get_tasks, new_task, update_task}, components::task::TaskItem, models::tasks::{Task, TaskInsert}};

#[component]
pub fn TaskList() -> Html {
    let tasks = use_state(|| Vec::new());

    {
        let tasks = tasks.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let res = get_tasks().await;
                tasks.set(res);
            });

            || ()
        });
    }

    let callback = {
        let tasks = tasks.clone();
        Callback::from(move |todo: Task| {
            {    
                let todo = todo.clone();
                spawn_local(async move {
                    update_task(todo.clone()).await;
                });
            }

            let mut new_tasks = (*tasks).clone();
            let found = new_tasks.iter_mut().find(|i| i == &&todo).unwrap();
            *found = todo;

            tasks.set(new_tasks);
        })
    };

    let delete_callback = {
        let tasks = tasks.clone();
        Callback::from(move |id: i32| {
            spawn_local(async move {
                delete_task(id).await;
            });

            let mut new_tasks = (*tasks).clone();
            new_tasks.retain(|t| t.id != id);
            tasks.set(new_tasks);
        })
    };

    let new_item = {
        let tasks = tasks.clone();
        Callback::from(move |_| {
            let tasks = tasks.clone();
            spawn_local(async move {
                let mut new_tasks = (*tasks).clone();
                let item = TaskInsert {
                    description: String::new(),
                    title: String::new(),
                };
                let res = new_task(item).await;
                new_tasks.push(res);
                tasks.set(new_tasks);
            });
        })
    };

    let update_handler = {
        let tasks = tasks.clone();
        Callback::from(move |_| {
            let tasks = tasks.clone();
            spawn_local(async move {
                let items = get_tasks().await;
                tasks.set(items);
            });
        })
    };

    html!(
        <div>
            <div class="my-5 flex justify-between">
                <button class="bg-amber-50 font-mono py-2 px-5 text-xl rounded-xl border-1 border-amber-400" onclick={new_item}>{"New Item"}</button>
                <button class="bg-amber-50 font-mono py-2 px-5 text-xl rounded-xl border-1 border-amber-400" onclick={update_handler}>{"Refresh"}</button>
            </div>
            <div  class="flex gap-3 flex-col">
                for todo in tasks.iter() {
                    <TaskItem item={todo.clone()} update_item_callback={callback.clone()} delete_callback={delete_callback.clone()}/>
                }

            </div>
        </div>
    )
}