use web_sys::{HtmlInputElement, js_sys::futures::spawn_local, window};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{Route, api, models::{context::{ActionContext, AppContext}, users::Login}};

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

    let oidc_click = {
        Callback::from(move |_| {
            spawn_local(async move {
                let redirect_uri = api::oidc_get_url().await;
                window().unwrap().location().set_href(&redirect_uri).unwrap();
            });
        })
    };

    let oidc_state = use_state(|| None);

    {
        let oidc_state = oidc_state.clone();
        use_effect_with((), move |_| {
            let oidc_state = oidc_state.clone();
            spawn_local(async move {
                let res = api::oidc_exists().await;
                oidc_state.set(Some(res));
            });
        });
    }

    html!(
        <div class="w-lg mx-auto bg-white rounded-3xl p-10">
            <h1 class="text-center text-5xl font-mono my-10"> {"LOGIN"} </h1>
            if let Some(error) = login_err.as_ref() {
                <div class="bg-red-100 p-2 rounded">
                    {error}
                </div>
            }
            <form onsubmit={submit} class="flex flex-col gap-5 mt-2">
                <input class="border-b-1 border-amber-400 p-1" oninput={username_change} type="text" placeholder="user id" name="user" />
                <input class="border-b-1 border-amber-400 p-1" oninput={password_change} type="password" placeholder="password" name="password" />
                <button type="submit" class="bg-amber-50 border-1 rounded-xl p-3 font-mono border-amber-400">{"Submit"}</button>
            </form>

            if *oidc_state == None {
                <div> {"OR"} </div>
                {"OIDC loading..."}
            } else if *oidc_state == Some(true) {
                <div> {"OR"} </div>
                <button onclick={oidc_click}>{"Login with OIDC"}</button>
            }
        </div>
    )
}