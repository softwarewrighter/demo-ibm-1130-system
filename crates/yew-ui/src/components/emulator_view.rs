use yew::prelude::*;

/// Register state for the IBM 1130 emulator
#[derive(Clone, Debug, PartialEq, Default)]
pub struct RegisterState {
    /// Accumulator register
    pub acc: u16,
    /// Extension register
    pub ext: u16,
    /// Program counter
    pub pc: u16,
    /// Index registers (XR1, XR2, XR3)
    pub index_registers: [u16; 3],
    /// Condition code
    pub condition_code: u8,
}

/// View mode for the emulator state
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewMode {
    /// Show register values
    Registers,
    /// Show memory contents
    Memory,
    /// Show device status
    Devices,
    /// Show program output
    Output,
}

/// Properties for the EmulatorView component
#[derive(Properties, PartialEq)]
pub struct EmulatorViewProps {
    /// Current register state
    pub registers: RegisterState,
    /// Program output text
    #[prop_or_default]
    pub output: String,
    /// Console/debug messages
    #[prop_or_default]
    pub console: String,
}

/// Emulator state view component
///
/// Displays the current state of the emulator including registers,
/// memory, device status, and output.
#[function_component(EmulatorView)]
pub fn emulator_view(props: &EmulatorViewProps) -> Html {
    let current_view = use_state(|| ViewMode::Registers);

    let on_view_change = {
        let current_view = current_view.clone();
        move |mode: ViewMode| {
            current_view.set(mode);
        }
    };

    html! {
        <div class="emulator-view">
            <div class="view-tabs">
                <button
                    class={classes!("view-tab", (*current_view == ViewMode::Registers).then_some("active"))}
                    onclick={{
                        let on_view_change = on_view_change.clone();
                        Callback::from(move |_| on_view_change(ViewMode::Registers))
                    }}
                >
                    { "Registers" }
                </button>
                <button
                    class={classes!("view-tab", (*current_view == ViewMode::Memory).then_some("active"))}
                    onclick={{
                        let on_view_change = on_view_change.clone();
                        Callback::from(move |_| on_view_change(ViewMode::Memory))
                    }}
                >
                    { "Memory" }
                </button>
                <button
                    class={classes!("view-tab", (*current_view == ViewMode::Devices).then_some("active"))}
                    onclick={{
                        let on_view_change = on_view_change.clone();
                        Callback::from(move |_| on_view_change(ViewMode::Devices))
                    }}
                >
                    { "Devices" }
                </button>
                <button
                    class={classes!("view-tab", (*current_view == ViewMode::Output).then_some("active"))}
                    onclick={{
                        let on_view_change = on_view_change.clone();
                        Callback::from(move |_| on_view_change(ViewMode::Output))
                    }}
                >
                    { "Output" }
                </button>
            </div>

            <div class="view-content">
                {
                    match *current_view {
                        ViewMode::Registers => render_registers(&props.registers),
                        ViewMode::Memory => render_memory_placeholder(),
                        ViewMode::Devices => render_devices_placeholder(),
                        ViewMode::Output => render_output(&props.output, &props.console),
                    }
                }
            </div>
        </div>
    }
}

