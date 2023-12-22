use wasm_bindgen::JsCast;
use web_sys::{HtmlTextAreaElement, InputEvent};
use yew::{
    classes, function_component, html, virtual_dom::VNode, AttrValue, Callback, Classes, Properties,
};

#[derive(Properties, PartialEq)]
pub struct TextAreaProps {
    #[prop_or_default]
    pub id: AttrValue,
    pub name: AttrValue,
    #[prop_or_default]
    pub label: AttrValue,
    #[prop_or_default]
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub value: AttrValue,
    #[prop_or_default]
    pub on_change: Callback<AttrValue>,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or(3)]
    pub rows: u8,
}

#[function_component(TextArea)]
pub fn component(props: &TextAreaProps) -> VNode {
    let classes = classes!(
        ["rounded-md", "p-4", "border", "border-border"],
        props.classes.clone()
    );

    let oninput = {
        let on_change = props.on_change.clone();

        Callback::from(move |e: InputEvent| {
            let target = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());

            if let Some(target) = target {
                on_change.emit(target.value().into());
            }
        })
    };

    html! {
        <textarea
            class={classes}
            id={props.id.clone()}
            name={props.name.clone()}
            placeholder={props.placeholder.clone()}
            value={props.value.clone()}
            rows={props.rows.to_string()}
            {oninput}
        />
    }
}
