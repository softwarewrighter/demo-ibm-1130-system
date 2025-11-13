use crate::models::educational::{LearningProgress, Tutorial, TutorialCategory};
use yew::prelude::*;

/// Properties for the TutorialBrowser component
#[derive(Properties, PartialEq)]
pub struct TutorialBrowserProps {
    /// List of available tutorials
    pub tutorials: Vec<Tutorial>,
    /// Current learning progress
    pub progress: LearningProgress,
    /// Callback when a tutorial is selected
    pub on_select: Callback<String>,
}

/// Tutorial browser component
///
/// Displays tutorials grouped by category with completion status
#[function_component(TutorialBrowser)]
pub fn tutorial_browser(props: &TutorialBrowserProps) -> Html {
    // Group tutorials by category
    let categories = [
        TutorialCategory::GettingStarted,
        TutorialCategory::ProgrammingBasics,
        TutorialCategory::DeviceOperations,
        TutorialCategory::AdvancedTopics,
    ];

    html! {
        <div class="tutorial-browser">
            <h3>{ "Available Tutorials" }</h3>
            {
                categories.iter().map(|&category| {
                    let tutorials_in_category: Vec<Tutorial> = props.tutorials
                        .iter()
                        .filter(|t| t.category == category)
                        .cloned()
                        .collect();

                    if tutorials_in_category.is_empty() {
                        return html! {};
                    }

                    html! {
                        <div class="tutorial-category">
                            <h4>{ category.display_name() }</h4>
                            <div class="tutorial-list">
                                {
                                    tutorials_in_category.into_iter().map(|tutorial| {
                                        let is_completed = props.progress.is_tutorial_completed(&tutorial.id);
                                        let tutorial_id = tutorial.id.clone();
                                        let tutorial_available = tutorial.available;
                                        let on_select = props.on_select.clone();

                                        html! {
                                            <div
                                                class={classes!("tutorial-card", tutorial_available.then_some("available"))}
                                                onclick={Callback::from(move |_| {
                                                    if tutorial_available {
                                                        on_select.emit(tutorial_id.clone());
                                                    }
                                                })}
                                            >
                                                <div class="tutorial-header">
                                                    <span class="tutorial-title">{ &tutorial.title }</span>
                                                    {
                                                        if is_completed {
                                                            html! { <span class="tutorial-badge completed">{ "[X] Completed" }</span> }
                                                        } else if tutorial_available {
                                                            html! { <span class="tutorial-badge available">{ "Start" }</span> }
                                                        } else {
                                                            html! { <span class="tutorial-badge locked">{ "Locked" }</span> }
                                                        }
                                                    }
                                                </div>
                                                <div class="tutorial-meta">
                                                    <span class="difficulty">{ tutorial.difficulty.display_name() }</span>
                                                    <span class="duration">{ format!("{} min", tutorial.estimated_minutes) }</span>
                                                </div>
                                                {
                                                    if !tutorial.learning_objectives.is_empty() {
                                                        html! {
                                                            <div class="tutorial-objectives">
                                                                <strong>{ "You'll learn:" }</strong>
                                                                <ul>
                                                                    {
                                                                        tutorial.learning_objectives.iter().take(2).map(|obj| {
                                                                            html! { <li>{ obj }</li> }
                                                                        }).collect::<Html>()
                                                                    }
                                                                </ul>
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
                            </div>
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
