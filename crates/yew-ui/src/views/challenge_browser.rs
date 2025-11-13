use crate::models::educational::{Challenge, ChallengeCategory, LearningProgress};
use yew::prelude::*;

/// Properties for the ChallengeBrowser component
#[derive(Properties, PartialEq)]
pub struct ChallengeBrowserProps {
    /// List of available challenges
    pub challenges: Vec<Challenge>,
    /// Current learning progress
    pub progress: LearningProgress,
    /// Callback when a challenge is selected
    pub on_select: Callback<String>,
}

/// Challenge browser component
///
/// Displays challenges grouped by category with completion status and scores
#[function_component(ChallengeBrowser)]
pub fn challenge_browser(props: &ChallengeBrowserProps) -> Html {
    // Group challenges by category
    let categories = [
        ChallengeCategory::Programming,
        ChallengeCategory::Optimization,
        ChallengeCategory::Debugging,
    ];

    html! {
        <div class="challenge-browser">
            <div class="browser-header">
                <h3>{ "Coding Challenges" }</h3>
                <div class="total-points">
                    { format!("Total Points: {}", props.progress.total_points) }
                </div>
            </div>
            {
                categories.iter().map(|&category| {
                    let challenges_in_category: Vec<Challenge> = props.challenges
                        .iter()
                        .filter(|c| c.category == category)
                        .cloned()
                        .collect();

                    if challenges_in_category.is_empty() {
                        return html! {};
                    }

                    html! {
                        <div class="challenge-category">
                            <h4>{ category.display_name() }</h4>
                            <div class="challenge-list">
                                {
                                    challenges_in_category.into_iter().map(|challenge| {
                                        let is_completed = props.progress.is_challenge_completed(&challenge.id);
                                        let score = props.progress.get_challenge_score(&challenge.id);
                                        let challenge_id = challenge.id.clone();
                                        let challenge_available = challenge.available;
                                        let on_select = props.on_select.clone();

                                        html! {
                                            <div
                                                class={classes!("challenge-card", challenge_available.then_some("available"))}
                                                onclick={Callback::from(move |_| {
                                                    if challenge_available {
                                                        on_select.emit(challenge_id.clone());
                                                    }
                                                })}
                                            >
                                                <div class="challenge-header">
                                                    <span class="challenge-title">{ &challenge.title }</span>
                                                    <span class="challenge-points">{ format!("{} pts", challenge.points) }</span>
                                                </div>
                                                <div class="challenge-description">
                                                    { &challenge.description }
                                                </div>
                                                <div class="challenge-meta">
                                                    <span class="difficulty">{ challenge.difficulty.display_name() }</span>
                                                    {
                                                        if let Some(time_limit) = challenge.time_limit_seconds {
                                                            html! {
                                                                <span class="time-limit">
                                                                    { format!("{}s time limit", time_limit) }
                                                                </span>
                                                            }
                                                        } else {
                                                            html! {}
                                                        }
                                                    }
                                                    {
                                                        if is_completed {
                                                            if let Some(s) = score {
                                                                html! {
                                                                    <span class="challenge-badge completed">
                                                                        { format!("[X] {} pts", s) }
                                                                    </span>
                                                                }
                                                            } else {
                                                                html! {
                                                                    <span class="challenge-badge completed">
                                                                        { "[X] Completed" }
                                                                    </span>
                                                                }
                                                            }
                                                        } else if challenge_available {
                                                            html! {
                                                                <span class="challenge-badge available">
                                                                    { "Start" }
                                                                </span>
                                                            }
                                                        } else {
                                                            html! {
                                                                <span class="challenge-badge locked">
                                                                    { "Locked" }
                                                                </span>
                                                            }
                                                        }
                                                    }
                                                </div>
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
