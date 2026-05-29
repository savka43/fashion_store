use yew::prelude::*;

fn look_card(extra_class: &'static str, img: &'static str, alt: &'static str, tag: &'static str, title: &'static str, text: &'static str) -> Html {
    html! {
        <article class={classes!("look-card", extra_class)}>
            <img src={img} alt={alt} />
            <div>
                <p class="tag">{ tag }</p>
                <h2>{ title }</h2>
                <p>{ text }</p>
            </div>
        </article>
    }
}

#[function_component(LookbookPage)]
pub fn lookbook_page() -> Html {
    html! {
        <>
            <section class="page-title">
                <p class="eyebrow">{ "Lookbook" }</p>
                <h1>{ "Готовые образы" }</h1>
                <p class="lead">{ "Вдохновение высокой модой: резкие силуэты, монохром, свободная посадка и аксессуары." }</p>
            </section>
            <section class="lookbook-grid">
                { look_card(
                    "tall",
                    "https://images.unsplash.com/photo-1469334031218-e382a71b716b?auto=format&fit=crop&w=900&q=80",
                    "Модель в уличном образе",
                    "Look 01",
                    "Monochrome attitude",
                    "Чёрный образ с объёмным верхом и чистой линией низа.",
                ) }
                { look_card(
                    "",
                    "https://images.unsplash.com/photo-1495385794356-15371f348c31?auto=format&fit=crop&w=900&q=80",
                    "Модный образ",
                    "Look 02",
                    "Soft tailoring",
                    "Пиджак, широкие брюки и расслабленная посадка.",
                ) }
                { look_card(
                    "",
                    "https://images.unsplash.com/photo-1524504388940-b1c1722653e1?auto=format&fit=crop&w=900&q=80",
                    "Fashion портрет",
                    "Look 03",
                    "Night editorial",
                    "Лаконичный вечерний образ без лишнего декора.",
                ) }
            </section>
        </>
    }
}
