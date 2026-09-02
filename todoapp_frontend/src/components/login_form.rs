use web_sys::{HtmlInputElement, js_sys::futures::spawn_local};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{Route, api, models::{context::{ActionContext, AppContext}, users::{Login, User}}};

#[component]
pub fn LoginForm() -> Html {
    let navigator = use_navigator().unwrap();
    let login = use_state(|| Login {
        username: String::new(),
        password: String::new(),
    });
    let login_err = use_state(|| None);

    let username_change = {
        let login = login.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let username = input.value().parse().unwrap_or_default();
            let actual = (*login).clone();
            login.set(Login {
                username,
                password: actual.password,
            });
        })
    };
    let password_change = {
        let login = login.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let password = input.value().parse().unwrap_or_default();
            let actual = (*login).clone();
            login.set(Login {
                password,
                username: actual.username,
            });
        })
    };

    let state = use_context::<AppContext>().unwrap();

    let submit = {
        let login = login.clone();
        let state = state.clone();
        let navigator = navigator.clone();
        let login_err = login_err.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let login = login.clone();
            let state = state.clone();
            let login_err = login_err.clone();
            let navigator = navigator.clone();
            spawn_local(async move {
                let token = api::login(&login).await;
                match token {
                    Ok(token) => {
                        state.dispatch(ActionContext::SetUser(token));
                        navigator.push(&Route::Home);
                    },
                    Err(err) => login_err.set(Some(err)),
                };
            });
        })
    };

    html!(
        <div>
            if let Some(error) = login_err.as_ref() {
                <div>
                    {error}
                </div>
            }
            <form onsubmit={submit}>
                <input oninput={username_change} type="text" placeholder="user id" name="user" />
                <input oninput={password_change} type="password" placeholder="password" name="password" />
                <button type="submit">{"Login"}</button>
            </form>
        </div>
    )
}