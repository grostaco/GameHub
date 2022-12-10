mod app;
pub mod components;
pub mod routes;
mod services;

pub use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::Renderer::<App>::new().render();
}
