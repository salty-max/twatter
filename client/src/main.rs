use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class={format!("p-2")}>
            {"Hello Yew!"}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
