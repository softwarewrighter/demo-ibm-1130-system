use yew::prelude::*;

/// Execution state for the emulator
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExecutionState {
    /// Program not loaded
    NotLoaded,
    /// Program loaded and ready to run
    Ready,
    /// Program currently executing
    Running,
    /// Program execution paused
    Paused,
    /// Program completed
    Completed,
    /// Error during execution
    Error,
}

impl ExecutionState {
    /// Check if execution can be started or resumed
    pub fn can_run(&self) -> bool {
        matches!(self, ExecutionState::Ready | ExecutionState::Paused)
    }

    /// Check if execution can be paused
    pub fn can_pause(&self) -> bool {
        matches!(self, ExecutionState::Running)
    }

    /// Check if program is loaded
    pub fn is_loaded(&self) -> bool {
        !matches!(self, ExecutionState::NotLoaded)
    }

    /// Get display label for current state
    pub fn display_label(&self) -> &'static str {
        match self {
            ExecutionState::NotLoaded => "Not Loaded",
            ExecutionState::Ready => "Ready",
            ExecutionState::Running => "Running",
            ExecutionState::Paused => "Paused",
            ExecutionState::Completed => "Completed",
            ExecutionState::Error => "Error",
        }
    }
}

/// Properties for the ControlPanel component
#[derive(Properties, PartialEq)]
pub struct ControlPanelProps {
    /// Current execution state
    pub state: ExecutionState,
    /// Callback when Load button is clicked
    pub on_load: Callback<()>,
    /// Callback when Run button is clicked
    pub on_run: Callback<()>,
    /// Callback when Step button is clicked
    pub on_step: Callback<()>,
    /// Callback when Pause button is clicked
    pub on_pause: Callback<()>,
    /// Callback when Reset button is clicked
    pub on_reset: Callback<()>,
}

/// Control panel component for emulator execution controls
///
/// Provides buttons for Load, Run, Step, Pause, and Reset operations.
/// Button states are automatically managed based on the current ExecutionState.
#[function_component(ControlPanel)]
pub fn control_panel(props: &ControlPanelProps) -> Html {
    let on_load_click = {
        let on_load = props.on_load.clone();
        Callback::from(move |_| on_load.emit(()))
    };

    let on_run_click = {
        let on_run = props.on_run.clone();
        Callback::from(move |_| on_run.emit(()))
    };

    let on_step_click = {
        let on_step = props.on_step.clone();
        Callback::from(move |_| on_step.emit(()))
    };

    let on_pause_click = {
        let on_pause = props.on_pause.clone();
        Callback::from(move |_| on_pause.emit(()))
    };

    let on_reset_click = {
        let on_reset = props.on_reset.clone();
        Callback::from(move |_| on_reset.emit(()))
    };

    html! {
        <div class="control-panel">
            <div class="control-buttons">
                <button
                    class="control-button"
                    onclick={on_load_click}
                    disabled={props.state.is_loaded()}
                >
                    { "Load" }
                </button>
                <button
                    class="control-button control-button-primary"
                    onclick={on_run_click}
                    disabled={!props.state.can_run()}
                >
                    { if props.state == ExecutionState::Paused { "Resume" } else { "Run" } }
                </button>
                <button
                    class="control-button"
                    onclick={on_step_click}
                    disabled={!props.state.can_run()}
                >
                    { "Step" }
                </button>
                <button
                    class="control-button"
                    onclick={on_pause_click}
                    disabled={!props.state.can_pause()}
                >
                    { "Pause" }
                </button>
                <button
                    class="control-button"
                    onclick={on_reset_click}
                    disabled={!props.state.is_loaded()}
                >
                    { "Reset" }
                </button>
            </div>
            <div class="execution-state">
                <span class="state-label">{ "State: " }</span>
                <span class={classes!("state-value", format!("state-{}", props.state.display_label().to_lowercase().replace(' ', "-")))}>
                    { props.state.display_label() }
                </span>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_state_can_run() {
        assert!(!ExecutionState::NotLoaded.can_run());
        assert!(ExecutionState::Ready.can_run());
        assert!(!ExecutionState::Running.can_run());
        assert!(ExecutionState::Paused.can_run());
        assert!(!ExecutionState::Completed.can_run());
        assert!(!ExecutionState::Error.can_run());
    }

    #[test]
    fn test_execution_state_can_pause() {
        assert!(!ExecutionState::NotLoaded.can_pause());
        assert!(!ExecutionState::Ready.can_pause());
        assert!(ExecutionState::Running.can_pause());
        assert!(!ExecutionState::Paused.can_pause());
        assert!(!ExecutionState::Completed.can_pause());
        assert!(!ExecutionState::Error.can_pause());
    }

    #[test]
    fn test_execution_state_is_loaded() {
        assert!(!ExecutionState::NotLoaded.is_loaded());
        assert!(ExecutionState::Ready.is_loaded());
        assert!(ExecutionState::Running.is_loaded());
        assert!(ExecutionState::Paused.is_loaded());
        assert!(ExecutionState::Completed.is_loaded());
        assert!(ExecutionState::Error.is_loaded());
    }

    #[test]
    fn test_execution_state_display_label() {
        assert_eq!(ExecutionState::NotLoaded.display_label(), "Not Loaded");
        assert_eq!(ExecutionState::Ready.display_label(), "Ready");
        assert_eq!(ExecutionState::Running.display_label(), "Running");
        assert_eq!(ExecutionState::Paused.display_label(), "Paused");
        assert_eq!(ExecutionState::Completed.display_label(), "Completed");
        assert_eq!(ExecutionState::Error.display_label(), "Error");
    }

    #[test]
    fn test_control_panel_props_creation() {
        let props = yew::props!(ControlPanelProps {
            state: ExecutionState::Ready,
            on_load: Callback::from(|_| {}),
            on_run: Callback::from(|_| {}),
            on_step: Callback::from(|_| {}),
            on_pause: Callback::from(|_| {}),
            on_reset: Callback::from(|_| {}),
        });

        assert_eq!(props.state, ExecutionState::Ready);
    }
}
