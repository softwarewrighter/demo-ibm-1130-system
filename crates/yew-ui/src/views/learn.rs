use crate::models::educational::{
    Difficulty, LearningProgress, QuizQuestion, Tutorial, TutorialCategory, TutorialSection,
    TutorialWithContent,
};
use crate::views::{tutorial_browser::TutorialBrowser, tutorial_viewer::TutorialViewer};
use yew::prelude::*;

/// Create sample tutorial content for demo
fn create_sample_tutorials() -> Vec<TutorialWithContent> {
    vec![
        TutorialWithContent::new(
            {
                let mut t = Tutorial::new(
                    "welcome",
                    "Welcome to the IBM 1130",
                    TutorialCategory::GettingStarted,
                    Difficulty::Beginner,
                );
                t.estimated_minutes = 10;
                t.learning_objectives = vec![
                    "Understand the IBM 1130 system".to_string(),
                    "Learn basic instruction format".to_string(),
                ];
                t.available = true;
                t
            },
            vec![
                TutorialSection::Theory {
                    title: "Introduction to IBM 1130".to_string(),
                    content: r#"The IBM 1130 was a 16-bit minicomputer introduced in 1965.

Key Features:
- 16-bit word architecture
- 2 microsecond cycle time
- Up to 32KB of core memory
- Removable disk storage (2315 cartridges)
- Card reader, line printer, and plotter support

The 1130 was one of the first truly affordable computers for
small businesses and universities."#
                        .to_string(),
                },
                TutorialSection::Theory {
                    title: "Instruction Format".to_string(),
                    content: r#"Every IBM 1130 instruction is 16 bits (1 word):

Format: [Opcode(6 bits)] [Tag(2 bits)] [Displacement(8 bits)]

- Opcode: What operation to perform (LD, STO, ADD, etc.)
- Tag: Index register selection (0=none, 1=XR1, 2=XR2, 3=XR3)
- Displacement: Memory address or offset

Example: LD 100
Loads the value from memory address 100 into the accumulator."#
                        .to_string(),
                },
                TutorialSection::HandsOn {
                    title: "Try Your First Instruction".to_string(),
                    instructions: "Write a simple program that loads the value 42 into the accumulator and halts."
                        .to_string(),
                    starter_code: "        * Your code here\n        WAIT\n".to_string(),
                    hints: vec![
                        "Use the LD instruction with a literal: LD =42".to_string(),
                        "The equals sign (=) means use the value directly, not as a memory address"
                            .to_string(),
                    ],
                    solution: "        LD   =42      * Load literal 42\n        WAIT          * Halt program\n"
                        .to_string(),
                },
                TutorialSection::Quiz {
                    title: "Check Your Understanding".to_string(),
                    questions: vec![
                        QuizQuestion::MultipleChoice {
                            question: "What does the LD instruction do?".to_string(),
                            options: vec![
                                "Load a value into the accumulator".to_string(),
                                "Store a value to memory".to_string(),
                                "Load disk data".to_string(),
                            ],
                            correct_index: 0,
                        },
                        QuizQuestion::TrueFalse {
                            question: "The IBM 1130 uses a 32-bit word architecture".to_string(),
                            correct_answer: false,
                        },
                    ],
                },
            ],
        ),
        TutorialWithContent::new(
            {
                let mut t = Tutorial::new(
                    "first-program",
                    "Writing Your First Program",
                    TutorialCategory::ProgrammingBasics,
                    Difficulty::Beginner,
                );
                t.estimated_minutes = 15;
                t.learning_objectives = vec![
                    "Write a complete program".to_string(),
                    "Understand memory operations".to_string(),
                ];
                t.prerequisites = vec!["welcome".to_string()];
                t.available = true;
                t
            },
            vec![
                TutorialSection::Theory {
                    title: "Program Structure".to_string(),
                    content: r#"A complete IBM 1130 program has:

1. Instructions - Commands for the CPU
2. Data declarations - Initial values and storage
3. Halt instruction - Stop the program

Comments start with * (asterisk).

Example program structure:
        LD   100      * Load value
        STO  200      * Store result
        WAIT          * Halt"#
                        .to_string(),
                },
                TutorialSection::HandsOn {
                    title: "Write a Complete Program".to_string(),
                    instructions: "Write a program that:\n1. Loads the value 10\n2. Stores it to memory address 100\n3. Halts"
                        .to_string(),
                    starter_code: "        * Complete program\n        WAIT\n".to_string(),
                    hints: vec!["Use LD =10 to load 10, STO 100 to store it".to_string()],
                    solution: "        LD   =10      * Load literal 10\n        STO  100      * Store to address 100\n        WAIT          * Halt\n"
                        .to_string(),
                },
            ],
        ),
        TutorialWithContent::new(
            {
                let mut t = Tutorial::new(
                    "arithmetic-ops",
                    "Arithmetic Operations",
                    TutorialCategory::ProgrammingBasics,
                    Difficulty::Intermediate,
                );
                t.estimated_minutes = 20;
                t.learning_objectives = vec![
                    "Perform arithmetic operations".to_string(),
                    "Work with the accumulator and extension".to_string(),
                ];
                t.prerequisites = vec!["first-program".to_string()];
                t.available = true;
                t
            },
            vec![TutorialSection::Theory {
                title: "Arithmetic Instructions".to_string(),
                content: r#"The IBM 1130 provides several arithmetic instructions:

ADD - Add to accumulator
SUB - Subtract from accumulator
MPY - Multiply (result in ACC+EXT)
DIV - Divide (quotient in ACC, remainder in EXT)

Example:
        LD   A         * Load first number
        ADD  B         * Add second number
        STO  RESULT    * Store sum
        WAIT

A       DC   10
B       DC   20
RESULT  BSS  1"#
                    .to_string(),
            }],
        ),
        TutorialWithContent::new(
            {
                let mut t = Tutorial::new(
                    "advanced-indexing",
                    "Advanced Addressing with Index Registers",
                    TutorialCategory::AdvancedTopics,
                    Difficulty::Advanced,
                );
                t.estimated_minutes = 30;
                t.learning_objectives = vec![
                    "Master indirect and indexed addressing".to_string(),
                    "Use multiple index registers efficiently".to_string(),
                ];
                t.prerequisites = vec!["arithmetic-ops".to_string()];
                t.available = false;
                t
            },
            vec![TutorialSection::Theory {
                title: "Index Registers".to_string(),
                content: "Advanced topic - coming soon!".to_string(),
            }],
        ),
    ]
}

