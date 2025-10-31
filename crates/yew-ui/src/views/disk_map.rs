use yew::prelude::*;

#[function_component(DiskMap)]
pub fn disk_map() -> Html {
    html! {
        <div class="disk-map">
            <h2>{ "Disk Map (2315 Cartridge)" }</h2>
            <div class="cylinder-grid">
                <p>{ "TODO: Cylinder × Sector grid visualization" }</p>
                <p>{ "200 cylinders × 2 heads × 4 sectors/track" }</p>
            </div>
        </div>
    }
}
