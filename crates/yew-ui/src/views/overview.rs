use yew::prelude::*;

#[function_component(Overview)]
pub fn overview() -> Html {
    html! {
        <div class="overview-content">
            <section class="overview-section">
                <h2>{ "Welcome to the IBM 1130 System Simulator" }</h2>
                <p>
                    { "This is an interactive web-based simulator of the IBM 1130 computing system, " }
                    { "a pioneering minicomputer introduced by IBM in 1965. The simulator provides " }
                    { "accurate timing models, disk geometry, and I/O operations for educational purposes." }
                </p>
            </section>

            <section class="overview-section">
                <h3>{ "About the IBM 1130" }</h3>
                <p>
                    { "The IBM 1130 was a 16-bit minicomputer introduced in February 1965, designed " }
                    { "for scientific and engineering applications. It was one of the first computers " }
                    { "small and affordable enough for individual departments and small organizations." }
                </p>
                <ul class="feature-list">
                    <li>{ "16-bit word architecture with 2 μs cycle time" }</li>
                    <li>{ "Up to 32KB of core memory" }</li>
                    <li>{ "Removable disk storage (IBM 2315 cartridges)" }</li>
                    <li>{ "Card reader/punch, line printer, and plotter support" }</li>
                    <li>{ "Disk Monitor System (DMS) operating system" }</li>
                </ul>
            </section>

            <section class="overview-section">
                <h3>{ "Historical Context" }</h3>
                <p>
                    { "The IBM 1130 was produced from 1965 to 1973 and was widely used in universities, " }
                    { "research laboratories, and engineering departments. It competed with the DEC PDP-8 " }
                    { "and helped establish the minicomputer market. Over 10,000 units were sold during " }
                    { "its production run." }
                </p>
                <p>
                    { "The system featured modular I/O devices that could be configured based on customer " }
                    { "needs, from basic card-based batch processing to sophisticated real-time data " }
                    { "acquisition with graphics display terminals." }
                </p>
            </section>

            <section class="overview-section">
                <h3>{ "About This Simulator" }</h3>
                <p>
                    { "This project recreates the IBM 1130's disk and I/O subsystems using Rust and " }
                    { "WebAssembly. It provides:" }
                </p>
                <ul class="feature-list">
                    <li>{ "Accurate IBM 2310/2311 disk drive timing and geometry" }</li>
                    <li>{ "IBM 1442 card reader/punch simulation" }</li>
                    <li>{ "IBM 1403 and 1132 line printer models" }</li>
                    <li>{ "Visual feedback through a browser-based interface" }</li>
                    <li>{ "Historical photos and documentation" }</li>
                </ul>
                <p>
                    { "Explore the Hardware tab to view detailed information about each component " }
                    { "of the IBM 1130 system." }
                </p>
            </section>
        </div>
    }
}
