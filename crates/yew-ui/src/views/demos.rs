use crate::views::demo_viewer::DemoViewer;
use yew::prelude::*;

/// Demo categories for organizing demonstrations
#[derive(Clone, Copy, PartialEq)]
pub enum DemoCategory {
    DeviceDemos,
    MultiDevice,
    Languages,
    SystemSoftware,
}

impl DemoCategory {
    pub fn title(&self) -> &'static str {
        match self {
            DemoCategory::DeviceDemos => "Device Demonstrations",
            DemoCategory::MultiDevice => "Multi-Device Workflows",
            DemoCategory::Languages => "High-Level Languages",
            DemoCategory::SystemSoftware => "System Software",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            DemoCategory::DeviceDemos => {
                "Interactive demonstrations of individual I/O devices showing basic operations and timing characteristics."
            }
            DemoCategory::MultiDevice => {
                "Complex workflows showing coordination between multiple I/O devices in realistic programs."
            }
            DemoCategory::Languages => {
                "Programs written in APL, Forth, FORTRAN, and other high-level languages from the 1960s-70s."
            }
            DemoCategory::SystemSoftware => {
                "Core system utilities including assemblers, loaders, and the Disk Monitor System."
            }
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            DemoCategory::DeviceDemos,
            DemoCategory::MultiDevice,
            DemoCategory::Languages,
            DemoCategory::SystemSoftware,
        ]
    }
}

/// Demo metadata for display
#[derive(Clone, PartialEq)]
pub struct Demo {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub category: DemoCategory,
    pub tags: &'static [&'static str],
    pub available: bool,
}

impl Demo {
    /// Get sample demos for initial display
    pub fn get_samples() -> Vec<Self> {
        vec![
            Demo {
                id: "disk_seek_visualization",
                title: "Disk Seek Visualization",
                description: "Watch the disk arm move across cylinders with realistic timing. See rotational latency and quantized seeks in action.",
                category: DemoCategory::DeviceDemos,
                tags: &["IBM 2310", "Disk", "Timing"],
                available: false,
            },
            Demo {
                id: "card_reader_bootstrap",
                title: "Card Reader Bootstrap",
                description: "Load the initial program loader (IPL) from cards. See binary card format and cold start sequence.",
                category: DemoCategory::DeviceDemos,
                tags: &["IBM 1442", "Card Reader", "Boot"],
                available: false,
            },
            Demo {
                id: "apl_matrix_operations",
                title: "APL Matrix Operations",
                description: "Interactive APL\\1130 session showing matrix arithmetic, array operations, and the iconic APL notation.",
                category: DemoCategory::Languages,
                tags: &["APL", "Matrix", "Interactive"],
                available: true, // Now available with placeholder
            },
            Demo {
                id: "forth_hello_world",
                title: "Forth Hello World",
                description: "Charles Moore's original 1968 Forth implementation. See the birth of stack-based programming.",
                category: DemoCategory::Languages,
                tags: &["Forth", "Historical", "Interactive"],
                available: true, // Now available with placeholder
            },
            Demo {
                id: "batch_job_processing",
                title: "Batch Job Processing",
                description: "Complete workflow: Read card deck, compile FORTRAN, execute program, print results.",
                category: DemoCategory::MultiDevice,
                tags: &["FORTRAN", "Cards", "Printer", "Workflow"],
                available: false,
            },
            Demo {
                id: "dms_cold_start",
                title: "DMS Cold Start",
                description: "Boot the Disk Monitor System from scratch. Watch system initialization and device configuration.",
                category: DemoCategory::SystemSoftware,
                tags: &["DMS", "Boot", "System"],
                available: false,
            },
        ]
    }
}

#[derive(Properties, PartialEq)]
pub struct DemoCardProps {
    pub demo: Demo,
    pub on_launch: Callback<String>,
}

