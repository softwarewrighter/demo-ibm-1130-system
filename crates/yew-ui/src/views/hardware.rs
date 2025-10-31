use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum HardwareDevice {
    Cpu1131,
    Printer1132,
    Multiplexor1133,
    Printer1403,
    CardReader1442,
    Plotter1627,
    Display2250,
}

impl HardwareDevice {
    pub fn name(&self) -> &'static str {
        match self {
            HardwareDevice::Cpu1131 => "1131 CPU",
            HardwareDevice::Printer1132 => "1132 Line Printer",
            HardwareDevice::Multiplexor1133 => "1133 Multiplexor",
            HardwareDevice::Printer1403 => "1403 Line Printer",
            HardwareDevice::CardReader1442 => "1442 Card Reader/Punch",
            HardwareDevice::Plotter1627 => "1627 Plotter",
            HardwareDevice::Display2250 => "2250 Vector Graphics Display",
        }
    }

    pub fn full_name(&self) -> &'static str {
        match self {
            HardwareDevice::Cpu1131 => "IBM 1131 Central Processing Unit",
            HardwareDevice::Printer1132 => "IBM 1132 Line Printer",
            HardwareDevice::Multiplexor1133 => "IBM 1133 Multiplexor",
            HardwareDevice::Printer1403 => "IBM 1403 Line Printer",
            HardwareDevice::CardReader1442 => "IBM 1442 Card Reader/Punch",
            HardwareDevice::Plotter1627 => "IBM 1627 Plotter (CalComp 565)",
            HardwareDevice::Display2250 => "IBM 2250 Vector Graphics Display",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            HardwareDevice::Cpu1131 => concat!(
                "The IBM 1131 Central Processing Unit was the heart of the IBM 1130 system. ",
                "It featured a 16-bit architecture with a 2-microsecond cycle time and could ",
                "address up to 32KB of magnetic core memory. The CPU included an arithmetic-logic ",
                "unit, index registers, and hardware multiply/divide capabilities. The control ",
                "console featured lights and switches for direct machine-code programming and debugging."
            ),
            HardwareDevice::Printer1132 => concat!(
                "The IBM 1132 was the standard line printer for the IBM 1130 system, capable of ",
                "printing 80 characters per line at speeds up to 80 lines per minute. It used ",
                "a chain printer mechanism and could print the full ASCII character set. The 1132 ",
                "was commonly used for program listings, reports, and general-purpose output in ",
                "small to medium-sized installations."
            ),
            HardwareDevice::Multiplexor1133 => concat!(
                "The IBM 1133 Multiplexor was a key component that allowed multiple I/O devices ",
                "to be connected to the IBM 1130 system. It managed device interrupts, data transfers, ",
                "and I/O command routing. The multiplexor enabled the 1130 to handle concurrent ",
                "operations across different peripherals, improving system throughput and enabling ",
                "real-time data acquisition applications."
            ),
            HardwareDevice::Printer1403 => concat!(
                "The IBM 1403 was a high-speed chain printer capable of printing up to 1,100 lines ",
                "per minute. Originally designed for larger IBM systems, it could be attached to ",
                "the IBM 1130 for installations requiring higher printing speeds. The 1403 used ",
                "a rotating chain of characters and hammers to achieve its impressive speed, making ",
                "it ideal for production environments with heavy printing requirements."
            ),
            HardwareDevice::CardReader1442 => concat!(
                "The IBM 1442 Card Read Punch was the primary input/output device for the IBM 1130 ",
                "system. It could read punched cards at speeds up to 400 cards per minute and punch ",
                "cards at up to 160 cards per minute. The 1442 supported both 80-column IBM cards ",
                "and various punch formats. It was essential for batch processing, program loading, ",
                "and data entry in the era before widespread use of terminals."
            ),
            HardwareDevice::Plotter1627 => concat!(
                "The IBM 1627 was a drum plotter based on the CalComp 565 mechanism. It could ",
                "produce precise engineering drawings and scientific plots on continuous paper or ",
                "individual sheets. The plotter featured a moving pen carriage and rotating drum, ",
                "allowing it to create complex vector graphics. This made the IBM 1130 popular in ",
                "engineering and scientific applications requiring graphical output."
            ),
            HardwareDevice::Display2250 => concat!(
                "The IBM 2250 was a sophisticated vector graphics display terminal featuring a ",
                "cathode ray tube (CRT), light pen, keyboard, and programmable function keys. ",
                "It could display high-resolution vector graphics and was used for interactive ",
                "computer-aided design (CAD), data visualization, and real-time system monitoring. ",
                "The 2250 represented cutting-edge human-computer interaction technology in the ",
                "1960s and early 1970s."
            ),
        }
    }

    pub fn image_path(&self) -> Option<&'static str> {
        match self {
            HardwareDevice::Cpu1131 => None, // We don't have a 1131 CPU image yet
            HardwareDevice::Printer1132 => Some("/demo-ibm-1130-system/static/licensed-media/ibm-1132-printer.jpg"),
            HardwareDevice::Multiplexor1133 => None, // We don't have a 1133 image yet
            HardwareDevice::Printer1403 => Some("/demo-ibm-1130-system/static/licensed-media/ibm-1403-printer.jpg"),
            HardwareDevice::CardReader1442 => Some("/demo-ibm-1130-system/static/licensed-media/ibm-1442-card-reader.jpg"),
            HardwareDevice::Plotter1627 => Some("/demo-ibm-1130-system/static/licensed-media/calcomp-565-plotter.jpg"),
            HardwareDevice::Display2250 => Some("/demo-ibm-1130-system/static/licensed-media/ibm-2250-display.png"),
        }
    }

    pub fn attribution(&self) -> Option<(&'static str, &'static str, &'static str)> {
        match self {
            HardwareDevice::Cpu1131 => None,
            HardwareDevice::Printer1132 => Some((
                "ThisIsNotABetter",
                "CC BY-SA 2.0",
                "https://creativecommons.org/licenses/by-sa/2.0/"
            )),
            HardwareDevice::Multiplexor1133 => None,
            HardwareDevice::Printer1403 => Some((
                "inra.dist (Jean Weber)",
                "CC BY 2.0",
                "https://creativecommons.org/licenses/by/2.0/"
            )),
            HardwareDevice::CardReader1442 => Some((
                "Unknown",
                "CC BY-SA 2.0",
                "https://creativecommons.org/licenses/by-sa/2.0/"
            )),
            HardwareDevice::Plotter1627 => Some((
                "Ryan Somma",
                "CC BY-SA 2.0",
                "https://creativecommons.org/licenses/by-sa/2.0/"
            )),
            HardwareDevice::Display2250 => Some((
                "Grlloyd",
                "CC BY-SA 4.0",
                "https://creativecommons.org/licenses/by-sa/4.0/"
            )),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct HardwareProps {
    pub selected_device: HardwareDevice,
    pub on_device_change: Callback<HardwareDevice>,
}

#[function_component(Hardware)]
pub fn hardware(props: &HardwareProps) -> Html {
    let devices = vec![
        HardwareDevice::Cpu1131,
        HardwareDevice::Printer1132,
        HardwareDevice::Multiplexor1133,
        HardwareDevice::Printer1403,
        HardwareDevice::CardReader1442,
        HardwareDevice::Plotter1627,
        HardwareDevice::Display2250,
    ];

    html! {
        <div class="hardware-container">
            <aside class="hardware-sidebar">
                <h3>{ "Hardware Components" }</h3>
                <nav class="hardware-nav">
                    {
                        devices.iter().map(|device| {
                            let device_copy = *device;
                            let on_click = {
                                let on_device_change = props.on_device_change.clone();
                                Callback::from(move |_| on_device_change.emit(device_copy))
                            };
                            html! {
                                <button
                                    class={classes!(
                                        "sidebar-item",
                                        (props.selected_device == device_copy).then_some("active")
                                    )}
                                    onclick={on_click}
                                >
                                    { device.name() }
                                </button>
                            }
                        }).collect::<Html>()
                    }
                </nav>
            </aside>
            <div class="hardware-content">
                <h2>{ props.selected_device.full_name() }</h2>
                {
                    if let Some(image_path) = props.selected_device.image_path() {
                        html! {
                            <div class="hardware-image-container">
                                <img
                                    src={image_path}
                                    alt={props.selected_device.full_name()}
                                    class="hardware-image"
                                />
                                {
                                    if let Some((author, license, license_url)) = props.selected_device.attribution() {
                                        html! {
                                            <p class="image-attribution">
                                                { "Photo by " }
                                                { author }
                                                { " / " }
                                                <a href={license_url} target="_blank" rel="noopener noreferrer">
                                                    { license }
                                                </a>
                                            </p>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                            </div>
                        }
                    } else {
                        html! {
                            <div class="no-image-placeholder">
                                <p>{ "Image not yet available for this device." }</p>
                            </div>
                        }
                    }
                }
                <div class="hardware-description">
                    <h3>{ "Description" }</h3>
                    <p>{ props.selected_device.description() }</p>
                </div>
            </div>
        </div>
    }
}
