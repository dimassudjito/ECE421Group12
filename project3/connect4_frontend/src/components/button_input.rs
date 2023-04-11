use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub onclick: Callback<()>,
    pub class: String,
}

#[function_component(ButtonInput)]
pub fn button_input(props: &Props) -> Html {
    let onclick = props.onclick.clone();
    let button_onclick = Callback::from(move |_| {
        onclick.emit(());
    });
    html! {
      <button onclick={button_onclick} class={props.class.clone()}>{&props.label}</button>
    }
}
