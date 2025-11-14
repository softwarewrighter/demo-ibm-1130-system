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
            },
            TutorialSection::HandsOn {
                title: "Addition Practice".to_string(),
                instructions: "Write a program that adds 15 and 27 and stores the result in memory address 300."
                    .to_string(),
                starter_code: "        * Add 15 + 27\n        WAIT\n".to_string(),
                hints: vec![
                    "Load the first number with LD =15".to_string(),
                    "Add the second number with ADD =27".to_string(),
                    "Store the result with STO 300".to_string(),
                ],
                solution: r#"        LD   =15      * Load first number
        ADD  =27      * Add second number
        STO  300      * Store result
        WAIT          * Halt
"#
                .to_string(),
            },
            TutorialSection::Quiz {
                title: "Arithmetic Quiz".to_string(),
                questions: vec![
                    QuizQuestion::MultipleChoice {
                        question: "Which instruction adds to the accumulator?".to_string(),
                        options: vec![
                            "LD".to_string(),
                            "ADD".to_string(),
                            "STO".to_string(),
                        ],
                        correct_index: 1,
                    },
                    QuizQuestion::TrueFalse {
                        question: "The SUB instruction subtracts from the accumulator".to_string(),
                        correct_answer: true,
                    },
                ],
            }],
        ),
        TutorialWithContent::new(
            {
                let mut t = Tutorial::new(
                    "branching-control",
                    "Branching and Control Flow",
                    TutorialCategory::ProgrammingBasics,
                    Difficulty::Intermediate,
                );
                t.estimated_minutes = 25;
                t.learning_objectives = vec![
                    "Use conditional and unconditional branches".to_string(),
                    "Implement loops and decision structures".to_string(),
                ];
                t.prerequisites = vec!["arithmetic-ops".to_string()];
                t.available = true;
                t
            },
            vec![
                TutorialSection::Theory {
                    title: "Branch Instructions".to_string(),
                    content: r#"Control flow instructions change program execution order:

B    - Unconditional branch (always jump)
BSC  - Branch and skip on condition
BC   - Branch on condition
MDX  - Modify index and skip

Condition codes are set by arithmetic operations:
- Zero
- Positive
- Negative
- Carry

Example loop:
LOOP    LD   COUNTER
        SUB  =1
        STO  COUNTER
        BSC  L,LOOP    * Branch if not zero
        WAIT

COUNTER DC   10"#
                        .to_string(),
                },
                TutorialSection::HandsOn {
                    title: "Simple Loop".to_string(),
                    instructions: "Write a program that counts down from 5 to 1 using a loop."
                        .to_string(),
                    starter_code: "        * Count down from 5\n        WAIT\n".to_string(),
                    hints: vec![
                        "Use a counter variable starting at 5".to_string(),
                        "Decrement with SUB =1".to_string(),
                        "Use BSC to loop while not zero".to_string(),
                    ],
                    solution: r#"LOOP    LD   COUNT
        SUB  =1
        STO  COUNT
        BSC  L,LOOP    * Loop if not zero
        WAIT

COUNT   DC   5
"#
                    .to_string(),
                },
            ],
        ),
        TutorialWithContent::new(
            {
                let mut t = Tutorial::new(
                    "memory-addressing",
                    "Memory Addressing Modes",
                    TutorialCategory::GettingStarted,
                    Difficulty::Beginner,
                );
                t.estimated_minutes = 18;
                t.learning_objectives = vec![
                    "Understand direct and indirect addressing".to_string(),
                    "Use literals and memory references correctly".to_string(),
                ];
                t.prerequisites = vec!["first-program".to_string()];
                t.available = true;
                t
            },
            vec![
                TutorialSection::Theory {
                    title: "Addressing Modes".to_string(),
                    content: r#"The IBM 1130 supports several addressing modes:

Direct - Use the value at an address
    LD  100        * Load from address 100

Literal - Use the value directly
    LD  =100       * Load the number 100

Indirect - Use address stored at an address
    LD  /100       * Load from address stored at 100

Indexed - Add index register to address
    LD  100,1      * Load from address 100 + XR1

These can be combined for powerful memory access patterns."#
                        .to_string(),
                },
                TutorialSection::Quiz {
                    title: "Addressing Quiz".to_string(),
                    questions: vec![
                        QuizQuestion::MultipleChoice {
                            question: "What does LD =50 do?".to_string(),
                            options: vec![
                                "Load from memory address 50".to_string(),
                                "Load the literal value 50".to_string(),
                                "Load indirectly from 50".to_string(),
                            ],
                            correct_index: 1,
                        },
                        QuizQuestion::TrueFalse {
                            question: "The / symbol indicates indirect addressing".to_string(),
                            correct_answer: true,
                        },
                    ],
                },
            ],
        ),
        TutorialWithContent::new(
            {
                let mut t = Tutorial::new(
                    "disk-operations",
                    "Disk I/O Operations",
                    TutorialCategory::DeviceOperations,
                    Difficulty::Intermediate,
                );
                t.estimated_minutes = 30;
                t.learning_objectives = vec![
                    "Read and write disk sectors".to_string(),
                    "Understand disk addressing (cylinder, head, sector)".to_string(),
                ];
                t.prerequisites = vec!["branching-control".to_string()];
                t.available = true;
                t
            },
            vec![
                TutorialSection::Theory {
                    title: "Disk Storage".to_string(),
                    content: r#"The IBM 2310 disk system uses removable 2315 cartridges:

Geometry:
- 200 cylinders
- 2 heads (surfaces)
- 4 sectors per track
- 320 words per sector

Addressing:
Specify Cylinder + Head + Sector to locate data

I/O Commands:
- READ  - Read sector into memory
- WRITE - Write memory to sector
- SEEK  - Position disk heads

Example:
        XIO  READ,DISK,100  * Read to address 100
        WAIT"#
                        .to_string(),
                },
                TutorialSection::HandsOn {
                    title: "Disk Read Operation".to_string(),
                    instructions: "Write a program that initiates a disk read to memory address 500."
                        .to_string(),
                    starter_code: "        * Read from disk\n        WAIT\n".to_string(),
                    hints: vec![
                        "Use XIO instruction for I/O operations".to_string(),
                        "Syntax: XIO READ,device,address".to_string(),
                    ],
                    solution: r#"        XIO  READ,DISK,500  * Read to address 500
        WAIT                * Halt
"#
                    .to_string(),
                },
            ],
        ),
        TutorialWithContent::new(
            {
                let mut t = Tutorial::new(
                    "index-registers",
                    "Working with Index Registers",
                    TutorialCategory::ProgrammingBasics,
                    Difficulty::Intermediate,
                );
                t.estimated_minutes = 22;
                t.learning_objectives = vec![
                    "Use index registers for array access".to_string(),
                    "Implement indexed loops".to_string(),
                ];
                t.prerequisites = vec!["branching-control".to_string()];
                t.available = true;
                t
            },
            vec![
                TutorialSection::Theory {
                    title: "Index Registers".to_string(),
                    content: r#"The IBM 1130 has three index registers (XR1, XR2, XR3):

Purpose:
- Array indexing
- Loop counters
- Dynamic addressing

Operations:
LDX  - Load index register
STX  - Store index register
MDX  - Modify index and skip

Indexed Addressing:
    LD  ARRAY,1    * Load from ARRAY + XR1

Example array access:
        LDX  =0,1      * XR1 = 0
LOOP    LD   ARRAY,1   * Load ARRAY[XR1]
        MDX  1,=1      * XR1++, skip if zero
        B    LOOP
        WAIT

ARRAY   DC   10,20,30,40,50"#
                        .to_string(),
                },
                TutorialSection::HandsOn {
                    title: "Array Sum".to_string(),
                    instructions: "Write a program that sums the first 3 elements of an array."
                        .to_string(),
                    starter_code: "        * Sum array elements\n        WAIT\nARRAY   DC   5,10,15\n"
                        .to_string(),
                    hints: vec![
                        "Initialize XR1 to 0 for array index".to_string(),
                        "Load each element with indexed addressing".to_string(),
                        "Use ADD to accumulate the sum".to_string(),
                    ],
                    solution: r#"        LDX  =0,1      * Index = 0
        LD   =0        * Sum = 0
        ADD  ARRAY,1   * Add ARRAY[0]
        MDX  1,=1      * Index++
        ADD  ARRAY,1   * Add ARRAY[1]
        MDX  1,=1      * Index++
        ADD  ARRAY,1   * Add ARRAY[2]
        WAIT

ARRAY   DC   5,10,15
"#
                    .to_string(),
                },
            ],
        ),
        TutorialWithContent::new(
            {
                let mut t = Tutorial::new(
                    "advanced-indexing",
                    "Advanced Addressing Techniques",
                    TutorialCategory::AdvancedTopics,
                    Difficulty::Advanced,
                );
                t.estimated_minutes = 35;
                t.learning_objectives = vec![
                    "Master indirect and indexed addressing combined".to_string(),
                    "Use multiple index registers efficiently".to_string(),
                    "Implement complex data structures".to_string(),
                ];
                t.prerequisites = vec!["index-registers".to_string(), "disk-operations".to_string()];
                t.available = true;
                t
            },
            vec![
                TutorialSection::Theory {
                    title: "Combined Addressing Modes".to_string(),
                    content: r#"Advanced addressing combines indexing and indirection:

Indirect Indexed:
    LD  /ARRAY,1   * Load from address at (ARRAY + XR1)

This enables:
- Pointers and references
- Dynamic data structures
- Table lookups
- Multi-dimensional arrays

Multiple Index Registers:
XR1, XR2, XR3 can be used simultaneously for complex patterns

Example: 2D array access
        LDX  =ROW,1    * Row index
        LDX  =COL,2    * Column index
        LD   TABLE,1   * Access element

Subroutine linkage uses BSI/BSC with index registers for
parameter passing and return addresses."#
                        .to_string(),
                },
                TutorialSection::HandsOn {
                    title: "Multi-Index Access".to_string(),
                    instructions: "Write a program using two index registers to access elements."
                        .to_string(),
                    starter_code: "        * Use XR1 and XR2\n        WAIT\n".to_string(),
                    hints: vec![
                        "Load XR1 and XR2 with different values".to_string(),
                        "Access memory using one index register".to_string(),
                    ],
                    solution: r#"        LDX  =10,1     * XR1 = 10
        LDX  =20,2     * XR2 = 20
        LD   DATA,1    * Load from DATA+10
        ADD  DATA,2    * Add from DATA+20
        WAIT

DATA    DC   0,1,2,3,4,5,6,7,8,9
        DC   10,11,12,13,14,15,16,17,18,19
        DC   20,21,22,23,24,25,26,27,28,29
"#
                    .to_string(),
                },
                TutorialSection::Quiz {
                    title: "Advanced Addressing Quiz".to_string(),
                    questions: vec![
                        QuizQuestion::MultipleChoice {
                            question: "What does /TABLE,1 mean?".to_string(),
                            options: vec![
                                "Load from TABLE indexed by XR1".to_string(),
                                "Load from address stored at (TABLE + XR1)".to_string(),
                                "Divide TABLE by XR1".to_string(),
                            ],
                            correct_index: 1,
                        },
                        QuizQuestion::TrueFalse {
                            question: "You can use all three index registers in one program".to_string(),
                            correct_answer: true,
                        },
                    ],
                },
            ],
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
