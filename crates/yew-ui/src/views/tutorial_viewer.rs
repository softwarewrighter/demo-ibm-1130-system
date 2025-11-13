use crate::components::code_editor::CodeEditor;
use crate::models::educational::{QuizQuestion, TutorialSection, TutorialWithContent};
use crate::services::bridge::MockEmulatorBridge;
use yew::prelude::*;

/// Properties for the TutorialViewer component
#[derive(Properties, PartialEq)]
pub struct TutorialViewerProps {
    /// Tutorial to display
    pub tutorial: TutorialWithContent,
    /// Callback when tutorial is completed
    pub on_complete: Callback<String>,
    /// Callback to return to browser
    pub on_back: Callback<()>,
}

/// Tutorial viewer component with step-by-step navigation
#[function_component(TutorialViewer)]
pub fn tutorial_viewer(props: &TutorialViewerProps) -> Html {
    let current_section = use_state(|| 0_usize);
    let user_code = use_state(String::new);
    let emulator = use_state(MockEmulatorBridge::new);
    let quiz_answers = use_state(Vec::<Option<usize>>::new);
    let show_hint = use_state(|| false);
    let show_solution = use_state(|| false);

    let section_count = props.tutorial.section_count();
    let is_last_section = *current_section >= section_count.saturating_sub(1);

    // Handle next button
    let on_next = {
        let current_section = current_section.clone();
        let on_complete = props.on_complete.clone();
        let tutorial_id = props.tutorial.metadata.id.clone();

        Callback::from(move |_| {
            if *current_section < section_count.saturating_sub(1) {
                current_section.set(*current_section + 1);
            } else {
                // Tutorial completed
                on_complete.emit(tutorial_id.clone());
            }
        })
    };

    // Handle previous button
    let on_prev = {
        let current_section = current_section.clone();
        Callback::from(move |_| {
            if *current_section > 0 {
                current_section.set(*current_section - 1);
            }
        })
    };

    let current_section_content = props.tutorial.sections.get(*current_section);

    html! {
        <div class="tutorial-viewer">
            <div class="tutorial-viewer-header">
                <button class="back-button" onclick={props.on_back.reform(|_| ())}>
                    { "← Back to Tutorials" }
                </button>
                <h2>{ &props.tutorial.metadata.title }</h2>
                <div class="progress-indicator">
                    { format!("Section {} of {}", *current_section + 1, section_count) }
                </div>
            </div>

            <div class="tutorial-content">
                {
                    if let Some(section) = current_section_content {
                        render_section(section, &user_code, &emulator, &quiz_answers, &show_hint, &show_solution)
                    } else {
                        html! { <p>{ "Section not found" }</p> }
                    }
                }
            </div>

            <div class="tutorial-navigation">
                <button
                    class="nav-button"
                    onclick={on_prev}
                    disabled={*current_section == 0}
                >
                    { "Previous" }
                </button>
                <div class="section-dots">
                    {
                        (0..section_count).map(|i| {
                            html! {
                                <span class={classes!(
                                    "section-dot",
                                    (i == *current_section).then_some("active"),
                                    (i < *current_section).then_some("completed")
                                )} />
                            }
                        }).collect::<Html>()
                    }
                </div>
                <button
                    class="nav-button nav-button-primary"
                    onclick={on_next}
                >
                    { if is_last_section { "Complete Tutorial" } else { "Next" } }
                </button>
            </div>
        </div>
    }
}

