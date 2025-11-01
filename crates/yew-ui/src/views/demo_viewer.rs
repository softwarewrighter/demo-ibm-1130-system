use yew::prelude::*;

/// Tab selection for demo viewer
#[allow(dead_code)] // Used in Phase 2b when viewer is integrated
#[derive(Clone, Copy, PartialEq)]
pub enum ViewerTab {
    Code,
    State,
    Output,
    Console,
}

impl ViewerTab {
    #[allow(dead_code)] // Used in Phase 2b when viewer is integrated
    pub fn title(&self) -> &'static str {
        match self {
            ViewerTab::Code => "Code",
            ViewerTab::State => "State",
            ViewerTab::Output => "Output",
            ViewerTab::Console => "Console",
        }
    }
}

/// Execution state of the demo
#[allow(dead_code)] // Used in Phase 2b when viewer is integrated
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ExecutionState {
    NotLoaded,
    Ready,
    Running,
    Paused,
    Completed,
    Error,
}

impl ExecutionState {
    #[allow(dead_code)] // Used in Phase 2b when viewer is integrated
    pub fn can_run(&self) -> bool {
        matches!(self, ExecutionState::Ready | ExecutionState::Paused)
    }

    #[allow(dead_code)] // Used in Phase 2b when viewer is integrated
    pub fn can_step(&self) -> bool {
        matches!(self, ExecutionState::Ready | ExecutionState::Paused)
    }

    #[allow(dead_code)] // Used in Phase 2b when viewer is integrated
    pub fn can_reset(&self) -> bool {
        !matches!(self, ExecutionState::NotLoaded)
    }
}

/// Demo metadata structure
#[allow(dead_code)] // Used in Phase 2b when viewer is integrated
#[derive(Clone, PartialEq)]
pub struct DemoMetadata {
    pub title: String,
    pub description: String,
    pub language: String,
    pub source_code: String,
}

/// Properties for DemoViewer component
#[allow(dead_code)] // Used in Phase 2b when viewer is integrated
#[derive(Properties, PartialEq)]
pub struct DemoViewerProps {
    pub demo_id: Option<String>,
}

/// Main demo viewer component
#[function_component(DemoViewer)]
pub fn demo_viewer(props: &DemoViewerProps) -> Html {
    let current_tab = use_state(|| ViewerTab::Code);
    let execution_state = use_state(|| ExecutionState::NotLoaded);

    // Mock demo metadata for now
    let demo_metadata = if props.demo_id.is_some() {
        Some(DemoMetadata {
            title: "APL Matrix Operations".to_string(),
            description: "Demonstrates APL matrix arithmetic and array operations".to_string(),
            language: "APL".to_string(),
            source_code: "      A <- 3 3 ⍴ ⍳9\n      B <- 3 3 ⍴ 9 8 7 6 5 4 3 2 1\n      A + B\n      A +.× B".to_string(),
        })
    } else {
        None
    };

    let on_tab_change = {
        let current_tab = current_tab.clone();
        Callback::from(move |tab: ViewerTab| {
            current_tab.set(tab);
        })
    };

    let on_load = {
        let execution_state = execution_state.clone();
        Callback::from(move |_| {
            execution_state.set(ExecutionState::Ready);
        })
    };

    let on_run = {
        let execution_state = execution_state.clone();
        Callback::from(move |_| {
            if execution_state.can_run() {
                execution_state.set(ExecutionState::Running);
                // TODO: Start emulator execution
            }
        })
    };

    let on_step = {
        let execution_state = execution_state.clone();
        Callback::from(move |_| {
            if execution_state.can_step() {
                // TODO: Execute one instruction
                execution_state.set(ExecutionState::Paused);
            }
        })
    };

    let on_reset = {
        let execution_state = execution_state.clone();
        Callback::from(move |_| {
            if execution_state.can_reset() {
                execution_state.set(ExecutionState::Ready);
                // TODO: Reset emulator state
            }
        })
    };

    html! {
        <div class="demo-viewer">
            if let Some(metadata) = demo_metadata.as_ref() {
                <div class="viewer-header">
                    <h2>{ &metadata.title }</h2>
                    <p class="viewer-description">{ &metadata.description }</p>
                    <span class="language-badge">{ &metadata.language }</span>
                </div>

                <div class="viewer-controls">
                    <button
                        class="control-button load-button"
                        onclick={on_load}
                        disabled={!matches!(*execution_state, ExecutionState::NotLoaded)}
                    >
                        { "Load" }
                    </button>
                    <button
                        class="control-button run-button"
                        onclick={on_run}
                        disabled={!execution_state.can_run()}
                    >
                        { "Run" }
                    </button>
                    <button
                        class="control-button step-button"
                        onclick={on_step}
                        disabled={!execution_state.can_step()}
                    >
                        { "Step" }
                    </button>
                    <button
                        class="control-button reset-button"
                        onclick={on_reset}
                        disabled={!execution_state.can_reset()}
                    >
                        { "Reset" }
                    </button>
                    <div class="execution-status">
                        { format!("Status: {:?}", *execution_state) }
                    </div>
                </div>

                <div class="viewer-tabs">
                    <button
                        class={classes!("viewer-tab", (*current_tab == ViewerTab::Code).then_some("active"))}
                        onclick={on_tab_change.reform(|_| ViewerTab::Code)}
                    >
                        { "Code" }
                    </button>
                    <button
                        class={classes!("viewer-tab", (*current_tab == ViewerTab::State).then_some("active"))}
                        onclick={on_tab_change.reform(|_| ViewerTab::State)}
                    >
                        { "State" }
                    </button>
                    <button
                        class={classes!("viewer-tab", (*current_tab == ViewerTab::Output).then_some("active"))}
                        onclick={on_tab_change.reform(|_| ViewerTab::Output)}
                    >
                        { "Output" }
                    </button>
                    <button
                        class={classes!("viewer-tab", (*current_tab == ViewerTab::Console).then_some("active"))}
                        onclick={on_tab_change.reform(|_| ViewerTab::Console)}
                    >
                        { "Console" }
                    </button>
                </div>

                <div class="viewer-content">
                    {
                        match *current_tab {
                            ViewerTab::Code => html! { <CodePanel source={metadata.source_code.clone()} /> },
                            ViewerTab::State => html! { <StatePanel /> },
                            ViewerTab::Output => html! { <OutputPanel /> },
                            ViewerTab::Console => html! { <ConsolePanel /> },
                        }
                    }
                </div>
            } else {
                <div class="viewer-empty">
                    <h3>{ "No Demo Selected" }</h3>
                    <p>{ "Select a demo from the browser to view it here." }</p>
                </div>
            }
        </div>
    }
}

