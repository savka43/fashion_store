mod app;
mod calculator;
mod components;
mod models;
mod pages;
mod services;

fn main() {
    yew::Renderer::<app::App>::with_root(
        web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.get_element_by_id("root"))
            .expect("root element must exist"),
    )
    .render();
}
