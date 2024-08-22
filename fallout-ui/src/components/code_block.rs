use gloo::utils::window;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs, js_name = highlightElement)]
    fn highlight_element(element: &Element);
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Children,
    pub language: String,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(CodeBlock)]
pub fn code_block(props: &Props) -> Html {
    let id = format!("codeblock-{}", Uuid::new_v4());
    let code_class = format!("language-{}", &props.language);

    {
        let id = id.clone();
        use_effect_with(id, move |id| {
            if let Some(document) = window().document() {
                if let Some(element) = document.get_element_by_id(id) {
                    highlight_element(&element);
                }
            }
            || ()
        });
    }

    html! {
        <pre>
            <code id={id} class={classes!(code_class, props.class.clone())}>
                { for props.children.iter() }
            </code>
        </pre>
    }
}
