pub mod models;
pub mod api;
pub mod components;

use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch};

use crate::{components::{login_form::LoginForm, oidc::OidcRedirect, task_list::TaskList}, models::context::{AppContext, Context}};

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
        <ContextProvider<AppContext> context={state}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<AppContext>>
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
    html!(
        <div>
            <h1 class="bg-amber-50 text-center py-20 text-5xl font-mono">{"Todo List"}</h1>
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
