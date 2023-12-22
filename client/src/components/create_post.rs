use gloo::console::{error, log};
use web_sys::SubmitEvent;
use yew::{classes, function_component, html, use_state, virtual_dom::VNode, AttrValue, Callback};

use crate::{
    api::create_post,
    components::{button::Button, text_area::TextArea},
    helpers::colors::Color,
};

#[function_component(CreatePost)]
pub fn component() -> VNode {
    let input_value_handle = use_state(AttrValue::default);
    let input_value = (*input_value_handle).clone();

    let on_input_change = {
        let input_value_handle = input_value_handle.clone();

        Callback::from(move |value: AttrValue| {
            input_value_handle.set(value);
        })
    };

    let on_submit = {
        Callback::from(move |e: SubmitEvent| {
            let input_value_handle = input_value_handle.clone();
            let input_value = (*input_value_handle).clone();

            e.prevent_default();

            wasm_bindgen_futures::spawn_local(async move {
                let post = match create_post(input_value).await {
                    Ok(post) => post,
                    Err(error) => {
                        error!("Error creating post", error.to_string());
                        return;
                    }
                };

                log!(format!("{post:?}"));
            });

            input_value_handle.set(AttrValue::default());
        })
    };

    html! {
        <form class={format!("flex flex-col gap-y-4")} onsubmit={on_submit}>
            <TextArea
                name="text"
                placeholder="Say something..."
                on_change={on_input_change}
                value={input_value}
                rows={5}
            />
            <Button
                variant={Color::Primary}
                classes={classes!("self-end")}
            >
                {"Submit post"}
            </Button>
        </form>
    }
}
