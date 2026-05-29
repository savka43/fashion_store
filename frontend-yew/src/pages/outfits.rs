use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::models::{Outfit, OutfitPayload};
use crate::pages::auth::{EMAIL_KEY, TOKEN_KEY};
use crate::services::{api, storage};

fn outfit_card(outfit: &Outfit, actions: Html) -> Html {
    let status = if outfit.is_published { "Опубликовано" } else { "Черновик" };
    html! {
        <article class="info-panel">
            <p class="tag">{ format!("{} · {}", outfit.occasion, status) }</p>
            <h2>{ &outfit.title }</h2>
            <p>{ &outfit.content }</p>
            <p class="small-status">{ format!("Автор: {} · {}", outfit.author_email, outfit.created_at) }</p>
            { actions }
        </article>
    }
}

#[function_component(FeedPage)]
pub fn feed_page() -> Html {
    let outfits = use_state(Vec::<Outfit>::new);
    let status = use_state(|| "Загрузка опубликованных образов...".to_string());

    {
        let outfits = outfits.clone();
        let status = status.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match api::get_public_outfits().await {
                    Ok(list) => {
                        let count = list.len();
                        outfits.set(list);
                        status.set(format!("Опубликованных записей: {count}."));
                    }
                    Err(err) => status.set(err),
                }
            });
            || ()
        });
    }

    html! {
        <>
            <section class="page-title">
                <p class="eyebrow">{ "Community looks" }</p>
                <h1>{ "Галерея образов" }</h1>
                <p class="lead">{ "Опубликованные подборки клиентов и редакции: поводы, силуэты и детали для вдохновения." }</p>
            </section>

            <section class="section">
                <p class="small-status">{ (*status).clone() }</p>
                <div class="cards two-columns">
                    { for outfits.iter().map(|outfit| outfit_card(outfit, html! {})) }
                </div>
            </section>
        </>
    }
}

