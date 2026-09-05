use serde::Deserialize;
use web_sys::js_sys::futures::spawn_local;
use yew::prelude::*;
use yew_router::hooks::{use_location, use_navigator};

use crate::{Route, api, models::context::{ActionContext, AppContext}};

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct OidcQueryParams {
    pub code: String,
}

#[component]
pub fn OidcRedirect() -> Html {
    let context = use_context::<AppContext>().unwrap();

    let location = use_location().unwrap();

    let query_params = location.query::<OidcQueryParams>().unwrap();

    let code = query_params.code;

    let navigator = use_navigator().unwrap();

    use_effect_with((), move |_| {
        spawn_local(async move {
            let res = api::oidc_redirect(&code).await;

            context.dispatch(ActionContext::SetUser(res));

            navigator.push(&Route::Home);
        });

        || {}
    });

    html!(<>
        {"Loging in..."}
    </>)
}