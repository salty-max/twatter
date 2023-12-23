use yew::{classes, function_component, html, virtual_dom::VNode, Properties};

use crate::{
    components::title::{Title, TitleLevel},
    helpers::colors::Color,
};

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: VNode,
}

#[function_component(Layout)]
pub fn component(props: &LayoutProps) -> VNode {
    html! {
        <>
            <header>
                <Title
                    variant={Color::Primary}
                    level={TitleLevel::One}
                    center={true} bold={true}
                    classes={classes!("py-8")}>
                    {"Twatter"}
                </Title>
            </header>
            <main class={format!("w-8/12 mx-auto bg-gray-400 rounded-md bg-clip-padding backdrop-filter backdrop-blur-md bg-opacity-10 border border-gray-100")}>

                   {props.children.clone()}
            </main>
        </>
    }
}
