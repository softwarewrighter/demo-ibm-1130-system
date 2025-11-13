//! Educational content models for tutorials, challenges, and learning progress.
use std::collections::HashMap;

/// Difficulty level for educational content
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

impl Difficulty {
    /// Get display name for difficulty level
    pub fn display_name(&self) -> &'static str {
        match self {
            Difficulty::Beginner => "Beginner",
            Difficulty::Intermediate => "Intermediate",
            Difficulty::Advanced => "Advanced",
        }
    }
}

/// Category for tutorials
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TutorialCategory {
    GettingStarted,
    ProgrammingBasics,
    DeviceOperations,
    AdvancedTopics,
}

impl TutorialCategory {
    /// Get display name for tutorial category
    pub fn display_name(&self) -> &'static str {
        match self {
            TutorialCategory::GettingStarted => "Getting Started",
            TutorialCategory::ProgrammingBasics => "Programming Basics",
            TutorialCategory::DeviceOperations => "Device Operations",
            TutorialCategory::AdvancedTopics => "Advanced Topics",
        }
    }
}

/// Category for challenges
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChallengeCategory {
    Programming,
    Optimization,
    Debugging,
}

impl ChallengeCategory {
    /// Get display name for challenge category
    pub fn display_name(&self) -> &'static str {
        match self {
            ChallengeCategory::Programming => "Programming Challenges",
            ChallengeCategory::Optimization => "Optimization Challenges",
            ChallengeCategory::Debugging => "Debugging Challenges",
        }
    }
}

/// Tutorial metadata
#[derive(Clone, Debug, PartialEq)]
pub struct Tutorial {
    pub id: String,
    pub title: String,
    pub category: TutorialCategory,
    pub difficulty: Difficulty,
    pub estimated_minutes: u8,
    pub prerequisites: Vec<String>,
    pub learning_objectives: Vec<String>,
    pub available: bool,
}

impl Tutorial {
    /// Create a new tutorial
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        category: TutorialCategory,
        difficulty: Difficulty,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            category,
            difficulty,
            estimated_minutes: 15,
            prerequisites: Vec::new(),
            learning_objectives: Vec::new(),
            available: false,
        }
    }
}

/// Challenge metadata
#[derive(Clone, Debug, PartialEq)]
pub struct Challenge {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: ChallengeCategory,
    pub difficulty: Difficulty,
    pub points: u16,
    pub time_limit_seconds: Option<u32>,
    pub test_cases: Vec<TestCase>,
    pub starter_code: String,
    pub available: bool,
}

/// Test case for challenge validation
#[derive(Clone, Debug, PartialEq)]
pub struct TestCase {
    pub name: String,
    pub description: String,
    pub expected_output: String,
    pub is_hidden: bool,
}

impl Challenge {
    /// Create a new challenge
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        category: ChallengeCategory,
        difficulty: Difficulty,
        points: u16,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: String::new(),
            category,
            difficulty,
            points,
            time_limit_seconds: None,
            test_cases: Vec::new(),
            starter_code: String::new(),
            available: false,
        }
    }
}

/// User's learning progress
#[derive(Clone, Debug, PartialEq, Default)]
pub struct LearningProgress {
    pub completed_tutorials: Vec<String>,
    pub completed_challenges: Vec<String>,
    pub challenge_scores: HashMap<String, u16>,
    pub total_points: u16,
}

impl LearningProgress {
    /// Create new empty progress
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if tutorial is completed
    pub fn is_tutorial_completed(&self, tutorial_id: &str) -> bool {
        self.completed_tutorials.contains(&tutorial_id.to_string())
    }

    /// Mark tutorial as completed
    pub fn complete_tutorial(&mut self, tutorial_id: String) {
        if !self.completed_tutorials.contains(&tutorial_id) {
            self.completed_tutorials.push(tutorial_id);
        }
    }

    /// Check if challenge is completed
    pub fn is_challenge_completed(&self, challenge_id: &str) -> bool {
        self.completed_challenges
            .contains(&challenge_id.to_string())
    }

    /// Mark challenge as completed with score
    pub fn complete_challenge(&mut self, challenge_id: String, score: u16) {
        if !self.completed_challenges.contains(&challenge_id) {
            self.completed_challenges.push(challenge_id.clone());
        }

        // Update score if better than previous
        let current_score = self
            .challenge_scores
            .get(&challenge_id)
            .copied()
            .unwrap_or(0);
        if score > current_score {
            self.challenge_scores.insert(challenge_id, score);
            self.total_points = self
                .total_points
                .saturating_sub(current_score)
                .saturating_add(score);
        }
    }

