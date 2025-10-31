use yew::prelude::*;

#[function_component(Console)]
pub fn console() -> Html {
    html! {
        <div class="console">
            <h3>{ "Console & DSW Status" }</h3>
            <div class="dsw-display">
                <p>{ "TODO: Device Status Word visualization" }</p>
            </div>
        </div>
    }
}
