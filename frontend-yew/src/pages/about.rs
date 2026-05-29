use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::services::storage;

const DRAFT_KEY: &str = "noir_atelier_draft";

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
struct Draft {
    title: String,
    body: String,
}

#[function_component(AboutPage)]
pub fn about_page() -> Html {
    let draft = use_state(|| storage::get_json::<Draft>(DRAFT_KEY).unwrap_or_default());
    let status = use_state(|| {
        if storage::get_json::<Draft>(DRAFT_KEY).is_some() {
            "Загружен сохранённый черновик.".to_string()
        } else {
            "Черновик пока не сохранён.".to_string()
        }
    });

    let on_title = {
        let draft = draft.clone();
        Callback::from(move |event: InputEvent| {
            let input: web_sys::HtmlInputElement = event.target_unchecked_into();
            draft.set(Draft { title: input.value(), body: draft.body.clone() });
        })
    };

    let on_body = {
        let draft = draft.clone();
        Callback::from(move |event: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = event.target_unchecked_into();
            draft.set(Draft { title: draft.title.clone(), body: input.value() });
        })
    };

    let onsubmit = {
        let draft = draft.clone();
        let status = status.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            storage::set_json(DRAFT_KEY, &*draft);
            status.set("Черновик сохранён локально.".to_string());
        })
    };

    html! {
        <>
            <section class="page-title">
                <p class="eyebrow">{ "About" }</p>
                <h1>{ "О магазине" }</h1>
                <p class="lead">
                    { "NOIR ATELIER — интернет-магазин одежды с подборками, лукбуком и персональными образами." }
                </p>
            </section>

            <section class="about-layout">
                <article class="info-panel">
                    <h2>{ "Идея проекта" }</h2>
                    <p>{ "Магазин вдохновлён эстетикой высокой моды: тёмная палитра, крупные заголовки, простые формы и акцент на изображениях товаров." }</p>
                    <p>{ "Сервис помогает быстро собрать комплект, сохранить пожелания и поделиться готовым образом в общей галерее." }</p>
                </article>

                <article class="form-card">
                    <h2>{ "Черновик заявки" }</h2>
                    <p>{ "Форма сохраняет текст на этом устройстве, чтобы к заявке можно было вернуться позже." }</p>
                    <form novalidate=true onsubmit={onsubmit}>
                        <label for="draft-title">{ "Название образа / товара" }</label>
                        <input
                            id="draft-title"
                            name="title"
                            type="text"
                            maxlength="120"
                            autocomplete="off"
                            placeholder="Например: чёрное пальто"
                            value={draft.title.clone()}
                            oninput={on_title}
                        />

                        <label for="draft-body">{ "Комментарий к заказу" }</label>
                        <textarea
                            id="draft-body"
                            name="body"
                            rows="5"
                            maxlength="1000"
                            placeholder="Размер, цвет, пожелания"
                            value={draft.body.clone()}
                            oninput={on_body}
                        />

                        <button class="button" type="submit">{ "Сохранить черновик" }</button>
                        <p class="small-status" aria-live="polite">{ (*status).clone() }</p>
                    </form>
                </article>
            </section>
        </>
    }
}
