use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::components::nav::Nav;
use crate::services::storage;

const THEME_KEY: &str = "noir_atelier_theme";

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

fn apply_theme(theme: &str) {
    if let Some(root) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.document_element())
    {
        let _ = root.set_attribute("data-theme", theme);
    }
    storage::set_string(THEME_KEY, theme);
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let theme = use_state(|| {
        storage::get_string(THEME_KEY)
            .filter(|value| value == "dark" || value == "light")
            .unwrap_or_else(|| "dark".to_string())
    });

    {
        let theme = theme.clone();
        use_effect_with((), move |_| {
            apply_theme(&theme);
            || ()
        });
    }

    let toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_| {
            let next = if theme.as_str() == "dark" { "light" } else { "dark" };
            apply_theme(next);
            theme.set(next.to_string());
        })
    };

    html! {
        <>
            <header class="site-header">
                <div class="header-inner">
                    <Link<Route> classes="brand" to={Route::Home}>{ "NOIR ATELIER" }</Link<Route>>
                    <Nav />
                    <button class="theme-toggle" type="button" onclick={toggle_theme}>{ "Тема" }</button>
                </div>
            </header>
            <main>{ for props.children.iter() }</main>
            <footer class="site-footer">
                <span>{ "NOIR ATELIER · curated fashion store" }</span>
                <span>{ "Персональные подборки и авторские образы" }</span>
                <span>{ "ИДБ-23-08 Березкин Савелий Андреевич" }</span>
            </footer>
        </>
    }
}
