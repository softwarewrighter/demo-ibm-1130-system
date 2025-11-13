use crate::components::{
    code_editor::CodeEditor, control_panel::ControlPanel, emulator_view::EmulatorView,
};
use crate::services::bridge::MockEmulatorBridge;
use yew::prelude::*;

/// Code template for playground
#[derive(Clone, Debug, PartialEq)]
pub struct Template {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub code: &'static str,
}

/// Get available code templates
fn get_templates() -> &'static [Template] {
    &[
        Template {
            id: "blank",
            name: "Blank Program",
            description: "Start with an empty program",
            code: "        WAIT          * Halt program\n",
        },
        Template {
            id: "hello",
            name: "Hello World",
            description: "Print text to the line printer",
            code: r#"* Hello World Program
* Prints a message to the 1403 line printer

        LD   =42       * Load literal 42
        STO  100       * Store to memory
        WAIT           * Halt program

* TODO: Add actual HELLO WORLD printer output
"#,
        },
        Template {
            id: "loop",
            name: "Memory Copy Loop",
            description: "Basic loop with addressing",
            code: r#"* Memory Copy Loop
* Copy 10 words from SRC to DEST

        LD   =10       * Loop counter
        STO  COUNT
LOOP    LD   SRC,1     * Load from source (indexed)
        STO  DEST,1    * Store to destination (indexed)
        MDX  1,-1      * Decrement index
        BSC  NZ,LOOP   * Branch if not zero
        WAIT

COUNT   DC   0
SRC     DC   1,2,3,4,5,6,7,8,9,10
DEST    BSS  10        * Reserve 10 words
"#,
        },
    ]
}

#[function_component(Playground)]
pub fn playground() -> Html {
    // Program code state
    let code = use_state(|| get_templates()[0].code.to_string());

    // Emulator bridge state
    let emulator = use_state(MockEmulatorBridge::new);

    // Handle code changes
    let on_code_change = {
        let code = code.clone();
        Callback::from(move |new_code: String| {
            code.set(new_code);
        })
    };

    // Handle Load button
    let on_load = {
        let code = code.clone();
        let emulator = emulator.clone();
        Callback::from(move |_| {
            let mut emu = (*emulator).clone();
            emu.load((*code).clone());
            emulator.set(emu);
        })
    };

    // Handle Run button
    let on_run = {
        let emulator = emulator.clone();
        Callback::from(move |_| {
            let mut emu = (*emulator).clone();
            emu.run();
            emulator.set(emu);
        })
    };

    // Handle Step button
    let on_step = {
        let emulator = emulator.clone();
        Callback::from(move |_| {
            let mut emu = (*emulator).clone();
            emu.step();
            emulator.set(emu);
        })
    };

    // Handle Pause button
    let on_pause = {
        let emulator = emulator.clone();
        Callback::from(move |_| {
            let mut emu = (*emulator).clone();
            emu.pause();
            emulator.set(emu);
        })
    };

    // Handle Reset button
    let on_reset = {
        let emulator = emulator.clone();
        Callback::from(move |_| {
            let mut emu = (*emulator).clone();
            emu.reset();
            emulator.set(emu);
        })
    };

    // Handle template selection
    let on_template_select = {
        let code = code.clone();
        move |template_id: &'static str| {
            if let Some(template) = get_templates().iter().find(|t| t.id == template_id) {
                code.set(template.code.to_string());
            }
        }
    };

    html! {
        <div class="playground-container">
            <div class="playground-toolbar">
                <h2>{ "IBM 1130 Programming Playground" }</h2>
                <div class="template-buttons">
                    <label>{ "Templates: " }</label>
                    {
                        get_templates().iter().map(|template| {
                            let template_id = template.id;
                            let on_select = on_template_select.clone();
                            html! {
                                <button
                                    class="template-button"
                                    onclick={Callback::from(move |_| on_select(template_id))}
                                    title={template.description}
                                >
                                    { template.name }
                                </button>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>

            <div class="playground-split">
                <div class="playground-left">
                    <div class="editor-header">
                        <h3>{ "Code Editor" }</h3>
                    </div>
                    <CodeEditor
                        code={(*code).clone()}
                        on_change={on_code_change}
                        placeholder="Enter IBM 1130 assembly code here..."
                    />
                </div>

                <div class="playground-right">
                    <div class="control-header">
                        <h3>{ "Emulator" }</h3>
                    </div>
                    <ControlPanel
                        state={emulator.get_state()}
                        on_load={on_load}
                        on_run={on_run}
                        on_step={on_step}
                        on_pause={on_pause}
                        on_reset={on_reset}
                    />
                    <EmulatorView
                        registers={emulator.get_registers()}
                        output={emulator.get_output().to_string()}
                        console={emulator.get_console().to_string()}
                    />
                </div>
            </div>

            <div class="playground-footer">
                <p class="note">
                    { "This is a mock emulator for UI development. Full IBM 1130 emulation coming in Phase 6!" }
                </p>
            </div>
        </div>
    }
}
