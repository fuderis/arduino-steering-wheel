use app::{ prelude::*, Counter, };

/// The application component
#[function_component(App)]
fn app() -> Html {
    html! {
        <Counter/>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
