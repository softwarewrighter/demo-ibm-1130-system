// Bridge between UI and core-sim (Web Worker / wasm_bindgen channels)
//
// Mock implementation for Phase 2 - simulates basic execution for UI development
// Real implementation with core-sim integration will be added in Phase 6

use crate::components::{control_panel::ExecutionState, emulator_view::RegisterState};

/// Mock emulator bridge for UI development
///
/// This provides a simple simulation of program execution for testing
/// the UI components. It doesn't actually execute IBM 1130 code, but
/// simulates register changes and output.
#[derive(Clone, Debug, PartialEq)]
pub struct MockEmulatorBridge {
    /// Current execution state
    state: ExecutionState,
    /// Current register values
    registers: RegisterState,
    /// Program output buffer
    output: String,
    /// Console/debug messages
    console: String,
    /// Loaded program code
    code: String,
    /// Simulated instruction counter for step execution
    instruction_count: usize,
}

impl MockEmulatorBridge {
    /// Create a new mock emulator bridge
    pub fn new() -> Self {
        Self {
            state: ExecutionState::NotLoaded,
            registers: RegisterState::default(),
            output: String::new(),
            console: String::new(),
            code: String::new(),
            instruction_count: 0,
        }
    }

    /// Load a program into the emulator
    pub fn load(&mut self, code: String) {
        // Check for empty programs
        if code.trim().is_empty() {
            self.state = ExecutionState::Error;
            self.console.push_str("[ERROR] Cannot load empty program\n");
            return;
        }

        self.code = code;
        self.state = ExecutionState::Ready;
        self.registers = RegisterState::default();
        self.output.clear();
        self.console.push_str("[INFO] Program loaded\n");
        self.instruction_count = 0;
    }

    /// Start or resume execution
    pub fn run(&mut self) {
        if self.state.can_run() {
            self.state = ExecutionState::Running;
            self.console.push_str("[INFO] Execution started\n");

            // Simulate execution completing quickly
            self.simulate_execution();
            self.state = ExecutionState::Completed;
            self.console.push_str("[INFO] Execution completed\n");
        }
    }

    /// Execute a single instruction
    pub fn step(&mut self) {
        if self.state.can_run() {
            self.instruction_count += 1;
            self.console
                .push_str(&format!("[INFO] Step {}\n", self.instruction_count));

            // Simulate register changes
            self.registers.pc = self.registers.pc.wrapping_add(1);
            self.registers.acc = self.registers.acc.wrapping_add(1);

            // Complete after 10 steps for demo
            if self.instruction_count >= 10 {
                self.state = ExecutionState::Completed;
                self.console.push_str("[INFO] Program completed\n");
            }
        }
    }

    /// Pause execution
    pub fn pause(&mut self) {
        if self.state.can_pause() {
            self.state = ExecutionState::Paused;
            self.console.push_str("[INFO] Execution paused\n");
        }
    }

    /// Reset to initial state
    pub fn reset(&mut self) {
        if self.state.is_loaded() {
            self.state = ExecutionState::Ready;
            self.registers = RegisterState::default();
            self.output.clear();
            self.console.push_str("[INFO] Program reset\n");
            self.instruction_count = 0;
        }
    }

    /// Get current execution state
    pub fn get_state(&self) -> ExecutionState {
        self.state
    }

    /// Get current register state
    pub fn get_registers(&self) -> RegisterState {
        self.registers.clone()
    }

    /// Get program output
    pub fn get_output(&self) -> &str {
        &self.output
    }

    /// Get console messages
    pub fn get_console(&self) -> &str {
        &self.console
    }

    /// Simulate program execution (mock implementation)
    fn simulate_execution(&mut self) {
        // Simulate some register changes
        self.registers.acc = 0x1234;
        self.registers.ext = 0x5678;
        self.registers.pc = 0x0100;
        self.registers.index_registers = [10, 20, 30];
        self.registers.condition_code = 1;

        // Simulate some output
        if self.code.contains("HELLO") || self.code.contains("Hello") {
            self.output.push_str("Hello, World!\n");
        } else {
            self.output.push_str("Program executed successfully\n");
        }

        self.instruction_count = 10;
    }
}

impl Default for MockEmulatorBridge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_emulator_new() {
        let bridge = MockEmulatorBridge::new();
        assert_eq!(bridge.get_state(), ExecutionState::NotLoaded);
        assert_eq!(bridge.get_registers().acc, 0);
        assert_eq!(bridge.get_output(), "");
    }

    #[test]
    fn test_mock_emulator_load() {
        let mut bridge = MockEmulatorBridge::new();
        bridge.load("LD 100\nWAIT".to_string());

        assert_eq!(bridge.get_state(), ExecutionState::Ready);
        assert!(bridge.get_console().contains("Program loaded"));
    }

    #[test]
    fn test_mock_emulator_run() {
        let mut bridge = MockEmulatorBridge::new();
        bridge.load("WAIT".to_string());
        bridge.run();

        assert_eq!(bridge.get_state(), ExecutionState::Completed);
        assert!(bridge.get_console().contains("Execution started"));
        assert!(bridge.get_console().contains("Execution completed"));
        assert!(!bridge.get_output().is_empty());
    }

    #[test]
    fn test_mock_emulator_step() {
        let mut bridge = MockEmulatorBridge::new();
        bridge.load("WAIT".to_string());

        bridge.step();
        assert_eq!(bridge.get_registers().pc, 1);

        bridge.step();
        assert_eq!(bridge.get_registers().pc, 2);
    }

    #[test]
    fn test_mock_emulator_reset() {
        let mut bridge = MockEmulatorBridge::new();
        bridge.load("WAIT".to_string());
        bridge.run();

        bridge.reset();
        assert_eq!(bridge.get_state(), ExecutionState::Ready);
        assert_eq!(bridge.get_registers().acc, 0);
        assert_eq!(bridge.get_output(), "");
    }

    #[test]
    fn test_mock_emulator_hello_world_detection() {
        let mut bridge = MockEmulatorBridge::new();
        bridge.load("HELLO WORLD".to_string());
        bridge.run();

        assert!(bridge.get_output().contains("Hello, World!"));
    }

    #[test]
    fn test_mock_emulator_step_completion() {
        let mut bridge = MockEmulatorBridge::new();
        bridge.load("WAIT".to_string());

        // Step 10 times to complete
        for _ in 0..10 {
            if bridge.get_state() != ExecutionState::Completed {
                bridge.step();
            }
        }

        assert_eq!(bridge.get_state(), ExecutionState::Completed);
    }

    #[test]
    fn test_mock_emulator_load_empty_program_error() {
        let mut bridge = MockEmulatorBridge::new();
        bridge.load("".to_string());

        assert_eq!(bridge.get_state(), ExecutionState::Error);
        assert!(bridge.get_console().contains("ERROR"));
        assert!(bridge.get_console().contains("empty program"));
    }

    #[test]
    fn test_mock_emulator_load_whitespace_only_error() {
        let mut bridge = MockEmulatorBridge::new();
        bridge.load("   \n\t  ".to_string());

        assert_eq!(bridge.get_state(), ExecutionState::Error);
    }
}
