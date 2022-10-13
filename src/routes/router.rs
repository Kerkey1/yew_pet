use yew::prelude::*;
use yew_router::prelude::*;
use crate::component::MainPage;
#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Main,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Main => html! {<MainPage />},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}