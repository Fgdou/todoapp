pub mod models;
pub mod api;
pub mod components;

use yew::prelude::*;

use crate::components::todo_list::TodoList;

#[component]
fn App() -> Html {
    html!(
        <div>
            <h1>{"TODO List"}</h1>
            <TodoList/>
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
