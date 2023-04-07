use std::ops::Deref;
use yew::prelude::*;
use chrono::{Datelike, Timelike, Utc};

use crate::components::text_input::TextInput;
use crate::components::button_input::ButtonInput;
use crate::boardgame::*;
use crate::chip::*;
use crate::requests::postGame;

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
    let over = use_state(|| false);
    let winner = use_state(|| 0);

    let cloned_con4 = con4.clone();
    let cloned_over = over.clone();
    let cloned_winner = winner.clone();
    let cloned_player1_name = player1_name.clone();
    let cloned_player2_name = player2_name.clone();
    let add_chip = Callback::from(move |col: usize| {
        let mut data = cloned_con4.deref().clone();
        let chip = if data.board.counter % 2 == 0 {
            Chip::One
        } else {
            Chip::Two
        };
        let res = data.insert(col, chip);
        cloned_con4.set(data);
        if let Ok(x) = res {
            if let Some(y) = x {
                cloned_over.set(true);
                if y == 1 {
                    cloned_winner.set(1);
                } else {
                    cloned_winner.set(2);
                }

                let winner_name = if y == 1 {
                    cloned_player1_name.deref().clone()
                } else {
                    cloned_player2_name.deref().clone()
                };

                let now = Utc::now();
                let (is_pm, hour) = now.hour12();
                let time = format!(
                    "{:02}/{:02}/{:02} {:02}:{:02}:{:02} {}",
                    now.month(),
                    now.day(),
                    now.year(),
                    hour,
                    now.minute(),
                    now.second(),
                    if is_pm { "PM" } else { "AM" }
                );

                postGame("connect4".to_string(), cloned_player1_name.deref().clone(), cloned_player2_name.deref().clone(), winner_name, time.to_string(), None);
            }
        }
    });

    let cloned_con4 = con4.clone();
    let cloned_over = over.clone();
    let cloned_winner = winner.clone();
    let reset_board = Callback::from(move |_| {
        let new_board = BoardGame::connect4(6, 8);
        cloned_con4.set(new_board);
        cloned_over.set(false);
        cloned_winner.set(0);
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
            if *over {
                if *winner == 1 {
                    <p>{format!("{} wins - Click on reset button to start over", &*player1_name)}</p>
                } else {
                    <p>{format!("{} wins - Click on reset button to start over", &*player2_name)}</p>
                }
                <ButtonInput class="" label="Reset" onclick={reset_board} />
            } else {
                <ButtonInput class="btn-col" label="0" onclick={add_chip.reform(|_| 0)} />
                <ButtonInput class="btn-col" label="1" onclick={add_chip.reform(|_| 1)} />
                <ButtonInput class="btn-col" label="2" onclick={add_chip.reform(|_| 2)} />
                <ButtonInput class="btn-col" label="3" onclick={add_chip.reform(|_| 3)} />
                <ButtonInput class="btn-col" label="4" onclick={add_chip.reform(|_| 4)} />
                <ButtonInput class="btn-col" label="5" onclick={add_chip.reform(|_| 5)} />
                <ButtonInput class="btn-col" label="6" onclick={add_chip.reform(|_| 6)} />
            }
            
            <table>
                { for con4.board.container.iter().map(|inner_vec| {
                    html! {
                        <tr>
                            { for inner_vec.iter().map(|&value| {
                                if value.is_none() {
                                    html! {
                                        <td></td>
                                    }
                                } else {
                                    if value.unwrap() == Chip::One {
                                        html! {
                                            <td class="red"><center>{"R"}</center></td>
                                        }
                                    } else {
                                        html! {
                                            <td class="yellow"><center>{"Y"}</center></td>
                                        }
                                    }
                                }
                                
                            })}
                        </tr>
                    }
                })}
            </table>
        </div>
    }
}
