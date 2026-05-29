use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::services::{api, storage};

pub const TOKEN_KEY: &str = "noir_atelier_jwt_v1";
pub const EMAIL_KEY: &str = "noir_atelier_email_v1";

#[function_component(AuthPage)]
pub fn auth_page() -> Html {
    let email = use_state(String::new);
    let password = use_state(String::new);
    let status = use_state(|| {
        storage::get_string(EMAIL_KEY)
            .map(|email| format!("Вы вошли как {email}."))
            .unwrap_or_else(|| "Введите email и пароль для регистрации или входа.".to_string())
    });
    let navigator = use_navigator();

    let on_email = {
        let email = email.clone();
        Callback::from(move |event: InputEvent| {
            let input: web_sys::HtmlInputElement = event.target_unchecked_into();
            email.set(input.value());
        })
    };

    let on_password = {
        let password = password.clone();
        Callback::from(move |event: InputEvent| {
            let input: web_sys::HtmlInputElement = event.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_register = {
        let email = email.clone();
        let password = password.clone();
        let status = status.clone();
        Callback::from(move |_| {
            let email_value = (*email).clone();
            let password_value = (*password).clone();
            let status = status.clone();
            spawn_local(async move {
                match api::register(&email_value, &password_value).await {
                    Ok(response) => status.set(response.message),
                    Err(err) => status.set(err),
                }
            });
        })
    };

    let on_login = {
        let email = email.clone();
        let password = password.clone();
        let status = status.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            let email_value = (*email).clone();
            let password_value = (*password).clone();
            let status = status.clone();
            let navigator = navigator.clone();
            spawn_local(async move {
                match api::login(&email_value, &password_value).await {
                    Ok(response) => {
                        storage::set_string(TOKEN_KEY, &response.token);
                        storage::set_string(EMAIL_KEY, &response.email);
                        status.set(format!("Вход выполнен: {}.", response.email));
                        if let Some(navigator) = navigator {
                            navigator.push(&Route::MyOutfits);
                        }
                    }
                    Err(err) => status.set(err),
                }
            });
        })
    };

    let on_logout = {
        let status = status.clone();
        Callback::from(move |_| {
            storage::set_string(TOKEN_KEY, "");
            storage::set_string(EMAIL_KEY, "");
            status.set("Вы вышли из аккаунта.".to_string());
        })
    };

    html! {
        <>
            <section class="page-title">
                <p class="eyebrow">{ "Private account" }</p>
                <h1>{ "Регистрация и вход" }</h1>
                <p class="lead">{ "Войдите, чтобы сохранять свои образы, редактировать черновики и публиковать подборки в галерее." }</p>
            </section>

            <section class="about-layout">
                <article class="form-card">
                    <h2>{ "Аккаунт" }</h2>
                    <label for="auth-email">{ "Email" }</label>
                    <input id="auth-email" type="email" value={(*email).clone()} oninput={on_email} placeholder="student@example.com" />

                    <label for="auth-password">{ "Пароль" }</label>
                    <input id="auth-password" type="password" value={(*password).clone()} oninput={on_password} placeholder="Минимум 6 символов" />

                    <div class="hero-actions">
                        <button class="button" type="button" onclick={on_login}>{ "Войти" }</button>
                        <button class="button button-ghost" type="button" onclick={on_register}>{ "Зарегистрироваться" }</button>
                        <button class="button button-ghost" type="button" onclick={on_logout}>{ "Выйти" }</button>
                    </div>
                    <p class="small-status" aria-live="polite">{ (*status).clone() }</p>
                </article>

                <article class="info-panel">
                    <h2>{ "Зачем нужен аккаунт" }</h2>
                    <p>{ "Сохраняйте черновики образов и возвращайтесь к ним позже." }</p>
                    <p>{ "Публикуйте готовые подборки в общей галерее NOIR ATELIER." }</p>
                    <p>{ "Ваши личные записи доступны только после входа." }</p>
                </article>
            </section>
        </>
    }
}
