use yew::prelude::*;

#[function_component(StatusBar)]
pub fn status_bar() -> Html {
    html! {
        <div class="status-bar">
            <h3>{ "System Status" }</h3>
            <div class="status-items">
                <div class="status-item">
                    <label>{ "Device: " }</label>
                    <span>{ "Ready" }</span>
                </div>
                <div class="status-item">
                    <label>{ "Timing: " }</label>
                    <span>{ "1.0x" }</span>
                </div>
            </div>
        </div>
    }
}
