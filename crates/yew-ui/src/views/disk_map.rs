use core_sim::disk::Geometry;
use gloo::file::File;
use gloo::file::callbacks::FileReader;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub struct DiskState {
    pub geometry: Geometry,
    pub data: Vec<u16>,
    pub loaded: bool,
}

impl Default for DiskState {
    fn default() -> Self {
        Self {
            geometry: Geometry::IBM2315,
            data: vec![0u16; Geometry::IBM2315.total_words()],
            loaded: false,
        }
    }
}

#[function_component(DiskMap)]
pub fn disk_map() -> Html {
    let disk_state = use_state(DiskState::default);
    let file_reader = use_state(|| None::<FileReader>);
    let error_msg = use_state(|| None::<String>);
    let hover_sector = use_state(|| None::<(u16, u8, u8)>); // (cyl, head, sector)

    let on_file_change = {
        let disk_state = disk_state.clone();
        let file_reader = file_reader.clone();
        let error_msg = error_msg.clone();

        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files()
                && let Some(file) = files.get(0)
            {
                let file = File::from(file);
                let file_name = file.name();

                if !file_name.ends_with(".dsk") {
                    error_msg.set(Some("Please select a .dsk file".to_string()));
                    return;
                }

                let disk_state = disk_state.clone();
                let error_msg = error_msg.clone();

                let reader = gloo::file::callbacks::read_as_bytes(&file, move |res| {
                    match res {
                        Ok(bytes) => {
                            // Parse the .dsk file
                            match parse_dsk_bytes(&bytes) {
                                Ok((geo, data)) => {
                                    disk_state.set(DiskState {
                                        geometry: geo,
                                        data,
                                        loaded: true,
                                    });
                                    error_msg.set(None);
                                }
                                Err(e) => {
                                    error_msg.set(Some(format!("Error loading file: {}", e)));
                                }
                            }
                        }
                        Err(e) => {
                            error_msg.set(Some(format!("Error reading file: {:?}", e)));
                        }
                    }
                });

                file_reader.set(Some(reader));
            }
        })
    };

    html! {
        <div class="disk-map">
            <h2>{ "Disk Map (2315 Cartridge)" }</h2>

            <div class="file-controls">
                <label for="disk-file-input" class="file-label">
                    { "Load .dsk file:" }
                </label>
                <input
                    type="file"
                    id="disk-file-input"
                    accept=".dsk"
                    onchange={on_file_change}
                />
            </div>

            if let Some(err) = (*error_msg).clone() {
                <div class="error-message">
                    { err }
                </div>
            }

            <div class="disk-info">
                <p><strong>{ "Cylinders:" }</strong> { disk_state.geometry.cylinders }</p>
                <p><strong>{ "Heads:" }</strong> { disk_state.geometry.heads }</p>
                <p><strong>{ "Sectors/Track:" }</strong> { disk_state.geometry.sectors_per_track }</p>
                <p><strong>{ "Words/Sector:" }</strong> { disk_state.geometry.words_per_sector }</p>
                <p><strong>{ "Total Words:" }</strong> { disk_state.geometry.total_words() }</p>
                <p><strong>{ "Loaded:" }</strong> { if disk_state.loaded { "Yes" } else { "No (blank disk)" } }</p>
            </div>

            <div class="cylinder-grid">
                { render_disk_grid(&disk_state, &hover_sector) }
            </div>

            if let Some((cyl, head, sector)) = *hover_sector {
                <div class="tooltip">
                    { format!("C:{} H:{} S:{}", cyl, head, sector) }
                </div>
            }
        </div>
    }
}