fn render_section(
    section: &TutorialSection,
    user_code: &UseStateHandle<String>,
    emulator: &UseStateHandle<MockEmulatorBridge>,
    quiz_answers: &UseStateHandle<Vec<Option<usize>>>,
    show_hint: &UseStateHandle<bool>,
    show_solution: &UseStateHandle<bool>,
) -> Html {
    match section {
        TutorialSection::Theory { title, content } => {
            html! {
                <div class="theory-section">
                    <h3>{ title }</h3>
                    <div class="theory-content">
                        <pre class="theory-text">{ content }</pre>
                    </div>
                </div>
            }
        }
        TutorialSection::HandsOn {
            title,
            instructions,
            starter_code,
            hints,
            solution,
        } => {
            let on_code_change = {
                let user_code = user_code.clone();
                Callback::from(move |code: String| {
                    user_code.set(code);
                })
            };

            let on_load = {
                let emulator = emulator.clone();
                let user_code = user_code.clone();
                Callback::from(move |_| {
                    let mut emu = (*emulator).clone();
                    emu.load((*user_code).clone());
                    emulator.set(emu);
                })
            };

            let on_run = {
                let emulator = emulator.clone();
                Callback::from(move |_| {
                    let mut emu = (*emulator).clone();
                    emu.run();
                    emulator.set(emu);
                })
            };

            let on_hint = {
                let show_hint = show_hint.clone();
                Callback::from(move |_| {
                    show_hint.set(true);
                })
            };

            let on_solution = {
                let show_solution = show_solution.clone();
                Callback::from(move |_| {
                    show_solution.set(true);
                })
            };

            html! {
                <div class="hands-on-section">
                    <h3>{ title }</h3>
                    <div class="instructions">
                        <pre class="instructions-text">{ instructions }</pre>
                    </div>

                    <div class="hands-on-workspace">
                        <div class="editor-area">
                            <h4>{ "Your Code" }</h4>
                            <CodeEditor
                                code={if user_code.is_empty() { starter_code.clone() } else { (*user_code).to_string() }}
                                on_change={on_code_change}
                                placeholder="Write your code here..."
                            />
                        </div>
                        <div class="emulator-area">
                            <h4>{ "Test Your Code" }</h4>
                            <div class="simple-controls">
                                <button onclick={on_load}>{ "Load" }</button>
                                <button onclick={on_run}>{ "Run" }</button>
                            </div>
                            <div class="output-display">
                                <pre>{ emulator.get_console() }</pre>
                            </div>
                        </div>
                    </div>

                    <div class="help-section">
                        {
                            if !hints.is_empty() {
                                html! {
                                    <>
                                        <button class="hint-button" onclick={on_hint}>
                                            { "[!] Show Hint" }
                                        </button>
                                        {
                                            if **show_hint {
                                                html! {
                                                    <div class="hint-box">
                                                        { &hints[0] }
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </>
                                }
                            } else {
                                html! {}
                            }
                        }
                        <button class="solution-button" onclick={on_solution}>
                            { "Show Solution" }
                        </button>
                        {
                            if **show_solution {
                                html! {
                                    <div class="solution-box">
                                        <h5>{ "Solution:" }</h5>
                                        <pre>{ solution }</pre>
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                </div>
            }
        }
        TutorialSection::Quiz { title, questions } => {
            html! {
                <div class="quiz-section">
                    <h3>{ title }</h3>
                    {
                        questions.iter().enumerate().map(|(idx, question)| {
                            render_quiz_question(idx, question, quiz_answers)
                        }).collect::<Html>()
                    }
                </div>
            }
        }
    }
}

fn render_quiz_question(
    index: usize,
    question: &QuizQuestion,
    quiz_answers: &UseStateHandle<Vec<Option<usize>>>,
) -> Html {
    match question {
        QuizQuestion::MultipleChoice {
            question: q,
            options,
            correct_index,
        } => {
            let selected_answer = quiz_answers.get(index).and_then(|&a| a);

            html! {
                <div class="quiz-question">
                    <p class="question-text">{ format!("{}. {}", index + 1, q) }</p>
                    <div class="quiz-options">
                        {
                            options.iter().enumerate().map(|(opt_idx, option)| {
                                let is_selected = selected_answer == Some(opt_idx);
                                let is_correct = opt_idx == *correct_index;
                                let show_feedback = selected_answer.is_some();

                                let quiz_answers = quiz_answers.clone();
                                let onclick = Callback::from(move |_| {
                                    let mut answers = (*quiz_answers).clone();
                                    while answers.len() <= index {
                                        answers.push(None);
                                    }
                                    answers[index] = Some(opt_idx);
                                    quiz_answers.set(answers);
                                });

                                html! {
                                    <button
                                        class={classes!(
                                            "quiz-option",
                                            is_selected.then_some("selected"),
                                            (show_feedback && is_selected && is_correct).then_some("correct"),
                                            (show_feedback && is_selected && !is_correct).then_some("incorrect")
                                        )}
                                        onclick={onclick}
                                    >
                                        { option }
                                    </button>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            }
        }
        QuizQuestion::TrueFalse {
            question: q,
            correct_answer,
        } => {
            let selected_answer = quiz_answers.get(index).and_then(|&a| a);

            html! {
                <div class="quiz-question">
                    <p class="question-text">{ format!("{}. {}", index + 1, q) }</p>
                    <div class="quiz-options">
                        {
                            [("True", true), ("False", false)].iter().enumerate().map(|(opt_idx, (label, value))| {
                                let is_selected = selected_answer == Some(opt_idx);
                                let is_correct = *value == *correct_answer;
                                let show_feedback = selected_answer.is_some();

                                let quiz_answers = quiz_answers.clone();
                                let onclick = Callback::from(move |_| {
                                    let mut answers = (*quiz_answers).clone();
                                    while answers.len() <= index {
                                        answers.push(None);
                                    }
                                    answers[index] = Some(opt_idx);
                                    quiz_answers.set(answers);
                                });

                                html! {
                                    <button
                                        class={classes!(
                                            "quiz-option",
                                            is_selected.then_some("selected"),
                                            (show_feedback && is_selected && is_correct).then_some("correct"),
                                            (show_feedback && is_selected && !is_correct).then_some("incorrect")
                                        )}
                                        onclick={onclick}
                                    >
                                        { label }
                                    </button>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            }
        }
    }
}
