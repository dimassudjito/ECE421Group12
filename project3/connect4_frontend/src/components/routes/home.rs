use yew::prelude::*;

#[function_component]
pub fn HomePage() -> Html {
    html! {<div>

        <div  style="margin-top:75px;width:100%">
            <h5 style="font-size: 40px;" ><b>{"Welcome"}</b></h5>
            <p>{"This application contains the following two board games, both in human Vs. human and human Vs. Computer versions."}
            </p>
            <p>{"The computer versions have multiple difficulty levels."}
            </p>
            <ul>
                <li>{"Connect 4"}</li>

                <li>{"TOOT-OTTO"}</li>
            </ul>
            <p>{"Select the game of your choice from the side bar, and start playing. Enjoy!"}</p>
        </div>

    </div>}
}
