use crate::views::{
    challenges::Challenges,
    demos::Demos,
    hardware::{Hardware, HardwareDevice},
    header_nav::{HeaderNav, Tab},
    learn::Learn,
    overview::Overview,
    playground::Playground,
    reference::Reference,
};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let current_tab = use_state(|| Tab::Overview);
    let selected_device = use_state(|| HardwareDevice::Cpu1131);

    let on_tab_change = {
        let current_tab = current_tab.clone();
        Callback::from(move |tab: Tab| {
            current_tab.set(tab);
        })
    };

    let on_device_change = {
        let selected_device = selected_device.clone();
        Callback::from(move |device: HardwareDevice| {
            selected_device.set(device);
        })
    };

    html! {
        <div class="app-container">
            <HeaderNav current_tab={*current_tab} on_tab_change={on_tab_change} />
            <main class="main-content">
                {
                    match *current_tab {
                        Tab::Overview => html! { <Overview /> },
                        Tab::Hardware => html! {
                            <Hardware
                                selected_device={*selected_device}
                                on_device_change={on_device_change}
                            />
                        },
                        Tab::Demos => html! { <Demos /> },
                        Tab::Learn => html! { <Learn /> },
                        Tab::Challenges => html! { <Challenges /> },
                        Tab::Playground => html! { <Playground /> },
                        Tab::Reference => html! { <Reference /> },
                    }
                }
            </main>
            <footer class="app-footer">
                <div class="footer-content">
                    <div class="footer-links">
                        <a href="https://github.com/softwarewrighter/demo-ibm-1130-system" target="_blank" rel="noopener noreferrer">
                            { "GitHub Repository" }
                        </a>
                        <span class="separator">{ " | " }</span>
                        <a href="https://github.com/softwarewrighter/demo-ibm-1130-system/blob/main/LICENSE" target="_blank" rel="noopener noreferrer">
                            { "MIT License" }
                        </a>
                        <span class="separator">{ " | " }</span>
                        <a href="https://github.com/softwarewrighter/demo-ibm-1130-system/blob/main/licensed-media/LICENSE.txt" target="_blank" rel="noopener noreferrer">
                            { "Media Licenses" }
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
