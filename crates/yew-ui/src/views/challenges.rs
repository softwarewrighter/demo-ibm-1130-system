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
        {
            let mut c = Challenge::new(
                "multiplication-table",
                "Multiplication Table",
                ChallengeCategory::Programming,
                Difficulty::Intermediate,
                120,
            );
            c.description =
                "Create a program that multiplies 7 by 8 and outputs the result (56).".to_string();
            c.starter_code = "        * Multiply 7 * 8\n        WAIT\n".to_string();
            c.test_cases = vec![TestCase {
                name: "Multiplication Test".to_string(),
                description: "Program should output '56'".to_string(),
                expected_output: "56".to_string(),
                is_hidden: false,
            }];
            c.available = true;
            c
        },
        {
            let mut c = Challenge::new(
                "conditional-max",
                "Find Maximum Value",
                ChallengeCategory::Programming,
                Difficulty::Intermediate,
                130,
            );
            c.description = "Write a program that finds the maximum of two numbers: 42 and 37. Output the larger value."
                .to_string();
            c.starter_code = "        * Find max of 42 and 37\n        WAIT\n".to_string();
            c.test_cases = vec![TestCase {
                name: "Maximum Test".to_string(),
                description: "Program should output '42'".to_string(),
                expected_output: "42".to_string(),
                is_hidden: false,
            }];
            c.available = true;
            c
        },
        {
            let mut c = Challenge::new(
                "optimize-sum",
                "Optimized Summation",
                ChallengeCategory::Optimization,
                Difficulty::Intermediate,
                140,
            );
            c.description =
                "Sum the numbers 1 through 5 using the fewest instructions possible.".to_string();
            c.starter_code = "        * Sum 1+2+3+4+5\n        WAIT\n".to_string();
            c.time_limit_seconds = Some(45);
            c.test_cases = vec![
                TestCase {
                    name: "Correctness Test".to_string(),
                    description: "Program should output '15'".to_string(),
                    expected_output: "15".to_string(),
                    is_hidden: false,
                },
                TestCase {
                    name: "Efficiency Test".to_string(),
                    description: "Should use minimal instructions".to_string(),
                    expected_output: "15".to_string(),
                    is_hidden: true,
                },
            ];
            c.available = true;
            c
        },
        {
            let mut c = Challenge::new(
                "optimize-multiply",
                "Fast Multiplication by Powers of 2",
                ChallengeCategory::Optimization,
                Difficulty::Advanced,
                180,
            );
            c.description = "Multiply a number by 16 using shifts instead of MPY instruction. Input: 5, Output: 80."
                .to_string();
            c.starter_code = "        * Multiply 5 by 16 using shifts\n        WAIT\n".to_string();
            c.time_limit_seconds = Some(90);
            c.test_cases = vec![TestCase {
                name: "Shift Optimization Test".to_string(),
                description: "Program should output '80'".to_string(),
                expected_output: "80".to_string(),
                is_hidden: false,
            }];
            c.available = true;
            c
        },
        {
            let mut c = Challenge::new(
                "debug-division",
                "Fix Division by Zero",
                ChallengeCategory::Debugging,
                Difficulty::Intermediate,
                110,
            );
            c.description = "The program attempts to divide 100 by 0, causing an error. Fix it to divide by 4 instead."
                .to_string();
            c.starter_code = r#"        LD   =100
        DIV  =0        * Bug: Division by zero!
        WAIT
"#
            .to_string();
            c.test_cases = vec![TestCase {
                name: "Division Fix Test".to_string(),
                description: "Program should output '25' (100/4)".to_string(),
                expected_output: "25".to_string(),
                is_hidden: false,
            }];
            c.available = true;
            c
        },
        {
            let mut c = Challenge::new(
                "debug-loop-error",
                "Fix Infinite Loop",
                ChallengeCategory::Debugging,
                Difficulty::Advanced,
                160,
            );
            c.description =
                "This loop is supposed to count down from 3 to 1, but has a bug. Fix it!"
                    .to_string();
            c.starter_code = r#"LOOP    LD   COUNT
        ADD  =1        * Bug: Should be SUB!
        STO  COUNT
        BSC  L,LOOP
        WAIT

COUNT   DC   3
"#
            .to_string();
            c.test_cases = vec![TestCase {
                name: "Loop Fix Test".to_string(),
                description: "Program should count down: 3, 2, 1".to_string(),
                expected_output: "3 2 1".to_string(),
                is_hidden: false,
            }];
            c.available = true;
            c
        },
        {
            let mut c = Challenge::new(
                "string-length",
                "Count Characters",
                ChallengeCategory::Programming,
                Difficulty::Beginner,
                85,
            );
            c.description =
                "Count the number of characters in 'IBM' (3 characters) and output the count."
                    .to_string();
            c.starter_code = "        * Count characters in 'IBM'\n        WAIT\n".to_string();
            c.test_cases = vec![TestCase {
                name: "Character Count Test".to_string(),
                description: "Program should output '3'".to_string(),
                expected_output: "3".to_string(),
                is_hidden: false,
            }];
            c.available = true;
            c
        },
        {
            let mut c = Challenge::new(
                "fibonacci",
                "Fibonacci Sequence",
                ChallengeCategory::Programming,
                Difficulty::Advanced,
                220,
            );
            c.description =
                "Calculate the 7th Fibonacci number (0, 1, 1, 2, 3, 5, 8, 13). Output: 13."
                    .to_string();
            c.starter_code = "        * Calculate 7th Fibonacci number\n        WAIT\n".to_string();
            c.test_cases = vec![
                TestCase {
                    name: "Fibonacci Test".to_string(),
                    description: "Program should output '13'".to_string(),
                    expected_output: "13".to_string(),
                    is_hidden: false,
                },
                TestCase {
                    name: "Correctness Test".to_string(),
                    description: "Verify calculation method".to_string(),
                    expected_output: "13".to_string(),
                    is_hidden: true,
                },
            ];
            c.available = true;
            c
        },
        {
            let mut c = Challenge::new(
                "debug-store-error",
                "Missing Store Instruction",
                ChallengeCategory::Debugging,
                Difficulty::Beginner,
                90,
            );
            c.description = "The program loads 99 but never stores it. Add the missing STO instruction to save it to address 200."
                .to_string();
            c.starter_code = r#"        LD   =99
        * Bug: Missing STO 200
        WAIT
"#
            .to_string();
            c.test_cases = vec![TestCase {
                name: "Store Fix Test".to_string(),
                description: "Program should store 99".to_string(),
                expected_output: "99".to_string(),
                is_hidden: false,
            }];
            c.available = true;
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
