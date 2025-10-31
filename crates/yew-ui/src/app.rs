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
                    <div class="hardware-info">
                        <h3>{ "IBM 2315 Disk Cartridge" }</h3>
                        <img
                            src="/demo-ibm-1130-system/licensed-media/ibm-2315-cartridge.jpg"
                            alt="IBM 2315 disk cartridge - Removable disk cartridge used in an IBM 1130"
                            class="hardware-image"
                        />
                        <p class="image-attribution">
                            <a href="https://commons.wikimedia.org/wiki/File:Disk_Cartridge_2315_type.jb.jpg"
                               target="_blank"
                               rel="noopener noreferrer">
                                { "Photo by James Berlin" }
                            </a>
                            { " / " }
                            <a href="https://creativecommons.org/licenses/by-sa/3.0/"
                               target="_blank"
                               rel="noopener noreferrer">
                                { "CC BY-SA 3.0" }
                            </a>
                        </p>
                    </div>
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
                        <span class="separator">{ "|" }</span>
                        <a href="https://github.com/softwarewrighter/demo-ibm-1130-system/blob/main/licensed-media/LICENSE-CC-BY-SA-3.0.txt" target="_blank" rel="noopener noreferrer">
                            { "Media: CC BY-SA 3.0" }
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
