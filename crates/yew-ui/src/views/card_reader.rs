use yew::prelude::*;

#[function_component(CardReader)]
pub fn card_reader() -> Html {
    html! {
        <div class="card-reader">
            <h3>{ "IBM 1442 Card Reader/Punch" }</h3>
            <div class="hopper">
                <label>{ "Hopper: " }</label>
                <span>{ "0 cards" }</span>
            </div>
            <div class="stackers">
                <div>
                    <label>{ "Stacker A: " }</label>
                    <span>{ "0 cards" }</span>
                </div>
                <div>
                    <label>{ "Stacker B: " }</label>
                    <span>{ "0 cards" }</span>
                </div>
            </div>
            <button>{ "Load Deck" }</button>
        </div>
    }
}
