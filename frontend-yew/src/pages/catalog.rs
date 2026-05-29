use yew::prelude::*;

use crate::models::{format_price, Product, PRODUCTS};
use crate::services::storage;

const FAV_KEY: &str = "noir_atelier_favorites";

#[function_component(CatalogPage)]
pub fn catalog_page() -> Html {
    let query = use_state(String::new);
    let favorites = use_state(|| storage::get_json::<Vec<String>>(FAV_KEY).unwrap_or_default());

    let on_search = {
        let query = query.clone();
        Callback::from(move |event: InputEvent| {
            let input: web_sys::HtmlInputElement = event.target_unchecked_into();
            query.set(input.value());
        })
    };

    let normalized_query = query.to_lowercase();
    let filtered = PRODUCTS
        .iter()
        .filter(|product| product.search.to_lowercase().contains(&normalized_query))
        .collect::<Vec<_>>();

    html! {
        <>
            <section class="page-title">
                <p class="eyebrow">{ "Catalog" }</p>
                <h1>{ "Каталог одежды" }</h1>
                <p class="lead">{ "Фильтруйте товары по названию. Избранное сохраняется локально в браузере." }</p>
            </section>

            <section class="toolbar" aria-label="Поиск по каталогу">
                <label for="product-search">{ "Поиск товара" }</label>
                <input
                    id="product-search"
                    type="search"
                    placeholder="Например: пальто, худи, брюки"
                    value={(*query).clone()}
                    oninput={on_search}
                />
                <p id="catalog-status" class="small-status" aria-live="polite">
                    { format!("Показано товаров: {}. В избранном: {}.", filtered.len(), favorites.len()) }
                </p>
            </section>

            <section class="product-list" id="product-list">
                { for filtered.iter().map(|product| product_card(product, favorites.clone())) }
            </section>
        </>
    }
}

fn product_card(product: &Product, favorites: UseStateHandle<Vec<String>>) -> Html {
    let is_active = favorites.iter().any(|title| title == product.title);
    let button_text = if is_active { "В избранном" } else { "В избранное" };
    let product_title = product.title.to_string();
    let onclick = Callback::from(move |_| {
        let mut next = (*favorites).clone();
        if let Some(index) = next.iter().position(|title| title == &product_title) {
            next.remove(index);
        } else {
            next.push(product_title.clone());
        }
        storage::set_json(FAV_KEY, &next);
        favorites.set(next);
    });

    html! {
        <article class="product-card">
            <img src={product.image} alt={product.alt} />
            <div class="product-info">
                <p class="tag">{ product.tag }</p>
                <h2>{ product.title }</h2>
                <p>{ product.text }</p>
                <div class="product-bottom">
                    <span>{ format_price(product.price) }</span>
                    <button
                        type="button"
                        class={classes!("fav-btn", is_active.then_some("is-active"))}
                        onclick={onclick}
                    >
                        { button_text }
                    </button>
                </div>
            </div>
        </article>
    }
}
