use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
    <h1>{ "Project 3 ECE 421" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