/// Code display panel
#[allow(dead_code)] // Used in Phase 2b when viewer is integrated
#[derive(Properties, PartialEq)]
struct CodePanelProps {
    source: String,
}

#[function_component(CodePanel)]
fn code_panel(props: &CodePanelProps) -> Html {
    html! {
        <div class="code-panel">
            <div class="code-toolbar">
                <span class="code-info">{ "Source Code" }</span>
            </div>
            <pre class="code-display">
                <code>{ &props.source }</code>
            </pre>
            <div class="code-footer">
                <span class="code-hint">
                    { "Syntax highlighting will be added in a future update" }
                </span>
            </div>
        </div>
    }
}

/// State visualization panel
#[function_component(StatePanel)]
fn state_panel() -> Html {
    html! {
            <div class="state-panel">
                <div class="state-section">
                    <h4>{ "Registers" }</h4>
                    <div class="register-grid">
                        <div class="register-item">
                            <span class="register-name">{ "ACC" }</span>
                            <span class="register-value">{ "0000" }</span>
                        </div>
                        <div class="register-item">
                            <span class="register-name">{ "EXT" }</span>
                            <span class="register-value">{ "0000" }</span>
                        </div>
                        <div class="register-item">
                            <span class="register-name">{ "IAR" }</span>
                            <span class="register-value">{ "0000" }</span>
                        </div>
                        <div class="register-item">
                            <span class="register-name">{ "XR1" }</span>
                            <span class="register-value">{ "0000" }</span>
                        </div>
                        <div class="register-item">
                            <span class="register-name">{ "XR2" }</span>
                            <span class="register-value">{ "0000" }</span>
                        </div>
                        <div class="register-item">
                            <span class="register-name">{ "XR3" }</span>
                            <span class="register-value">{ "0000" }</span>
                        </div>
                    </div>
                </div>
                <div class="state-section">
                    <h4>{ "Memory" }</h4>
                    <div class="memory-view">
                        <pre>
    { "0000: 0000 0000 0000 0000 0000 0000 0000 0000\n" }
    { "0008: 0000 0000 0000 0000 0000 0000 0000 0000\n" }
    { "0010: 0000 0000 0000 0000 0000 0000 0000 0000" }
                        </pre>
                    </div>
                </div>
                <div class="state-section">
                    <h4>{ "Device Status" }</h4>
                    <div class="device-status">
                        <div class="device-item">
                            <span class="device-name">{ "Card Reader" }</span>
                            <span class="device-state ready">{ "Ready" }</span>
                        </div>
                        <div class="device-item">
                            <span class="device-name">{ "Printer" }</span>
                            <span class="device-state ready">{ "Ready" }</span>
                        </div>
                        <div class="device-item">
                            <span class="device-name">{ "Disk" }</span>
                            <span class="device-state ready">{ "Ready" }</span>
                        </div>
                    </div>
                </div>
                <div class="state-placeholder">
                    <p>{ "Live state visualization will be connected to emulator backend" }</p>
                </div>
            </div>
        }
}

/// Output panel for program results
#[function_component(OutputPanel)]
fn output_panel() -> Html {
    html! {
        <div class="output-panel">
            <div class="output-toolbar">
                <span>{ "Program Output" }</span>
                <button class="clear-output-button">{ "Clear" }</button>
            </div>
            <pre class="output-display">
                <code>{ "Output will appear here when the program runs..." }</code>
            </pre>
        </div>
    }
}

/// Console panel for debugging messages
#[function_component(ConsolePanel)]
fn console_panel() -> Html {
    html! {
        <div class="console-panel">
            <div class="console-toolbar">
                <span>{ "Debug Console" }</span>
                <button class="clear-console-button">{ "Clear" }</button>
            </div>
            <pre class="console-display">
                <code>{ "[INFO] Demo viewer initialized\n[INFO] Ready for emulator connection" }</code>
            </pre>
        </div>
    }
}
