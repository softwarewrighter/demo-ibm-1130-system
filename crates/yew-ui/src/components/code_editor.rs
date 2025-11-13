use yew::prelude::*;

/// Properties for the CodeEditor component
#[derive(Properties, PartialEq)]
pub struct CodeEditorProps {
    /// Current code content
    pub code: String,
    /// Callback when code changes
    pub on_change: Callback<String>,
    /// Whether the editor is read-only
    #[prop_or(false)]
    pub read_only: bool,
    /// Placeholder text when empty
    #[prop_or_default]
    pub placeholder: Option<String>,
}

/// Code editor component with basic editing capabilities
///
/// This component provides a simple text area for editing IBM 1130 assembly code.
/// Future enhancements will include syntax highlighting and line numbers.
#[function_component(CodeEditor)]
pub fn code_editor(props: &CodeEditorProps) -> Html {
    let oninput = {
        let on_change = props.on_change.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
                on_change.emit(input.value());
            }
        })
    };

    html! {
        <div class="code-editor-container">
            <textarea
                class="code-editor"
                value={props.code.clone()}
                {oninput}
                readonly={props.read_only}
                placeholder={props.placeholder.clone().unwrap_or_default()}
                spellcheck="false"
                autocomplete="off"
            />
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_editor_props_creation() {
        let callback = Callback::from(|_: String| {});
        let props = yew::props!(CodeEditorProps {
            code: "LD 100".to_string(),
            on_change: callback,
        });

        assert_eq!(props.code, "LD 100");
        assert!(!props.read_only);
        assert!(props.placeholder.is_none());
    }

    #[test]
    fn test_code_editor_props_with_placeholder() {
        let callback = Callback::from(|_: String| {});
        let props = yew::props!(CodeEditorProps {
            code: String::new(),
            on_change: callback,
            placeholder: Some("Enter code here...".to_string()),
        });

        assert_eq!(props.code, "");
        assert_eq!(props.placeholder, Some("Enter code here...".to_string()));
    }

    #[test]
    fn test_code_editor_props_read_only() {
        let callback = Callback::from(|_: String| {});
        let props = yew::props!(CodeEditorProps {
            code: "WAIT".to_string(),
            on_change: callback,
            read_only: true,
        });

        assert!(props.read_only);
    }
}
