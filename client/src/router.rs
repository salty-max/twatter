use yew::{html, Html};
use yew_router::Routable;

use crate::routes::home::Home;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home />  },
    }
}
