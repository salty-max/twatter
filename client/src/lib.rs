use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Switch};

use crate::{
    components::layout::Layout,
    router::{switch, Route},
};

mod api;
mod components;
mod helpers;
mod router;
mod routes;

#[function_component]
pub fn App() -> Html {
    html! {
        <Layout>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </Layout>
    }
}