#[function_component(Learn)]
pub fn learn() -> Html {
    // Initialize learning progress
    let progress = use_state(LearningProgress::new);
    let selected_tutorial = use_state(|| Option::<String>::None);

    // Get sample tutorials
    let all_tutorials = create_sample_tutorials();
    let tutorial_metadata: Vec<Tutorial> =
        all_tutorials.iter().map(|t| t.metadata.clone()).collect();

    // Handle tutorial selection
    let on_tutorial_select = {
        let selected_tutorial = selected_tutorial.clone();
        Callback::from(move |tutorial_id: String| {
            selected_tutorial.set(Some(tutorial_id));
        })
    };

    // Handle tutorial completion
    let on_tutorial_complete = {
        let progress = progress.clone();
        let selected_tutorial = selected_tutorial.clone();
        Callback::from(move |tutorial_id: String| {
            let mut p = (*progress).clone();
            p.complete_tutorial(tutorial_id);
            progress.set(p);
            selected_tutorial.set(None);
        })
    };

    // Handle back to browser
    let on_back = {
        let selected_tutorial = selected_tutorial.clone();
        Callback::from(move |_| {
            selected_tutorial.set(None);
        })
    };

    html! {
        <div class="learn-container">
            {
                if let Some(tutorial_id) = (*selected_tutorial).as_ref() {
                    // Show tutorial viewer
                    if let Some(tutorial) = all_tutorials.iter().find(|t| &t.metadata.id == tutorial_id) {
                        html! {
                            <TutorialViewer
                                tutorial={tutorial.clone()}
                                on_complete={on_tutorial_complete}
                                on_back={on_back}
                            />
                        }
                    } else {
                        html! { <p>{ "Tutorial not found" }</p> }
                    }
                } else {
                    // Show tutorial browser
                    html! {
                        <>
                            <div class="learn-header">
                                <h2>{ "Learn IBM 1130 Programming" }</h2>
                                <p>
                                    { "Interactive tutorials with theory, hands-on exercises, and quizzes. " }
                                    { format!("You've completed {} tutorials", progress.completed_tutorials.len()) }
                                </p>
                            </div>
                            <TutorialBrowser
                                tutorials={tutorial_metadata}
                                progress={(*progress).clone()}
                                on_select={on_tutorial_select}
                            />
                        </>
                    }
                }
            }
        </div>
    }
}
