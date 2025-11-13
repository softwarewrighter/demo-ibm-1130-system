use crate::models::educational::{
    Challenge, ChallengeCategory, Difficulty, LearningProgress, TestCase,
};
use crate::views::{challenge_browser::ChallengeBrowser, challenge_viewer::ChallengeViewer};
use yew::prelude::*;

/// Create sample challenge content for demo
fn create_sample_challenges() -> Vec<Challenge> {
    vec![
        {
            let mut c = Challenge::new(
                "hello-world-challenge",
                "Hello World Challenge",
                ChallengeCategory::Programming,
                Difficulty::Beginner,
                50,
            );
            c.description = "Write a program that prints 'Hello World' to the console using IBM 1130 assembly instructions.".to_string();
            c.starter_code = "        * Your code here\n        WAIT\n".to_string();
            c.test_cases = vec![TestCase {
                name: "Output Test".to_string(),
                description: "Program should output 'Hello World'".to_string(),
                expected_output: "Hello World".to_string(),
                is_hidden: false,
            }];
            c.available = true;
            c
        },
        {
            let mut c = Challenge::new(
                "simple-arithmetic",
                "Simple Arithmetic",
                ChallengeCategory::Programming,
                Difficulty::Beginner,
                75,
            );
            c.description =
                "Write a program that adds two numbers (5 and 10) and outputs the result (15)."
                    .to_string();
            c.starter_code = "        * Add 5 + 10\n        WAIT\n".to_string();
            c.test_cases = vec![TestCase {
                name: "Addition Test".to_string(),
                description: "Program should output '15'".to_string(),
                expected_output: "15".to_string(),
                is_hidden: false,
            }];
            c.available = true;
            c
        },
        {
            let mut c = Challenge::new(
                "optimize-loop",
                "Loop Optimization",
                ChallengeCategory::Optimization,
                Difficulty::Intermediate,
                150,
            );
            c.description =
                "Optimize a loop to count from 1 to 10. Minimize the number of instructions used."
                    .to_string();
            c.starter_code = "        * Optimized counting loop\n        WAIT\n".to_string();
            c.time_limit_seconds = Some(60);
            c.test_cases = vec![
                TestCase {
                    name: "Correctness Test".to_string(),
                    description: "Program should count from 1 to 10".to_string(),
                    expected_output: "1 2 3 4 5 6 7 8 9 10".to_string(),
                    is_hidden: false,
                },
                TestCase {
                    name: "Performance Test".to_string(),
                    description: "Program should use fewer than 20 instructions".to_string(),
                    expected_output: "1 2 3 4 5 6 7 8 9 10".to_string(),
                    is_hidden: true,
                },
            ];
            c.available = true;
            c
        },
        {
            let mut c = Challenge::new(
                "debug-program",
                "Debug the Broken Program",
                ChallengeCategory::Debugging,
                Difficulty::Intermediate,
                100,
            );
            c.description =
                "The following program is supposed to subtract 5 from 10, but it has bugs. Fix it!"
                    .to_string();
            c.starter_code = r#"        LD   10
        SUB  5
        * Bug: Missing STO instruction
        WAIT
"#
            .to_string();
            c.test_cases = vec![TestCase {
                name: "Subtraction Test".to_string(),
                description: "Program should output '5'".to_string(),
                expected_output: "5".to_string(),
                is_hidden: false,
            }];
            c.available = true;
            c
        },
        {
            let mut c = Challenge::new(
                "advanced-indexing",
                "Array Processing with Index Registers",
                ChallengeCategory::Programming,
                Difficulty::Advanced,
                200,
            );
            c.description =
                "Process an array of numbers using index registers. Sum all elements in the array."
                    .to_string();
            c.starter_code = "        * Process array\n        WAIT\n".to_string();
            c.test_cases = vec![TestCase {
                name: "Array Sum Test".to_string(),
                description: "Sum array [1, 2, 3, 4, 5] should be 15".to_string(),
                expected_output: "15".to_string(),
                is_hidden: false,
            }];
            c.available = false; // Locked until prerequisites met
            c
        },
    ]
}

#[function_component(Challenges)]
pub fn challenges() -> Html {
    // Initialize learning progress
    let progress = use_state(LearningProgress::new);
    let selected_challenge_id = use_state(|| None::<String>);

    // Get all challenges
    let all_challenges = create_sample_challenges();

    // Extract metadata for browser
    let challenge_list = all_challenges.clone();

    // Handle challenge selection
    let on_select_challenge = {
        let selected_challenge_id = selected_challenge_id.clone();
        Callback::from(move |challenge_id: String| {
            selected_challenge_id.set(Some(challenge_id));
        })
    };

    // Handle challenge completion
    let on_complete_challenge = {
        let progress = progress.clone();
        let selected_challenge_id = selected_challenge_id.clone();
        Callback::from(move |(challenge_id, score): (String, u16)| {
            let mut new_progress = (*progress).clone();
            new_progress.complete_challenge(challenge_id, score);
            progress.set(new_progress);
            selected_challenge_id.set(None);
        })
    };

    // Handle back to browser
    let on_back_to_browser = {
        let selected_challenge_id = selected_challenge_id.clone();
        Callback::from(move |_| {
            selected_challenge_id.set(None);
        })
    };

    html! {
        <div class="challenges-view">
            {
                if let Some(ref challenge_id) = *selected_challenge_id {
                    // Find the selected challenge
                    if let Some(challenge) = all_challenges.iter().find(|c| &c.id == challenge_id) {
                        html! {
                            <ChallengeViewer
                                challenge={challenge.clone()}
                                on_complete={on_complete_challenge}
                                on_back={on_back_to_browser}
                            />
                        }
                    } else {
                        html! {
                            <div class="error">{ "Challenge not found" }</div>
                        }
                    }
                } else {
                    html! {
                        <ChallengeBrowser
                            challenges={challenge_list}
                            progress={(*progress).clone()}
                            on_select={on_select_challenge}
                        />
                    }
                }
            }
        </div>
    }
}
