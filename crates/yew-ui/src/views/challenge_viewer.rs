use crate::components::code_editor::CodeEditor;
use crate::models::educational::Challenge;
use crate::services::bridge::MockEmulatorBridge;
use yew::prelude::*;

/// Properties for the ChallengeViewer component
#[derive(Properties, PartialEq)]
pub struct ChallengeViewerProps {
    /// Challenge to display
    pub challenge: Challenge,
    /// Callback when challenge is completed with score
    pub on_complete: Callback<(String, u16)>,
    /// Callback to return to browser
    pub on_back: Callback<()>,
}

/// Test result for a single test case
#[derive(Clone, Debug, PartialEq)]
struct TestResult {
    test_name: String,
    passed: bool,
    expected: String,
    actual: String,
}

/// Challenge viewer component with code editor and test validation
#[function_component(ChallengeViewer)]
pub fn challenge_viewer(props: &ChallengeViewerProps) -> Html {
    let user_code = use_state(|| props.challenge.starter_code.clone());
    let emulator = use_state(MockEmulatorBridge::new);
    let test_results = use_state(Vec::<TestResult>::new);
    let is_running_tests = use_state(|| false);

    // Handle code change
    let on_code_change = {
        let user_code = user_code.clone();
        Callback::from(move |code: String| {
            user_code.set(code);
        })
    };

    // Handle run tests
    let on_run_tests = {
        let user_code = user_code.clone();
        let emulator = emulator.clone();
        let test_results = test_results.clone();
        let is_running_tests = is_running_tests.clone();
        let challenge = props.challenge.clone();

        Callback::from(move |_| {
            is_running_tests.set(true);
            test_results.set(Vec::new());

            // Load and run the user's code
            let mut emu = MockEmulatorBridge::new();
            emu.load((*user_code).clone());
            emu.run();

            // Run all test cases
            let mut results = Vec::new();
            for test_case in &challenge.test_cases {
                if !test_case.is_hidden {
                    // For now, simple comparison of console output
                    let actual_output = emu.get_console().trim().to_string();
                    let expected_output = test_case.expected_output.trim().to_string();
                    let passed = actual_output == expected_output;

                    results.push(TestResult {
                        test_name: test_case.name.clone(),
                        passed,
                        expected: expected_output,
                        actual: actual_output,
                    });
                }
            }

            test_results.set(results);
            emulator.set(emu);
            is_running_tests.set(false);
        })
    };

    // Handle submit solution
    let on_submit = {
        let test_results = test_results.clone();
        let on_complete = props.on_complete.clone();
        let challenge = props.challenge.clone();

        Callback::from(move |_| {
            // Calculate score based on passing tests
            let total_tests = challenge.test_cases.len();
            let passed_tests = test_results.iter().filter(|r| r.passed).count();

            // Calculate score as percentage of max points
            let score = if total_tests > 0 {
                ((passed_tests as f64 / total_tests as f64) * challenge.points as f64) as u16
            } else {
                0
            };

            on_complete.emit((challenge.id.clone(), score));
        })
    };

    // Check if all visible tests passed
    let all_tests_passed = test_results.iter().all(|r| r.passed) && !test_results.is_empty();

    html! {
        <div class="challenge-viewer">
            <div class="challenge-viewer-header">
                <button class="back-button" onclick={props.on_back.reform(|_| ())}>
                    { "<- Back to Challenges" }
                </button>
                <h2>{ &props.challenge.title }</h2>
                <div class="challenge-points-header">
                    { format!("{} points", props.challenge.points) }
                </div>
            </div>

            <div class="challenge-description-section">
                <h3>{ "Description" }</h3>
                <p>{ &props.challenge.description }</p>

                {
                    if let Some(time_limit) = props.challenge.time_limit_seconds {
                        html! {
                            <div class="time-limit-info">
                                { format!("Time Limit: {} seconds", time_limit) }
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }

                <h4>{ "Test Cases" }</h4>
                <div class="test-cases-list">
                    {
                        props.challenge.test_cases.iter().enumerate().map(|(idx, test)| {
                            if !test.is_hidden {
                                html! {
                                    <div class="test-case-description">
                                        <strong>{ format!("Test {}: ", idx + 1) }</strong>
                                        { &test.description }
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>

            <div class="challenge-workspace">
                <div class="editor-section">
                    <h3>{ "Your Solution" }</h3>
                    <CodeEditor
                        code={(*user_code).clone()}
                        on_change={on_code_change}
                        placeholder="Write your solution here..."
                    />
                </div>

                <div class="test-section">
                    <h3>{ "Test Results" }</h3>
                    <div class="test-controls">
                        <button
                            class="test-button"
                            onclick={on_run_tests}
                            disabled={*is_running_tests}
                        >
                            { if *is_running_tests { "Running Tests..." } else { "Run Tests" } }
                        </button>
                        <button
                            class="submit-button"
                            onclick={on_submit}
                            disabled={!all_tests_passed}
                        >
                            { "Submit Solution" }
                        </button>
                    </div>

                    <div class="test-results-display">
                        {
                            if test_results.is_empty() {
                                html! {
                                    <p class="no-results">{ "No test results yet. Click 'Run Tests' to check your solution." }</p>
                                }
                            } else {
                                html! {
                                    <div class="results-list">
                                        {
                                            test_results.iter().map(|result| {
                                                html! {
                                                    <div class={classes!(
                                                        "test-result",
                                                        if result.passed { "passed" } else { "failed" }
                                                    )}>
                                                        <div class="test-result-header">
                                                            <span class="test-name">{ &result.test_name }</span>
                                                            <span class="test-status">
                                                                { if result.passed { "[PASS]" } else { "[FAIL]" } }
                                                            </span>
                                                        </div>
                                                        {
                                                            if !result.passed {
                                                                html! {
                                                                    <div class="test-result-details">
                                                                        <div class="expected">
                                                                            <strong>{ "Expected: " }</strong>
                                                                            <pre>{ &result.expected }</pre>
                                                                        </div>
                                                                        <div class="actual">
                                                                            <strong>{ "Got: " }</strong>
                                                                            <pre>{ &result.actual }</pre>
                                                                        </div>
                                                                    </div>
                                                                }
                                                            } else {
                                                                html! {}
                                                            }
                                                        }
                                                    </div>
                                                }
                                            }).collect::<Html>()
                                        }
                                        {
                                            if all_tests_passed {
                                                html! {
                                                    <div class="all-passed-message">
                                                        { "[OK] All tests passed! You can submit your solution now." }
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </div>
                                }
                            }
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}
