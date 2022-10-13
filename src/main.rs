mod app;
mod routes;
mod component;

// const STYLE_FILE: &str = include_str!("styles/styles.css");

fn main() {
    yew::start_app::<app::App>();
}