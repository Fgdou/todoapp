use yew::{platform::spawn_local, prelude::*};
use yew_router::hooks::use_navigator;

use crate::{Route, api::{self, delete_task, get_tasks, new_task, update_task}, components::{button::Button, task::TaskItem}, models::{context::{ActionContext, AppContext}, tasks::{Task, TaskInsert}}};

#[component]
pub fn TaskList() -> Html {
    let tasks = use_state(|| Vec::new());
    let context = use_context::<AppContext>().unwrap();
    let navigator = use_navigator().unwrap();

    let refreshing = use_state(|| false);

    {
        let context = context.clone();
        use_effect_with((), move |_| {
            let context = context.clone();
            let user = &context.user;
            if user.as_ref().is_none() {
                let context = context.clone();
                spawn_local(async move {
                    let res = api::verify_login().await;
                    let context = context.clone();

                    match res {
                        None => {
                            navigator.push(&Route::Login);
                        },
                        Some(user) => {
                            context.dispatch(ActionContext::SetUser(user));
                        }
                    };
                });
            }
        });
    }

    let user = &context.user;

    {
        let refreshing = refreshing.clone();
        let tasks = tasks.clone();
        let user = user.clone();
        use_effect_with(user, move |user| {
            let return_closure = ||{};

            if user.is_none() {
                return return_closure;
            }
            refreshing.set(true);
            spawn_local(async move {
                let res = get_tasks().await;
                tasks.set(res);
                refreshing.set(false);
            });
            return_closure
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
        let refreshing = refreshing.clone();
        Callback::from(move |_| {
            let refreshing = refreshing.clone();
            refreshing.set(true);
            let tasks = tasks.clone();
            spawn_local(async move {
                let items = get_tasks().await;
                tasks.set(items);
                refreshing.set(false);
            });
        })
    };

    html!(
        <div class="m-5">
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