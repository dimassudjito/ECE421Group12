use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub handle_onchange: Callback<String>,
    pub id: String,
    pub placeholder: String,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        handle_onchange.emit(value);
    });

    html! {
        <input type="text" id={props.id.clone()} placeholder={props.placeholder.clone()} onchange={onchange}/>
    }
}