    /// Get challenge score
    pub fn get_challenge_score(&self, challenge_id: &str) -> Option<u16> {
        self.challenge_scores.get(challenge_id).copied()
    }
}

/// Tutorial section types
#[derive(Clone, Debug, PartialEq)]
pub enum TutorialSection {
    /// Theory explanation
    Theory { title: String, content: String },
    /// Hands-on coding exercise
    HandsOn {
        title: String,
        instructions: String,
        starter_code: String,
        hints: Vec<String>,
        solution: String,
    },
    /// Quiz questions
    Quiz {
        title: String,
        questions: Vec<QuizQuestion>,
    },
}

/// Quiz question types
#[derive(Clone, Debug, PartialEq)]
pub enum QuizQuestion {
    MultipleChoice {
        question: String,
        options: Vec<String>,
        correct_index: usize,
    },
    TrueFalse {
        question: String,
        correct_answer: bool,
    },
}

/// Complete tutorial with sections
#[derive(Clone, Debug, PartialEq)]
pub struct TutorialWithContent {
    pub metadata: Tutorial,
    pub sections: Vec<TutorialSection>,
}

impl TutorialWithContent {
    /// Create a new tutorial with content
    pub fn new(metadata: Tutorial, sections: Vec<TutorialSection>) -> Self {
        Self { metadata, sections }
    }

