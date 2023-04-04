use std::ops::Deref;
use yew::prelude::*;

use crate::components::text_input::TextInput;
use crate::components::button_input::ButtonInput;
use crate::boardgame::*;
use crate::chip::*;

#[function_component(Connect4HumanPage)]
pub fn connect4_human_page() -> Html {
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

    let con4 = use_state(|| BoardGame::connect4(6, 8));

    let cloned_con4 = con4.clone();
    let add_0 = Callback::from(move |_| {
        let mut data = cloned_con4.deref().clone();
        data.insert(0 as usize, Chip::One);
        cloned_con4.set(data);
    });

    let cloned_con4 = con4.clone();
    let add_1 = Callback::from(move |_| {
        let mut data = cloned_con4.deref().clone();
        data.insert(1 as usize, Chip::One);
        cloned_con4.set(data);
    });

    let cloned_con4 = con4.clone();
    let add_2 = Callback::from(move |_| {
        let mut data = cloned_con4.deref().clone();
        data.insert(2 as usize, Chip::One);
        cloned_con4.set(data);
    });

    let cloned_con4 = con4.clone();
    let add_3 = Callback::from(move |_| {
        let mut data = cloned_con4.deref().clone();
        data.insert(3 as usize, Chip::One);
        cloned_con4.set(data);
    });

    let cloned_con4 = con4.clone();
    let add_4 = Callback::from(move |_| {
        let mut data = cloned_con4.deref().clone();
        data.insert(4 as usize, Chip::One);
        cloned_con4.set(data);
    });

    let cloned_con4 = con4.clone();
    let add_5 = Callback::from(move |_| {
        let mut data = cloned_con4.deref().clone();
        data.insert(5 as usize, Chip::One);
        cloned_con4.set(data);
    });

    let cloned_con4 = con4.clone();
    let add_6 = Callback::from(move |_| {
        let mut data = cloned_con4.deref().clone();
        data.insert(6 as usize, Chip::One);
        cloned_con4.set(data);
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
            <small>{format!("(Disc Colors: {} - Red and {} - Yellow)", &*player1_name, &*player2_name)}</small>
            <br/>
            // <canvas id="gameboard" height="480" width="640"></canvas> // TODO: create board UI
            <ButtonInput label="0" onclick={add_0} />
            <ButtonInput label="1" onclick={add_1} />
            <ButtonInput label="2" onclick={add_2} />
            <ButtonInput label="3" onclick={add_3} />
            <ButtonInput label="4" onclick={add_4} />
            <ButtonInput label="5" onclick={add_5} />
            <ButtonInput label="6" onclick={add_6} />
            <table>
                { for con4.board.container.iter().map(|inner_vec| {
                    html! {
                        <tr>
                            { for inner_vec.iter().map(|&value| {
                                html! {
                                    <td>{"|"}{ value }</td>
                                }
                            })}
                        </tr>
                    }
                })}
            </table>
        </div>
    }
}
