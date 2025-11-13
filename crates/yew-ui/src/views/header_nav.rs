use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Tab {
    Overview,
    Hardware,
    Demos,
    Learn,
    Challenges,
    Playground,
    Reference,
}

#[derive(Properties, PartialEq)]
pub struct HeaderNavProps {
    pub current_tab: Tab,
    pub on_tab_change: Callback<Tab>,
}

#[function_component(HeaderNav)]
pub fn header_nav(props: &HeaderNavProps) -> Html {
    let on_overview_click = {
        let on_tab_change = props.on_tab_change.clone();
        Callback::from(move |_| on_tab_change.emit(Tab::Overview))
    };

    let on_hardware_click = {
        let on_tab_change = props.on_tab_change.clone();
        Callback::from(move |_| on_tab_change.emit(Tab::Hardware))
    };

    let on_demos_click = {
        let on_tab_change = props.on_tab_change.clone();
        Callback::from(move |_| on_tab_change.emit(Tab::Demos))
    };

    let on_learn_click = {
        let on_tab_change = props.on_tab_change.clone();
        Callback::from(move |_| on_tab_change.emit(Tab::Learn))
    };

    let on_challenges_click = {
        let on_tab_change = props.on_tab_change.clone();
        Callback::from(move |_| on_tab_change.emit(Tab::Challenges))
    };

    let on_playground_click = {
        let on_tab_change = props.on_tab_change.clone();
        Callback::from(move |_| on_tab_change.emit(Tab::Playground))
    };

    let on_reference_click = {
        let on_tab_change = props.on_tab_change.clone();
        Callback::from(move |_| on_tab_change.emit(Tab::Reference))
    };

    html! {
        <header class="app-header">
            <div class="header-content">
                <h1 class="header-title">{ "IBM 1130 System Simulator" }</h1>
                <nav class="header-nav">
                    <button
                        class={classes!("nav-tab", (props.current_tab == Tab::Overview).then_some("active"))}
                        onclick={on_overview_click}
                    >
                        { "Overview" }
                    </button>
                    <button
                        class={classes!("nav-tab", (props.current_tab == Tab::Hardware).then_some("active"))}
                        onclick={on_hardware_click}
                    >
                        { "Hardware" }
                    </button>
                    <button
                        class={classes!("nav-tab", (props.current_tab == Tab::Demos).then_some("active"))}
                        onclick={on_demos_click}
                    >
                        { "Demos" }
                    </button>
                    <button
                        class={classes!("nav-tab", (props.current_tab == Tab::Learn).then_some("active"))}
                        onclick={on_learn_click}
                    >
                        { "Learn" }
                    </button>
                    <button
                        class={classes!("nav-tab", (props.current_tab == Tab::Challenges).then_some("active"))}
                        onclick={on_challenges_click}
                    >
                        { "Challenges" }
                    </button>
                    <button
                        class={classes!("nav-tab", (props.current_tab == Tab::Playground).then_some("active"))}
                        onclick={on_playground_click}
                    >
                        { "Playground" }
                    </button>
                    <button
                        class={classes!("nav-tab", (props.current_tab == Tab::Reference).then_some("active"))}
                        onclick={on_reference_click}
                    >
                        { "Reference" }
                    </button>
                </nav>
            </div>
        </header>
    }
}