    /// Get total number of sections
    pub fn section_count(&self) -> usize {
        self.sections.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difficulty_display_name() {
        assert_eq!(Difficulty::Beginner.display_name(), "Beginner");
        assert_eq!(Difficulty::Intermediate.display_name(), "Intermediate");
        assert_eq!(Difficulty::Advanced.display_name(), "Advanced");
    }

    #[test]
    fn test_tutorial_category_display_name() {
        assert_eq!(
            TutorialCategory::GettingStarted.display_name(),
            "Getting Started"
        );
        assert_eq!(
            TutorialCategory::ProgrammingBasics.display_name(),
            "Programming Basics"
        );
    }

    #[test]
    fn test_challenge_category_display_name() {
        assert_eq!(
            ChallengeCategory::Programming.display_name(),
            "Programming Challenges"
        );
        assert_eq!(
            ChallengeCategory::Optimization.display_name(),
            "Optimization Challenges"
        );
        assert_eq!(
            ChallengeCategory::Debugging.display_name(),
            "Debugging Challenges"
        );
    }

    #[test]
    fn test_tutorial_creation() {
        let tutorial = Tutorial::new(
            "tutorial-1",
            "Test Tutorial",
            TutorialCategory::GettingStarted,
            Difficulty::Beginner,
        );
        assert_eq!(tutorial.id, "tutorial-1");
        assert_eq!(tutorial.title, "Test Tutorial");
        assert_eq!(tutorial.category, TutorialCategory::GettingStarted);
        assert_eq!(tutorial.difficulty, Difficulty::Beginner);
        assert_eq!(tutorial.estimated_minutes, 15);
        assert!(!tutorial.available);
    }

    #[test]
    fn test_challenge_creation() {
        let challenge = Challenge::new(
            "challenge-1",
            "Test Challenge",
            ChallengeCategory::Programming,
            Difficulty::Intermediate,
            100,
        );
        assert_eq!(challenge.id, "challenge-1");
        assert_eq!(challenge.title, "Test Challenge");
        assert_eq!(challenge.category, ChallengeCategory::Programming);
        assert_eq!(challenge.difficulty, Difficulty::Intermediate);
        assert_eq!(challenge.points, 100);
        assert!(!challenge.available);
        assert!(challenge.test_cases.is_empty());
    }

    #[test]
    fn test_learning_progress_tutorial_completion() {
        let mut progress = LearningProgress::new();
        assert!(!progress.is_tutorial_completed("tutorial-1"));

        progress.complete_tutorial("tutorial-1".to_string());
        assert!(progress.is_tutorial_completed("tutorial-1"));

        // Should not duplicate
        progress.complete_tutorial("tutorial-1".to_string());
        assert_eq!(progress.completed_tutorials.len(), 1);
    }

    #[test]
    fn test_learning_progress_challenge_completion() {
        let mut progress = LearningProgress::new();
        assert!(!progress.is_challenge_completed("challenge-1"));

        progress.complete_challenge("challenge-1".to_string(), 100);
        assert!(progress.is_challenge_completed("challenge-1"));
        assert_eq!(progress.get_challenge_score("challenge-1"), Some(100));
        assert_eq!(progress.total_points, 100);
    }

    #[test]
    fn test_learning_progress_challenge_score_update() {
        let mut progress = LearningProgress::new();

        // Complete with initial score
        progress.complete_challenge("challenge-1".to_string(), 100);
        assert_eq!(progress.total_points, 100);

        // Update with better score
        progress.complete_challenge("challenge-1".to_string(), 150);
        assert_eq!(progress.get_challenge_score("challenge-1"), Some(150));
        assert_eq!(progress.total_points, 150);
        assert_eq!(progress.completed_challenges.len(), 1); // Should not duplicate
    }

    #[test]
    fn test_learning_progress_challenge_score_no_downgrade() {
        let mut progress = LearningProgress::new();

        progress.complete_challenge("challenge-1".to_string(), 150);
        progress.complete_challenge("challenge-1".to_string(), 100); // Lower score

        // Should keep higher score
        assert_eq!(progress.get_challenge_score("challenge-1"), Some(150));
        assert_eq!(progress.total_points, 150);
    }

    #[test]
    fn test_tutorial_section_theory() {
        let section = TutorialSection::Theory {
            title: "Test Theory".to_string(),
            content: "Theory content".to_string(),
        };

        if let TutorialSection::Theory { title, content } = section {
            assert_eq!(title, "Test Theory");
            assert_eq!(content, "Theory content");
        } else {
            panic!("Expected Theory section");
        }
    }

    #[test]
    fn test_tutorial_section_hands_on() {
        let section = TutorialSection::HandsOn {
            title: "Test Exercise".to_string(),
            instructions: "Do this".to_string(),
            starter_code: "WAIT".to_string(),
            hints: vec!["Hint 1".to_string()],
            solution: "LD 100\nWAIT".to_string(),
        };

        if let TutorialSection::HandsOn {
            title,
            instructions,
            starter_code,
            hints,
            solution,
        } = section
        {
            assert_eq!(title, "Test Exercise");
            assert_eq!(instructions, "Do this");
            assert_eq!(starter_code, "WAIT");
            assert_eq!(hints.len(), 1);
            assert_eq!(solution, "LD 100\nWAIT");
        } else {
            panic!("Expected HandsOn section");
        }
    }

    #[test]
    fn test_quiz_question_multiple_choice() {
        let question = QuizQuestion::MultipleChoice {
            question: "What is 2+2?".to_string(),
            options: vec!["3".to_string(), "4".to_string(), "5".to_string()],
            correct_index: 1,
        };

        if let QuizQuestion::MultipleChoice {
            question: q,
            options,
            correct_index,
        } = question
        {
            assert_eq!(q, "What is 2+2?");
            assert_eq!(options.len(), 3);
            assert_eq!(correct_index, 1);
        } else {
            panic!("Expected MultipleChoice question");
        }
    }

    #[test]
    fn test_quiz_question_true_false() {
        let question = QuizQuestion::TrueFalse {
            question: "Is the sky blue?".to_string(),
            correct_answer: true,
        };

        if let QuizQuestion::TrueFalse {
            question: q,
            correct_answer,
        } = question
        {
            assert_eq!(q, "Is the sky blue?");
            assert!(correct_answer);
        } else {
            panic!("Expected TrueFalse question");
        }
    }

    #[test]
    fn test_tutorial_with_content() {
        let metadata = Tutorial::new(
            "test-tutorial",
            "Test Tutorial",
            TutorialCategory::GettingStarted,
            Difficulty::Beginner,
        );

        let sections = vec![
            TutorialSection::Theory {
                title: "Section 1".to_string(),
                content: "Content 1".to_string(),
            },
            TutorialSection::Quiz {
                title: "Section 2".to_string(),
                questions: vec![],
            },
        ];

        let tutorial = TutorialWithContent::new(metadata, sections);

        assert_eq!(tutorial.metadata.id, "test-tutorial");
        assert_eq!(tutorial.section_count(), 2);
    }
}
