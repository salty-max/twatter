use yew::{classes, function_component, html, virtual_dom::VNode, Properties};

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: VNode,
}

#[function_component(Layout)]
pub fn component(props: &LayoutProps) -> VNode {
    html! {
        <main class={format!("w-screen h-screen bg-dark")}>
            <div class={classes!("w-8/12", "mx-auto")}>
               {props.children.clone()}
            </div>
        </main>
    }
}
