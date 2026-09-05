use yew::{platform::spawn_local, prelude::*};
use yew_router::hooks::use_navigator;

use crate::{Route, api::{delete_task, get_tasks, new_task, update_task}, components::{button::Button, task::TaskItem}, models::{context::AppContext, tasks::{Task, TaskInsert}}};

#[component]
pub fn TaskList() -> Html {
    let tasks = use_state(|| Vec::new());
    let context = use_context::<AppContext>().unwrap();
    let user = &context.user;
    let token = user.as_ref().map(|u| u.token.clone()).unwrap_or_default();
    let navigator = use_navigator().unwrap();

    let refreshing = use_state(|| false);

    if user.as_ref().is_none() {
        navigator.push(&Route::Login);
        return html!(<></>)
    }

    {
        let refreshing = refreshing.clone();
        let tasks = tasks.clone();
        let token = token.clone();
        use_effect_with((), move |_| {
            refreshing.set(true);
            spawn_local(async move {
                let res = get_tasks(&token).await;
                tasks.set(res);
                refreshing.set(false);
            });

            || ()
        });
    }

    let callback = {
        let tasks = tasks.clone();
        let token = token.clone();
        Callback::from(move |todo: Task| {
            {    
                let todo = todo.clone();
                let token = token.clone();
                spawn_local(async move {
                    update_task(todo.clone(), &token).await;
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
        let token = token.clone();
        Callback::from(move |id: i32| {
            let token = token.clone();
            spawn_local(async move {
                delete_task(id, &token).await;
            });

            let mut new_tasks = (*tasks).clone();
            new_tasks.retain(|t| t.id != id);
            tasks.set(new_tasks);
        })
    };

    let new_item = {
        let tasks = tasks.clone();
        let token = token.clone();
        Callback::from(move |_| {
            let tasks = tasks.clone();
            let token = token.clone();
            spawn_local(async move {
                let mut new_tasks = (*tasks).clone();
                let item = TaskInsert {
                    description: String::new(),
                    title: String::new(),
                };
                let res = new_task(item, &token).await;
                new_tasks.push(res);
                tasks.set(new_tasks);
            });
        })
    };

    let update_handler = {
        let tasks = tasks.clone();
        let token = token.clone();
        let refreshing = refreshing.clone();
        Callback::from(move |_| {
            let refreshing = refreshing.clone();
            refreshing.set(true);
            let tasks = tasks.clone();
            let token = token.clone();
            spawn_local(async move {
                let items = get_tasks(&token).await;
                tasks.set(items);
                refreshing.set(false);
            });
        })
    };

    html!(
        <div>
            <div class="my-5 flex justify-between">
                <Button name="New Item" on_click={new_item}/>
                <Button name="Refresh" on_click={update_handler} loading={*refreshing}/>
            </div>
            <div  class="flex gap-3 flex-col">
                for todo in tasks.iter() {
                    <TaskItem item={todo.clone()} update_item_callback={callback.clone()} delete_callback={delete_callback.clone()}/>
                }

            </div>
        </div>
    )
}