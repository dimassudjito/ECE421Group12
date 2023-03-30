use yew::prelude::*;
#[function_component]
pub fn HowToConnect4Page() -> Html {
    html! {
        <div  style="margin-top:75px">
            <h5 ><b>{"How to Play Connect 4"}</b></h5>
            <p>{"Connect Four is a two-player connection game in which the players take turns dropping colored discs from the top into a seven-column, six-row vertically suspended grid. The objective of the game is to be the first to form a horizontal, vertical, or diagonal line of four of one's own discs."}
            </p>
            <br/>
            <div><h5>{"To play Connect 4 follow the following steps:"}</h5></div>
            <ul>

                <li>{"A new game describes discs of which color belongs to which player"}</li>

                <li>{"Click on the desired column on the game board to place your disc"}</li>

                <li>{"Try to connect 4 of your colored discs either horizontally or vertically or diagonally"}</li>

            </ul>
            <br/> {"For More information on Connect 4 click "}<a href="https://en.wikipedia.org/wiki/Connect_Four">{"here"}</a>
      </div>

    }
}
