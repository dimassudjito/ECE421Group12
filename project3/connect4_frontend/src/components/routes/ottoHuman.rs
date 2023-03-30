use yew::prelude::*;

use crate::components::text_input::TextInput;

#[function_component(OttoHumanPage)]
pub fn otto_human_page() -> Html {
    let player1_name = use_state(|| "Player 1".to_owned());
    let cloned_player1_name = player1_name.clone();
    let player1_name_changed = Callback::from(move |username: String| {
        cloned_player1_name.set(username);
    });

    let player2_name = use_state(|| "Player 2".to_owned());
    let cloned_player2_name = player2_name.clone();
    let player2_name_changed = Callback::from(move |username: String| {
        cloned_player2_name.set(username);
    });

    html! {
        <div>
            <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Player Names"}</b></h5>
            <hr style="width:50px;border:5px solid red" class="w3-round"/>
            <form>
                // <input id="startbutton" class="button" type="submit" value="Start Game"/> // TODO: create start button
                <TextInput handle_onchange={player1_name_changed} id="textbox1" placeholder="Player 1's Name" />
                <TextInput handle_onchange={player2_name_changed} id="textbox2" placeholder="Player 2's Name"/>
            </form>
            <br/>
            <h4>{format!("New Game: {} Vs {}", &*player1_name, &*player2_name)}</h4>
            <small>{format!("(Winning Combination: {} - TOOT and {} - OTTO)", &*player1_name, &*player2_name)}</small>
            <br/>
            <form>
                <h4>{"Select a Disc Type    :"}</h4>
                <input type="radio" name="choice" value="T"/> <span>{"T"}</span>
                <input type="radio" name="choice" value="O"/> <span>{"O"}</span>
            </form>
            <canvas id="gameboard" height="480" width="640"></canvas>
        </div>
    }
}
