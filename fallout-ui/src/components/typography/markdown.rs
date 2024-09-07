use gloo::utils::window;
use pulldown_cmark::{html::push_html, Options, Parser};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Element;
use yew::{prelude::*, virtual_dom::VNode};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = hljs, js_name = highlightElement)]
    fn highlight_element(element: &Element);
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(String::from("text-secondary"))]
    pub color_class: String,
    #[prop_or_default]
    pub class: Classes,
    pub children: Children,
}

#[function_component]
pub fn Markdown(props: &Props) -> Html {
    let Props {
        class,
        children,
        color_class,
    } = props.clone();

    let class = classes!("prose", color_class, class);

    let markdown_content: String = children
        .iter()
        .filter_map(|child| {
            // Attempt to get the text content of each child
            match child {
                VNode::VText(text) => Some(text.text.to_string()),
                _ => None, // Ignore non-text nodes
            }
        })
        .collect();

    use_effect_with((), move |_| {
        if let Some(document) = window().document() {
            let pre_elements = document.get_elements_by_tag_name("pre");
            for i in 0..pre_elements.length() {
                if let Some(pre_element) = pre_elements.item(i) {
                    let code_elements = pre_element.get_elements_by_tag_name("code");
                    if code_elements.length() > 0 {
                        if let Some(code_element) = code_elements.item(0) {
                            highlight_element(&code_element);
                        }
                    }
                }
            }
        }
        || {}
    });

    let parser = Parser::new(&markdown_content);
    let mut html_output = String::new();
    push_html(&mut html_output, parser);

    html! {
        <div {class}>
            { Html::from_html_unchecked(AttrValue::from(html_output)) }
        </div>
    }
}