#[function_component(MyOutfitsPage)]
pub fn my_outfits_page() -> Html {
    let token = use_state(|| storage::get_string(TOKEN_KEY).unwrap_or_default());
    let email = storage::get_string(EMAIL_KEY).unwrap_or_default();
    let outfits = use_state(Vec::<Outfit>::new);
    let status = use_state(|| "Загружаем ваши образы...".to_string());
    let payload = use_state(OutfitPayload::default);
    let editing_id = use_state(|| None::<i64>);
    let navigator = use_navigator();

    let reload = {
        let token = token.clone();
        let outfits = outfits.clone();
        let status = status.clone();
        Callback::from(move |_| {
            let token_value = (*token).clone();
            let outfits = outfits.clone();
            let status = status.clone();
            if token_value.trim().is_empty() {
                status.set("Сначала выполните вход.".to_string());
                return;
            }
            spawn_local(async move {
                match api::get_my_outfits(&token_value).await {
                    Ok(list) => {
                        let count = list.len();
                        outfits.set(list);
                        status.set(format!("Ваших записей: {count}."));
                    }
                    Err(err) => status.set(err),
                }
            });
        })
    };

    {
        let reload = reload.clone();
        use_effect_with((), move |_| {
            reload.emit(());
            || ()
        });
    }

    let set_field = |field: &'static str, payload: UseStateHandle<OutfitPayload>| {
        Callback::from(move |event: InputEvent| {
            let mut next = (*payload).clone();
            if field == "content" {
                let input: web_sys::HtmlTextAreaElement = event.target_unchecked_into();
                next.content = input.value();
            } else {
                let input: web_sys::HtmlInputElement = event.target_unchecked_into();
                match field {
                    "title" => next.title = input.value(),
                    "occasion" => next.occasion = input.value(),
                    _ => {}
                }
            }
            payload.set(next);
        })
    };

    let on_save = {
        let token = token.clone();
        let payload = payload.clone();
        let editing_id = editing_id.clone();
        let status = status.clone();
        let reload = reload.clone();
        Callback::from(move |_| {
            let token_value = (*token).clone();
            let payload_value = (*payload).clone();
            let edit_value = *editing_id;
            let status = status.clone();
            let payload = payload.clone();
            let editing_id = editing_id.clone();
            let reload = reload.clone();
            spawn_local(async move {
                let result = if let Some(id) = edit_value {
                    api::update_outfit(&token_value, id, &payload_value).await.map(|_| "Запись обновлена.".to_string())
                } else {
                    api::create_outfit(&token_value, &payload_value).await.map(|_| "Черновик создан.".to_string())
                };
                match result {
                    Ok(message) => {
                        payload.set(OutfitPayload::default());
                        editing_id.set(None);
                        status.set(message);
                        reload.emit(());
                    }
                    Err(err) => status.set(err),
                }
            });
        })
    };

    if token.trim().is_empty() {
        return html! {
            <>
                <section class="page-title">
                    <p class="eyebrow">{ "Private wardrobe" }</p>
                    <h1>{ "Мои образы" }</h1>
                    <p class="lead">{ "Войдите, чтобы работать с личными подборками." }</p>
                </section>
                <section class="banner">
                    <h2>{ "Сначала выполните вход" }</h2>
                    <p>{ "После входа здесь появятся ваши черновики и опубликованные образы." }</p>
                    <button class="button" type="button" onclick={Callback::from(move |_| {
                        if let Some(navigator) = navigator.clone() {
                            navigator.push(&Route::Auth);
                        }
                    })}>{ "Перейти ко входу" }</button>
                </section>
            </>
        };
    }

    html! {
        <>
            <section class="page-title">
                <p class="eyebrow">{ "Private wardrobe" }</p>
                <h1>{ "Мои образы" }</h1>
                <p class="lead">{ format!("Личный раздел пользователя {email}. Черновики не попадают в публичную ленту до публикации.") }</p>
            </section>

            <section class="about-layout">
                <article class="form-card">
                    <h2>{ if editing_id.is_some() { "Редактировать образ" } else { "Новый образ" } }</h2>
                    <label for="outfit-title">{ "Название" }</label>
                    <input id="outfit-title" value={payload.title.clone()} oninput={set_field("title", payload.clone())} placeholder="Например: Monochrome office" />

                    <label for="outfit-occasion">{ "Повод" }</label>
                    <input id="outfit-occasion" value={payload.occasion.clone()} oninput={set_field("occasion", payload.clone())} placeholder="Работа, вечер, прогулка" />

                    <label for="outfit-content">{ "Описание" }</label>
                    <textarea id="outfit-content" rows="6" value={payload.content.clone()} oninput={set_field("content", payload.clone())} placeholder="Опишите вещи, силуэт и настроение образа." />

                    <div class="hero-actions">
                        <button class="button" type="button" onclick={on_save}>{ if editing_id.is_some() { "Сохранить изменения" } else { "Создать черновик" } }</button>
                        <button class="button button-ghost" type="button" onclick={{
                            let payload = payload.clone();
                            let editing_id = editing_id.clone();
                            Callback::from(move |_| {
                                payload.set(OutfitPayload::default());
                                editing_id.set(None);
                            })
                        }}>{ "Очистить" }</button>
                    </div>
                    <p class="small-status">{ (*status).clone() }</p>
                </article>

                <div class="cards">
                    { for outfits.iter().map(|outfit| {
                        let id = outfit.id;
                        let token_value = (*token).clone();
                        let publish_reload = reload.clone();
                        let publish_status = status.clone();
                        let publish_label = if outfit.is_published { "Снять" } else { "Опубликовать" };
                        let on_publish = Callback::from(move |_| {
                            let token_value = token_value.clone();
                            let reload = publish_reload.clone();
                            let status = publish_status.clone();
                            spawn_local(async move {
                                let result = if publish_label == "Снять" {
                                    api::unpublish_outfit(&token_value, id).await.map(|_| "Запись снята с публикации.".to_string())
                                } else {
                                    api::publish_outfit(&token_value, id).await.map(|_| "Запись опубликована.".to_string())
                                };
                                match result {
                                    Ok(message) => {
                                        status.set(message);
                                        reload.emit(());
                                    }
                                    Err(err) => status.set(err),
                                }
                            });
                        });

                        let token_value = (*token).clone();
                        let reload = reload.clone();
                        let status = status.clone();
                        let on_delete = Callback::from(move |_| {
                            let token_value = token_value.clone();
                            let reload = reload.clone();
                            let status = status.clone();
                            spawn_local(async move {
                                match api::delete_outfit(&token_value, id).await {
                                    Ok(_) => {
                                        status.set("Запись удалена.".to_string());
                                        reload.emit(());
                                    }
                                    Err(err) => status.set(err),
                                }
                            });
                        });

                        let edit_payload = OutfitPayload {
                            title: outfit.title.clone(),
                            content: outfit.content.clone(),
                            occasion: outfit.occasion.clone(),
                        };
                        let payload = payload.clone();
                        let editing_id = editing_id.clone();
                        let on_edit = Callback::from(move |_| {
                            payload.set(edit_payload.clone());
                            editing_id.set(Some(id));
                        });

                        outfit_card(outfit, html! {
                            <div class="hero-actions">
                                <button class="button" type="button" onclick={on_publish}>{ publish_label }</button>
                                <button class="button button-ghost" type="button" onclick={on_edit}>{ "Редактировать" }</button>
                                <button class="button button-ghost" type="button" onclick={on_delete}>{ "Удалить" }</button>
                            </div>
                        })
                    }) }
                </div>
            </section>
        </>
    }
}
