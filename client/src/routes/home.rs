use gloo::console::error;
use yew::{classes, function_component, html, platform::spawn_local, use_effect, Html};
use yewdux::use_store;

use crate::{
    api::get_all_posts,
    components::{
        container::FlexContainer,
        create_post::CreatePost,
        posts_wall::PostsWall,
        title::{Title, TitleLevel},
    },
    helpers::{colors::Color, flex::FlexGap},
    store::AppStore,
};

#[function_component]
pub fn Home() -> Html {
    let (_, dispatch) = use_store::<AppStore>();

    use_effect(move || {
        // Get the posts from the store
        spawn_local(async move {
            match get_all_posts().await {
                Ok(posts_data) => {
                    dispatch.set(AppStore { posts: posts_data });
                }
                Err(error) => {
                    error!("Error getting top posts: {}", error.to_string());
                }
            }
        });

        || {}
    });

    html! {
        <>
            <FlexContainer classes={classes!("p-4")}  gap={FlexGap::LG}>
                <CreatePost />
                <PostsWall />
            </FlexContainer>
        </>
    }
}
