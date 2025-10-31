use crate::views::{card_reader::CardReader, disk_map::DiskMap, status_bar::StatusBar};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container">
            <header>
                <h1>{ "IBM 1130 Disk & I/O Simulator" }</h1>
            </header>
            <main class="grid">
                <div class="disk-section">
                    <DiskMap />
                </div>
                <aside class="controls-section">
                    <CardReader />
                    <StatusBar />
                </aside>
            </main>
        </div>
    }
}