#[function_component(DemoCard)]
fn demo_card(props: &DemoCardProps) -> Html {
    let demo = &props.demo;
    let status_class = if demo.available {
        "demo-status-available"
    } else {
        "demo-status-planned"
    };
    let status_text = if demo.available {
        "Available"
    } else {
        "Planned"
    };

    let on_click = {
        let on_launch = props.on_launch.clone();
        let demo_id = demo.id.to_string();
        Callback::from(move |_| {
            on_launch.emit(demo_id.clone());
        })
    };

    html! {
        <div class="demo-card">
            <div class="demo-card-header">
                <h3>{ demo.title }</h3>
                <span class={classes!("demo-status", status_class)}>
                    { status_text }
                </span>
            </div>
            <p class="demo-description">{ demo.description }</p>
            <div class="demo-tags">
                { for demo.tags.iter().map(|tag| html! {
                    <span class="demo-tag">{ tag }</span>
                }) }
            </div>
            <div class="demo-actions">
                if demo.available {
                    <button class="demo-launch-button" onclick={on_click}>{ "Launch Demo" }</button>
                } else {
                    <button class="demo-launch-button" disabled=true>{ "Coming Soon" }</button>
                }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CategorySectionProps {
    pub category: DemoCategory,
    pub demos: Vec<Demo>,
    pub on_launch: Callback<String>,
}

#[function_component(CategorySection)]
fn category_section(props: &CategorySectionProps) -> Html {
    let category = props.category;
    let demos = &props.demos;
    let on_launch = props.on_launch.clone();

    html! {
        <section class="demo-category-section">
            <div class="demo-category-header">
                <h2>{ category.title() }</h2>
                <p class="demo-category-description">{ category.description() }</p>
            </div>
            <div class="demo-grid">
                { for demos.iter().map(|demo| {
                    let on_launch = on_launch.clone();
                    html! {
                        <DemoCard demo={demo.clone()} on_launch={on_launch} />
                    }
                }) }
            </div>
        </section>
    }
}

/// Demos tab component showing interactive demonstrations
#[function_component(Demos)]
pub fn demos() -> Html {
    let all_demos = Demo::get_samples();
    let selected_demo = use_state(|| None::<String>);

    let on_launch = {
        let selected_demo = selected_demo.clone();
        Callback::from(move |demo_id: String| {
            selected_demo.set(Some(demo_id));
        })
    };

    let on_back = {
        let selected_demo = selected_demo.clone();
        Callback::from(move |_| {
            selected_demo.set(None);
        })
    };

    html! {
        <div class="demos-content">
            if let Some(demo_id) = (*selected_demo).as_ref() {
                // Show demo viewer
                <div class="demo-viewer-container">
                    <div class="demo-viewer-header">
                        <button class="back-to-demos-button" onclick={on_back}>
                            { "← Back to Demos" }
                        </button>
                    </div>
                    <DemoViewer demo_id={Some(demo_id.clone())} />
                </div>
            } else {
                // Show demo browser
                <>
                    <div class="demos-intro">
                        <h1>{ "Interactive Demonstrations" }</h1>
                        <p class="demos-intro-text">
                            { "Explore the IBM 1130 through interactive demos showing individual devices, \
                               multi-device workflows, and programs written in historical languages like \
                               APL and Forth. Each demo runs authentic 1960s software on our cycle-accurate simulator." }
                        </p>
                    </div>

                    { for DemoCategory::all().iter().map(|category| {
                        let category_demos: Vec<Demo> = all_demos.iter()
                            .filter(|d| d.category == *category)
                            .cloned()
                            .collect();
                        let on_launch = on_launch.clone();
                        html! {
                            <CategorySection category={*category} demos={category_demos} on_launch={on_launch} />
                        }
                    }) }

                    <div class="demos-footer">
                        <p>
                            { "Demo viewer now integrated! Click \"Launch Demo\" on available demos to see them in action. \
                               Placeholder data is shown until the emulator backend is connected." }
                        </p>
                    </div>
                </>
            }
        </div>
    }
}
