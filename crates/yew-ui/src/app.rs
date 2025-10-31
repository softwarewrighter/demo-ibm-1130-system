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
            <footer>
                <div class="footer-content">
                    <div class="footer-links">
                        <a href="https://github.com/softwarewrighter/demo-ibm-1130-system" target="_blank" rel="noopener noreferrer">
                            { "GitHub Repository" }
                        </a>
                        <span class="separator">{ "|" }</span>
                        <a href="https://github.com/softwarewrighter/demo-ibm-1130-system/blob/main/LICENSE" target="_blank" rel="noopener noreferrer">
                            { "MIT License" }
                        </a>
                    </div>
                    <div class="footer-copyright">
                        { "Copyright © 2025 Michael A. Wright" }
                    </div>
                </div>
            </footer>
        </div>
    }
}