fn render_registers(registers: &RegisterState) -> Html {
    html! {
        <div class="registers-view">
            <div class="register-group">
                <h4>{ "Main Registers" }</h4>
                <div class="register-row">
                    <span class="register-name">{ "ACC:" }</span>
                    <span class="register-value">{ format!("0x{:04X} ({})", registers.acc, registers.acc) }</span>
                </div>
                <div class="register-row">
                    <span class="register-name">{ "EXT:" }</span>
                    <span class="register-value">{ format!("0x{:04X} ({})", registers.ext, registers.ext) }</span>
                </div>
                <div class="register-row">
                    <span class="register-name">{ "PC:" }</span>
                    <span class="register-value">{ format!("0x{:04X} ({})", registers.pc, registers.pc) }</span>
                </div>
            </div>

            <div class="register-group">
                <h4>{ "Index Registers" }</h4>
                <div class="register-row">
                    <span class="register-name">{ "XR1:" }</span>
                    <span class="register-value">{ format!("0x{:04X} ({})", registers.index_registers[0], registers.index_registers[0]) }</span>
                </div>
                <div class="register-row">
                    <span class="register-name">{ "XR2:" }</span>
                    <span class="register-value">{ format!("0x{:04X} ({})", registers.index_registers[1], registers.index_registers[1]) }</span>
                </div>
                <div class="register-row">
                    <span class="register-name">{ "XR3:" }</span>
                    <span class="register-value">{ format!("0x{:04X} ({})", registers.index_registers[2], registers.index_registers[2]) }</span>
                </div>
            </div>

            <div class="register-group">
                <h4>{ "Status" }</h4>
                <div class="register-row">
                    <span class="register-name">{ "CC:" }</span>
                    <span class="register-value">{ format!("{}", registers.condition_code) }</span>
                </div>
            </div>
        </div>
    }
}

fn render_memory_placeholder() -> Html {
    html! {
        <div class="placeholder-view">
            <p>{ "Memory view coming soon..." }</p>
            <p class="note">{ "Will display memory contents in hexadecimal format" }</p>
        </div>
    }
}

fn render_devices_placeholder() -> Html {
    html! {
        <div class="placeholder-view">
            <p>{ "Device status view coming soon..." }</p>
            <p class="note">{ "Will show status of disk, card reader, printer, and other devices" }</p>
        </div>
    }
}

fn render_output(output: &str, console: &str) -> Html {
    html! {
        <div class="output-view">
            <div class="output-section">
                <h4>{ "Program Output" }</h4>
                <pre class="output-text">
                    { if output.is_empty() { "(no output yet)" } else { output } }
                </pre>
            </div>
            <div class="output-section">
                <h4>{ "Console Messages" }</h4>
                <pre class="console-text">
                    { if console.is_empty() { "(no messages)" } else { console } }
                </pre>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_state_default() {
        let registers = RegisterState::default();
        assert_eq!(registers.acc, 0);
        assert_eq!(registers.ext, 0);
        assert_eq!(registers.pc, 0);
        assert_eq!(registers.index_registers, [0, 0, 0]);
        assert_eq!(registers.condition_code, 0);
    }

    #[test]
    fn test_register_state_custom_values() {
        let registers = RegisterState {
            acc: 0x1234,
            ext: 0x5678,
            pc: 0x0100,
            index_registers: [1, 2, 3],
            condition_code: 1,
        };

        assert_eq!(registers.acc, 0x1234);
        assert_eq!(registers.ext, 0x5678);
        assert_eq!(registers.pc, 0x0100);
        assert_eq!(registers.index_registers, [1, 2, 3]);
        assert_eq!(registers.condition_code, 1);
    }

    #[test]
    fn test_view_mode_equality() {
        assert_eq!(ViewMode::Registers, ViewMode::Registers);
        assert_ne!(ViewMode::Registers, ViewMode::Memory);
        assert_ne!(ViewMode::Memory, ViewMode::Devices);
        assert_ne!(ViewMode::Devices, ViewMode::Output);
    }

    #[test]
    fn test_emulator_view_props_creation() {
        let props = yew::props!(EmulatorViewProps {
            registers: RegisterState::default(),
        });

        assert_eq!(props.registers.acc, 0);
        assert_eq!(props.output, "");
        assert_eq!(props.console, "");
    }

    #[test]
    fn test_emulator_view_props_with_output() {
        let props = yew::props!(EmulatorViewProps {
            registers: RegisterState::default(),
            output: "Hello, World!".to_string(),
            console: "[INFO] Program started".to_string(),
        });

        assert_eq!(props.output, "Hello, World!");
        assert_eq!(props.console, "[INFO] Program started");
    }
}
