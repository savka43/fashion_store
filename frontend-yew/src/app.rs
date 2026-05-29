use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::layout::Layout;
use crate::pages::{
    about::AboutPage, auth::AuthPage, calculator::CalculatorPage, catalog::CatalogPage,
    home::HomePage, lookbook::LookbookPage, outfits::{FeedPage, MyOutfitsPage},
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/catalog")]
    Catalog,
    #[at("/lookbook")]
    Lookbook,
    #[at("/calculator")]
    Calculator,
    #[at("/outfits")]
    Outfits,
    #[at("/outfits/mine")]
    MyOutfits,
    #[at("/auth")]
    Auth,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::Catalog => html! { <CatalogPage /> },
        Route::Lookbook => html! { <LookbookPage /> },
        Route::Calculator => html! { <CalculatorPage /> },
        Route::Outfits => html! { <FeedPage /> },
        Route::MyOutfits => html! { <MyOutfitsPage /> },
        Route::Auth => html! { <AuthPage /> },
        Route::About => html! { <AboutPage /> },
        Route::NotFound => html! { <HomePage /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Layout>
                <Switch<Route> render={switch} />
            </Layout>
        </BrowserRouter>
    }
}