#[allow(dead_code)]
fn render_disk_grid(
    disk_state: &DiskState,
    hover_sector: &UseStateHandle<Option<(u16, u8, u8)>>,
) -> Html {
    let geo = &disk_state.geometry;

    // For visualization, we'll show a simplified view: every 10th cylinder
    let cylinder_step = if geo.cylinders > 100 { 10 } else { 1 };
    let cylinders_to_show: Vec<u16> = (0..geo.cylinders).step_by(cylinder_step as usize).collect();

    html! {
        <div class="grid-container">
            <div class="grid-header">
                <div class="header-label">{ "Cyl" }</div>
                { for (0..geo.heads).map(|head| {
                    html! {
                        <div class="header-label" key={head}>
                            { format!("H{}", head) }
                        </div>
                    }
                })}
            </div>
            { for cylinders_to_show.iter().map(|&cyl| {
                html! {
                    <div class="grid-row" key={cyl}>
                        <div class="row-label">{ cyl }</div>
                        { for (0..geo.heads).map(|head| {
                            render_head_sectors(disk_state, cyl, head, hover_sector)
                        })}
                    </div>
                }
            })}
        </div>
    }
}

#[allow(dead_code)]
fn render_head_sectors(
    disk_state: &DiskState,
    cyl: u16,
    head: u8,
    hover_sector: &UseStateHandle<Option<(u16, u8, u8)>>,
) -> Html {
    let geo = &disk_state.geometry;

    html! {
        <div class="head-cell" key={head}>
            { for (0..geo.sectors_per_track).map(|sector| {
                render_sector(disk_state, cyl, head, sector, hover_sector)
            })}
        </div>
    }
}

#[allow(dead_code)]
fn render_sector(
    disk_state: &DiskState,
    cyl: u16,
    head: u8,
    sector: u8,
    hover_sector: &UseStateHandle<Option<(u16, u8, u8)>>,
) -> Html {
    let sector_index = calculate_sector_index(&disk_state.geometry, cyl, head, sector);
    let word_offset = sector_index * disk_state.geometry.words_per_sector as usize;

    // Determine if sector has data (non-zero)
    let has_data = if word_offset + 321 <= disk_state.data.len() {
        disk_state.data[word_offset..word_offset + 321]
            .iter()
            .any(|&w| w != 0)
    } else {
        false
    };

    let class = if has_data {
        "sector used"
    } else {
        "sector free"
    };

    let hover_sector_enter = hover_sector.clone();
    let onmouseenter = Callback::from(move |_| {
        hover_sector_enter.set(Some((cyl, head, sector)));
    });

    let hover_sector_leave = hover_sector.clone();
    let onmouseleave = Callback::from(move |_| {
        hover_sector_leave.set(None);
    });

    html! {
        <div
            class={class}
            key={sector}
            {onmouseenter}
            {onmouseleave}
            title={format!("C:{} H:{} S:{}", cyl, head, sector)}
        />
    }
}

#[allow(dead_code)]
fn calculate_sector_index(geo: &Geometry, cyl: u16, head: u8, sector: u8) -> usize {
    let sectors_per_cyl = geo.heads as usize * geo.sectors_per_track as usize;
    cyl as usize * sectors_per_cyl
        + head as usize * geo.sectors_per_track as usize
        + sector as usize
}

#[allow(dead_code)]
fn parse_dsk_bytes(bytes: &[u8]) -> Result<(Geometry, Vec<u16>), String> {
    // Simple in-memory parser (we can't use file_io::read_dsk_file directly in WASM)
    if bytes.len() < 48 {
        return Err("File too short".to_string());
    }

    // Check magic
    if &bytes[0..8] != b"I1130DSK" {
        return Err("Invalid magic number".to_string());
    }

    // Parse geometry
    let cylinders = u16::from_le_bytes([bytes[8], bytes[9]]);
    let heads = bytes[10];
    let sectors_per_track = bytes[11];
    let words_per_sector = u16::from_le_bytes([bytes[12], bytes[13]]);

    if cylinders == 0 || heads == 0 || sectors_per_track == 0 || words_per_sector == 0 {
        return Err("Invalid geometry".to_string());
    }

    let geo = Geometry {
        cylinders,
        heads,
        sectors_per_track,
        words_per_sector,
    };

    // Parse data
    let data_bytes = &bytes[48..];
    let expected_bytes = geo.total_words() * 2;

    if data_bytes.len() != expected_bytes {
        return Err(format!(
            "Size mismatch: expected {} bytes, got {}",
            expected_bytes,
            data_bytes.len()
        ));
    }

    let data: Vec<u16> = data_bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    Ok((geo, data))
}
