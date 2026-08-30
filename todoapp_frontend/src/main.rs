pub mod models;
pub mod api;
pub mod components;

use yew::prelude::*;

use crate::components::task_list::TaskList;

#[component]
fn App() -> Html {
    html!(
        <div>
            <h1 class="bg-amber-50 text-center py-20 text-5xl font-mono">{"Todo List"}</h1>
            <div class="container mx-auto max-w-xl">
                <TaskList/>
            </div>
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
