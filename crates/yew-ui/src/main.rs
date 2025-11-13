mod app;
mod models;
mod services;
mod views;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
