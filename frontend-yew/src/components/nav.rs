use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav class="nav" aria-label="Основная навигация">
            <Link<Route> to={Route::Home}>{ "Главная" }</Link<Route>>
            <Link<Route> to={Route::Catalog}>{ "Каталог" }</Link<Route>>
            <Link<Route> to={Route::Lookbook}>{ "Лукбук" }</Link<Route>>
            <Link<Route> to={Route::Calculator}>{ "Подбор образа" }</Link<Route>>
            <Link<Route> to={Route::Outfits}>{ "Галерея" }</Link<Route>>
            <Link<Route> to={Route::MyOutfits}>{ "Мои образы" }</Link<Route>>
            <Link<Route> to={Route::Auth}>{ "Вход" }</Link<Route>>
            <Link<Route> to={Route::About}>{ "О бренде" }</Link<Route>>
        </nav>
    }
}
