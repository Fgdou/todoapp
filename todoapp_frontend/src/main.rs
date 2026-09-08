pub mod models;
pub mod api;
pub mod components;

use web_sys::js_sys::futures::spawn_local;
use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch, hooks::use_navigator};

use crate::{components::{login_form::LoginForm, oidc::OidcRedirect, task_list::TaskList}, models::context::{ActionContext, AppContext, Context}};

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!(<Home/>),
        Route::Login => html!(<Login/>),
        Route::OidcRedirect => html!(<OidcRedirect/>)
    }
}

#[component]
fn App() -> Html {
    let state = use_reducer(Context::default);
    html!(
        <>
            <ContextProvider<AppContext> context={state}>
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </ContextProvider<AppContext>>
            <div>
                <a  class="absolute bottom-0 right-0 mx-5 my-3 text-xl pointer-select" href="https://github.com/Fgdou/todoapp" target="_blank">{"GitHub"}</a>
            </div>
        </>
    )
}

#[component]
fn Login() -> Html {
    html!(
        <div class="bg-amber-50 h-screen flex justify-center flex-col">
            <LoginForm />
        </div>
    )
}

#[component]
fn Home() -> Html {
    let context: AppContext = use_context().unwrap();
    let user = &context.user;
    let navigator = use_navigator().unwrap();

    let logout_click = {
        let context: UseReducerHandle<Context> = context.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            let context = context.clone();
            let navigator = navigator.clone();
            spawn_local(async move {
                api::logout().await;
                context.dispatch(ActionContext::ClearUser);
                navigator.push(&Route::Login);
            });
        })
    };

    html!(
        <div>
            <h1 class="bg-amber-50 text-center py-20 font-mono">
                <div class="text-5xl">{"Todo List"}</div>
                <div>
                    if let Some(user) = user {
                        {"Welcome "} 
                        {&user.username} 
                        {" ("} 
                        <a class="text-blue-500 underline" href="#" onclick={logout_click}>{"Logout"}</a> 
                        {")"}
                    } else {
                        {"User not connected"}
                    }
                </div>
            </h1>
            <div class="container mx-auto max-w-xl">
                <TaskList/>
            </div>
        </div>
    )
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/auth/oidc/redirect")]
    OidcRedirect,
}

fn main() {
    yew::Renderer::<App>::new().render();
}
