//! Educational content models for tutorials, challenges, and learning progress.

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

/// User's learning progress
#[derive(Clone, Debug, PartialEq, Default)]
pub struct LearningProgress {
    pub completed_tutorials: Vec<String>,
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
