use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

fn collection_card(img: &'static str, alt: &'static str, title: &'static str, text: &'static str, price: &'static str) -> Html {
    html! {
        <article class="card">
            <img src={img} alt={alt} />
            <div class="card-body">
                <h3>{ title }</h3>
                <p>{ text }</p>
                <span class="price">{ price }</span>
            </div>
        </article>
    }
}

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <>
            <section class="hero hero-home">
                <div class="hero-text">
                    <p class="eyebrow">{ "Premium streetwear / high fashion mood" }</p>
                    <h1>{ "Одежда с характером большого города" }</h1>
                    <p class="lead">
                        { "Минималистичный магазин одежды в эстетике подиума: чёрный, металл, свободные силуэты и акцентные детали." }
                    </p>
                    <div class="hero-actions">
                        <Link<Route> classes="button" to={Route::Catalog}>{ "Смотреть каталог" }</Link<Route>>
                        <Link<Route> classes="button button-ghost" to={Route::Lookbook}>{ "Открыть лукбук" }</Link<Route>>
                    </div>
                </div>
                <div class="hero-photo">
                    <img
                        src="https://images.unsplash.com/photo-1503342217505-b0a15ec3261c?auto=format&fit=crop&w=1100&q=80"
                        alt="Модель в модной одежде"
                    />
                </div>
            </section>

            <section class="section">
                <div class="section-head">
                    <p class="eyebrow">{ "Selected drop" }</p>
                    <h2>{ "Новая коллекция" }</h2>
                </div>
                <div class="cards three-columns">
                    { collection_card(
                        "https://images.unsplash.com/photo-1529139574466-a303027c1d8b?auto=format&fit=crop&w=900&q=80",
                        "Пальто oversize",
                        "Oversize пальто",
                        "Плотная ткань, прямой крой, подчёркнутая линия плеч.",
                        "24 900 ₽",
                    ) }
                    { collection_card(
                        "https://images.unsplash.com/photo-1496747611176-843222e1e57c?auto=format&fit=crop&w=900&q=80",
                        "Чёрный образ",
                        "Black total look",
                        "База для вечернего выхода и повседневного city-style.",
                        "18 700 ₽",
                    ) }
                    { collection_card(
                        "https://images.unsplash.com/photo-1483985988355-763728e1935b?auto=format&fit=crop&w=900&q=80",
                        "Модная сумка",
                        "Аксессуары",
                        "Сумки, очки и ремни как главный акцент образа.",
                        "от 5 200 ₽",
                    ) }
                </div>
            </section>

            <section class="banner">
                <p class="eyebrow">{ "Editorial service" }</p>
                <h2>{ "Не просто одежда, а настроение витрины высокой моды." }</h2>
                <p>{ "Сохраняйте избранное, собирайте комплекты и публикуйте собственные образы в галерее бренда." }</p>
            </section>
        </>
    }
}
