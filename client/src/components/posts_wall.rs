use yew::{function_component, html, virtual_dom::VNode};
use yewdux::use_store;

use crate::{
    components::{container::FlexContainer, post::Post},
    helpers::flex::FlexGap,
    store::AppStore,
};

#[function_component(PostsWall)]
pub fn component() -> VNode {
    let (store, _) = use_store::<AppStore>();

    html! {
        <FlexContainer gap={FlexGap::MD}>
            {
                store.posts.iter().map(|post| html! {
                    <Post post={post.clone()} />
                }).collect::<VNode>()
            }
        </FlexContainer>
    }
}
