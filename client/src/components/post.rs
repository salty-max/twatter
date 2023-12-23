use yew::{classes, function_component, html, virtual_dom::VNode, Properties};
use yew_icons::{Icon, IconId};

use crate::{
    components::container::{Container, FlexContainer},
    helpers::{colors::Color, flex::FlexGap},
    types::DtoPost,
};

#[derive(Properties, PartialEq, Clone)]
pub struct PostProps {
    pub post: DtoPost,
}

#[function_component(Post)]
pub fn component(props: &PostProps) -> VNode {
    html! {
        <Container
            bg={Color::Light}
            classes={classes!([
                "rounded-md",
                "flex",
                "flex-col",
                "overflow-hidden"
            ])}
        >
            <Container
                classes={classes!([
                    "p-4"
                ])}
            >
                <p>{props.post.text.clone()}</p>
            </Container>
            <Container
                classes={classes!([
                    "flex",
                    "justify-end",
                    "gap-x-4",
                    "border-t",
                    "border-border",
                    "py-2",
                    "px-4"
                ])}
            >
                <FlexContainer row={true} gap={FlexGap::SM}>
                    <Icon class={classes!("text-danger")} icon_id={IconId::LucideHeart} width={"1rem"} />
                    <span>{props.post.likes}</span>
                </FlexContainer>
                <FlexContainer row={true} gap={FlexGap::SM}>
                    <Icon icon_id={IconId::LucideReply} width={"1rem"} />
                    <span>{props.post.reply_count}</span>
                </FlexContainer>
            </Container>
        </Container>
    }
}
