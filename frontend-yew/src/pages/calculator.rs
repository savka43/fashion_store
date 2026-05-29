use yew::prelude::*;

use crate::calculator::{calculate, OutfitCalcState};
use crate::models::{format_price, PRODUCTS};
use crate::services::storage;

const CALC_KEY: &str = "noir_atelier_wasm_calc";

#[function_component(CalculatorPage)]
pub fn calculator_page() -> Html {
    let state = use_state(|| storage::get_json::<OutfitCalcState>(CALC_KEY).unwrap_or_default());
    let result = calculate(&state);

    {
        let state = state.clone();
        use_effect_with((*state).clone(), move |value| {
            storage::set_json(CALC_KEY, value);
            || ()
        });
    }

    let update_number = |field: &'static str, state: UseStateHandle<OutfitCalcState>| {
        Callback::from(move |event: InputEvent| {
            let input: web_sys::HtmlInputElement = event.target_unchecked_into();
            let value = input.value().parse::<i64>().unwrap_or(0);
            let mut next = (*state).clone();
            match field {
                "budget" => next.budget = value,
                "discount" => next.discount = value,
                "months" => next.months = value,
                "chest" => next.chest_cm = value,
                _ => {}
            }
            state.set(next);
        })
    };

    html! {
        <>
            <section class="page-title">
                <p class="eyebrow">{ "Personal styling" }</p>
                <h1>{ "Калькулятор образа" }</h1>
                <p class="lead">
                    { "Соберите комплект, проверьте итоговую стоимость, ежемесячный платёж и рекомендованный размер." }
                </p>
            </section>

            <section class="calc-layout">
                <form class="form-card">
                    <h2>{ "Параметры" }</h2>

                    <label for="calc-budget">{ "Бюджет, ₽" }</label>
                    <input id="calc-budget" type="number" min="1000" step="500" value={state.budget.to_string()} oninput={update_number("budget", state.clone())} />

                    <label for="calc-discount">{ "Скидка, %" }</label>
                    <input id="calc-discount" type="number" min="0" max="70" step="1" value={state.discount.to_string()} oninput={update_number("discount", state.clone())} />

                    <label for="calc-months">{ "Рассрочка, месяцев" }</label>
                    <input id="calc-months" type="number" min="1" max="24" step="1" value={state.months.to_string()} oninput={update_number("months", state.clone())} />

                    <label for="calc-chest">{ "Обхват груди, см" }</label>
                    <input id="calc-chest" type="number" min="70" max="130" step="1" value={state.chest_cm.to_string()} oninput={update_number("chest", state.clone())} />

                    <fieldset class="checkbox-grid">
                        <legend>{ "Состав образа" }</legend>
                        { for PRODUCTS.iter().take(4).map(|product| {
                            let id = product.id.to_string();
                            let checked = state.selected.iter().any(|item| item == product.id);
                            let state = state.clone();
                            html! {
                                <label>
                                    <input
                                        type="checkbox"
                                        checked={checked}
                                        onchange={Callback::from(move |_| {
                                            let mut next = (*state).clone();
                                            if next.selected.iter().any(|item| item == &id) {
                                                next.selected.retain(|item| item != &id);
                                            } else {
                                                next.selected.push(id.clone());
                                            }
                                            state.set(next);
                                        })}
                                    />
                                    { format!("{} · {}", product.title, format_price(product.price)) }
                                </label>
                            }
                        }) }
                    </fieldset>

                    <p class="small-status" aria-live="polite">{ "Параметры сохраняются в этом браузере." }</p>
                </form>

                <article class="info-panel calc-result">
                    <h2>{ "Итог" }</h2>
                    <dl class="result-list">
                        <div><dt>{ "Сумма до скидки" }</dt><dd>{ format_price(result.subtotal) }</dd></div>
                        <div><dt>{ "После скидки" }</dt><dd>{ format_price(result.total) }</dd></div>
                        <div><dt>{ "Платёж в месяц" }</dt><dd>{ format_price(result.monthly) }</dd></div>
                        <div><dt>{ "Баланс бюджета" }</dt><dd>{ format_price(result.balance) }</dd></div>
                        <div><dt>{ "Рекомендованный размер" }</dt><dd>{ result.size }</dd></div>
                    </dl>
                    <p class="small-status">{ "Итог обновляется сразу после изменения параметров." }</p>
                </article>
            </section>
        </>
    }
}
