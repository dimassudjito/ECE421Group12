use std::ops::Deref;
use yew::prelude::*;

use crate::boardgame::*;
use crate::chip::*;
use crate::components::button_input::ButtonInput;
use crate::components::text_input::TextInput;
use crate::requests::postGame;

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

    let chip_type = use_state(|| Chip::Two);
    let cloned_chip_type = chip_type.clone();
    let change_chip_type = Callback::from(move |chip: Chip| {
        cloned_chip_type.set(chip);
    });

    let start = use_state(|| false);
    let cloned_start = start.clone();
    let start_game = Callback::from(move |_| {
        cloned_start.set(true);
    });

    let toot_otto = use_state(|| BoardGame::toot_otto(6, 7));
    let over = use_state(|| false);
    let winner = use_state(|| 0);

    let cloned_toot_otto = toot_otto.clone();
    let cloned_over = over.clone();
    let cloned_winner = winner.clone();
    let cloned_player1_name = player1_name.clone();
    let cloned_player2_name = player2_name.clone();
    let cloned_chip_type = chip_type.clone();
    let add_chip = Callback::from(move |col: usize| {
        let mut data = cloned_toot_otto.deref().clone();
        let chip = cloned_chip_type.deref().clone();
        let res = data.insert(col, chip);
        cloned_toot_otto.set(data);
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
                postGame(
                    "toot otto".to_string(),
                    cloned_player1_name.deref().clone(),
                    cloned_player2_name.deref().clone(),
                    winner_name,
                    None,
                );
            }
        }
    });

    let cloned_toot_otto = toot_otto.clone();
    let cloned_over = over.clone();
    let cloned_winner = winner.clone();
    let reset_board = Callback::from(move |_| {
        let new_board = BoardGame::toot_otto(6, 7);
        cloned_toot_otto.set(new_board);
        cloned_over.set(false);
        cloned_winner.set(0);
    });

    html! {
        <div>
            <h1><b>{"Enter Player Names"}</b></h1>
            <hr/>
            <TextInput handle_onchange={player1_name_changed} id="textbox1" placeholder="Player 1's Name" />
            <TextInput handle_onchange={player2_name_changed} id="textbox2" placeholder="Player 2's Name"/>
            <ButtonInput class="btn-start" label="Start Game" onclick={start_game} />
            if *start {
                <br/>
                <h3>{format!("New Game: {} Vs {}", &*player1_name, &*player2_name)}</h3>
                <small>{format!("(Disc Colors: {} - TOOT and {} - OTTO)", &*player1_name, &*player2_name)}</small>
                <br/>
                if *over {
                    if *winner == 1 {
                        <p>{format!("{} wins - Click on reset button to start over", &*player1_name)}</p>
                    } else {
                        <p>{format!("{} wins - Click on reset button to start over", &*player2_name)}</p>
                    }
                    <ButtonInput class="btn-reset" label="Reset" onclick={reset_board} />
                } else {
                    <ButtonInput class={if *chip_type == Chip::Two {"btn-start"} else {"btn-normal"}} label="T" onclick={change_chip_type.reform(|_| Chip::Two)} />
                    <ButtonInput class={if *chip_type == Chip::One {"btn-start"} else {"btn-normal"}} label="O" onclick={change_chip_type.reform(|_| Chip::One)} />
                    <br/>
                    <ButtonInput class="btn-col" label="0" onclick={add_chip.reform(|_| 0)} />
                    <ButtonInput class="btn-col" label="1" onclick={add_chip.reform(|_| 1)} />
                    <ButtonInput class="btn-col" label="2" onclick={add_chip.reform(|_| 2)} />
                    <ButtonInput class="btn-col" label="3" onclick={add_chip.reform(|_| 3)} />
                    <ButtonInput class="btn-col" label="4" onclick={add_chip.reform(|_| 4)} />
                    <ButtonInput class="btn-col" label="5" onclick={add_chip.reform(|_| 5)} />
                    <ButtonInput class="btn-col" label="6" onclick={add_chip.reform(|_| 6)} />
                }

                <table>
                    { for toot_otto.board.container.iter().map(|inner_vec| {
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
                                                <td class="yellow"><center>{"O"}</center></td>
                                            }
                                        } else {
                                            html! {
                                                <td class="red"><center>{"T"}</center></td>
                                            }
                                        }
                                    }

                                })}
                            </tr>
                        }
                    })}
                </table>
            }
        </div>
    }
}
