use crate::models::educational::{
    Challenge, ChallengeCategory, Difficulty, LearningProgress, Tutorial, TutorialCategory,
};
use yew::prelude::*;

#[function_component(Learn)]
pub fn learn() -> Html {
    // Initialize learning progress (will be persisted to localStorage in future phases)
    let progress = use_state(LearningProgress::new);

    // Sample tutorial categories to display
    let tutorial_categories = [
        TutorialCategory::GettingStarted,
        TutorialCategory::ProgrammingBasics,
        TutorialCategory::DeviceOperations,
        TutorialCategory::AdvancedTopics,
    ];

    // Sample challenge categories to display
    let challenge_categories = [
        ChallengeCategory::CodeGolf,
        ChallengeCategory::SpeedRuns,
        ChallengeCategory::ResourceManagement,
        ChallengeCategory::RealWorldProblems,
    ];

    // Sample tutorials (will be loaded from JSON in future phases)
    let sample_tutorials = [
        Tutorial::new(
            "welcome",
            "Welcome to the IBM 1130",
            TutorialCategory::GettingStarted,
            Difficulty::Beginner,
        ),
        Tutorial::new(
            "first-program",
            "Your First Program",
            TutorialCategory::ProgrammingBasics,
            Difficulty::Beginner,
        ),
    ];

    // Sample challenges (will be loaded from JSON in future phases)
    let sample_challenges = [
        Challenge::new(
            "hello-world",
            "Hello World",
            ChallengeCategory::RealWorldProblems,
            Difficulty::Beginner,
            50,
        ),
        Challenge::new(
            "sum-array",
            "Sum an Array",
            ChallengeCategory::RealWorldProblems,
            Difficulty::Beginner,
            100,
        ),
    ];

    html! {
        <div class="learn-content">
            <section class="learn-section">
                <h2>{ "Learn IBM 1130 Programming" }</h2>
                <p>
                    { "Welcome to the interactive learning environment! This section provides " }
                    { "guided tutorials and hands-on challenges to help you master IBM 1130 " }
                    { "programming and system operation." }
                </p>
                <p>
                    { format!("Current Progress: {} tutorials completed, {} points earned",
                        progress.completed_tutorials.len(),
                        progress.total_points) }
                </p>
            </section>

            <section class="learn-section">
                <h3>{ "Tutorial Categories" }</h3>
                <ul class="feature-list">
                    {
                        tutorial_categories.iter().map(|category| {
                            html! {
                                <li>{ category.display_name() }</li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
                <div class="coming-soon">
                    <p>{ format!("{} sample tutorials available", sample_tutorials.len()) }</p>
                    {
                        // Show completion status for sample tutorials
                        sample_tutorials.iter().map(|tutorial| {
                            let is_completed = progress.is_tutorial_completed(&tutorial.id);
                            let status = if is_completed { "✓ Completed" } else { "Not started" };
                            html! {
                                <p>{ format!("{}: {}", tutorial.title, status) }</p>
                            }
                        }).collect::<Html>()
                    }
                    <p>{ "Full tutorial browser coming soon..." }</p>
                </div>
            </section>

            <section class="learn-section">
                <h3>{ "Challenge Categories" }</h3>
                <ul class="feature-list">
                    {
                        challenge_categories.iter().map(|category| {
                            html! {
                                <li>{ category.display_name() }</li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
                <div class="coming-soon">
                    <p>{ format!("{} sample challenges available", sample_challenges.len()) }</p>
                    {
                        // Show completion status and scores for sample challenges
                        sample_challenges.iter().map(|challenge| {
                            let score_text = if let Some(score) = progress.get_challenge_score(&challenge.id) {
                                format!("✓ Completed - {} points", score)
                            } else if progress.is_challenge_completed(&challenge.id) {
                                "✓ Completed".to_string()
                            } else {
                                "Not started".to_string()
                            };
                            html! {
                                <p>{ format!("{}: {}", challenge.title, score_text) }</p>
                            }
                        }).collect::<Html>()
                    }
                    <p>{ "Full challenge system coming soon..." }</p>
                </div>
            </section>

            <section class="learn-section">
                <h3>{ "Difficulty Levels" }</h3>
                <ul class="feature-list">
                    <li>{ format!("{} - Perfect for first-time users", Difficulty::Beginner.display_name()) }</li>
                    <li>{ format!("{} - For those with basic understanding", Difficulty::Intermediate.display_name()) }</li>
                    <li>{ format!("{} - Expert-level challenges", Difficulty::Advanced.display_name()) }</li>
                </ul>
            </section>

            <section class="learn-section">
                <h3>{ "Demo: Progress Tracking" }</h3>
                <p>{ "Try out the progress tracking system (for demonstration purposes):" }</p>
                <div>
                    <button
                        onclick={{
                            let progress = progress.clone();
                            Callback::from(move |_| {
                                let mut p = (*progress).clone();
                                p.complete_tutorial("welcome".to_string());
                                progress.set(p);
                            })
                        }}
                    >
                        { "Complete 'Welcome' Tutorial" }
                    </button>
                    {" "}
                    <button
                        onclick={{
                            let progress = progress.clone();
                            Callback::from(move |_| {
                                let mut p = (*progress).clone();
                                p.complete_challenge("hello-world".to_string(), 50);
                                progress.set(p);
                            })
                        }}
                    >
                        { "Complete 'Hello World' Challenge" }
                    </button>
                    {" "}
                    <button
                        onclick={{
                            let progress = progress.clone();
                            Callback::from(move |_| {
                                progress.set(LearningProgress::new());
                            })
                        }}
                    >
                        { "Reset Progress" }
                    </button>
                </div>
            </section>
        </div>
    }
}
