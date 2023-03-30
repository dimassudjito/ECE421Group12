use yew::prelude::*;

#[function_component(OttoHumanPage)]
pub fn otto_human_page() -> Html {
    html! {
        <div>
            <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Player Names"}</b></h5>
            <hr style="width:50px;border:5px solid red" class="w3-round"/>
            <form>
                <input id="textbox1" type="text" placeholder="Player 1's Name" ng-model="newGame.Player1Name"/>
                <input id="textbox2" type="text" placeholder="Player 2's Name" ng-model="newGame.Player2Name"/>
                <input id="startbutton" class="button" type="submit" value="Start Game"/>
            </form>
            <br/>
            <h4>{"New Game: <player1.name> Vs <player2.name>"}</h4>
            <small>{"(Winning Combination: <player1.name> - TOOT and <player2.name> - OTTO)"}</small>
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
