use yew::{classes, function_component, html, Html};

use crate::{
    components::{
        create_post::CreatePost,
        title::{Title, TitleLevel},
    },
    helpers::colors::Color,
};

#[function_component]
pub fn Home() -> Html {
    html! {
        <>
            <Title
                variant={Color::Primary}
                level={TitleLevel::One}
                center={true} bold={true}
                classes={classes!("p-4")}>
                {"Twatter"}
            </Title>
            <CreatePost />
        </>
    }
}
