use yew::prelude::*;

#[function_component(Playground)]
pub fn playground() -> Html {
    html! {
        <div class="playground-content">
            <section class="playground-section">
                <h2>{ "IBM 1130 Programming Playground" }</h2>
                <p>
                    { "Welcome to the programming playground! This is your sandbox environment " }
                    { "for experimenting with IBM 1130 assembly code. Write, test, and debug " }
                    { "programs with full access to all simulated hardware devices." }
                </p>
            </section>

            <section class="playground-section">
                <h3>{ "Features" }</h3>
                <ul class="feature-list">
                    <li>{ "Full-featured code editor with syntax highlighting" }</li>
                    <li>{ "Real-time execution and state inspection" }</li>
                    <li>{ "Step-by-step debugging with breakpoints" }</li>
                    <li>{ "Access to all simulated devices (disk, cards, printer)" }</li>
                    <li>{ "Save and load your programs" }</li>
                    <li>{ "Share code with others via URL" }</li>
                </ul>
            </section>

            <section class="playground-section">
                <h3>{ "Getting Started" }</h3>
                <p>
                    { "Start with a blank program or load one of our starter templates:" }
                </p>
                <ul class="feature-list">
                    <li>{ "Hello World - Print text to the line printer" }</li>
                    <li>{ "Memory Copy Loop - Basic loop and addressing" }</li>
                    <li>{ "Disk Read Example - Load data from disk storage" }</li>
                    <li>{ "Card Reader - Process a deck of punch cards" }</li>
                    <li>{ "Subroutine Template - Function calls and returns" }</li>
                </ul>
            </section>

            <div class="coming-soon">
                <p>{ "Interactive playground editor coming soon..." }</p>
                <p class="note">
                    { "The playground will include a split-screen view with code editor on the " }
                    { "left and live emulator state (registers, memory, device status) on the right." }
                </p>
            </div>
        </div>
    }
}
