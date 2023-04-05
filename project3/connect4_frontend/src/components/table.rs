use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct Props {
    pub headers: Vec<&'static str>,
    pub data: Vec<Vec<String>>,
}

#[function_component]
pub fn Table(props: &Props) -> Html {
    // use props to pass table data in

    html! {
        <table style="min-width:80%">
            <tr style=" text-align: left">
                {props.headers.clone().into_iter().map(|header| {
                    html!{<th key={header}>{header}</th>}
                }).collect::<Html>()}
            </tr>

            {props.data.clone().into_iter().map(|data| {

                    html!{
                        <tr>

                            {data.into_iter().map(|e| {
                                html!{<td>{e}</td>}
                            }).collect::<Html>()}

                       </tr>
                    }

                }).collect::<Html>()}

        </table>
    }
}
