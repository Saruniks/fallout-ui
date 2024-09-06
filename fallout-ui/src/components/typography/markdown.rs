use pulldown_cmark::{html::push_html, Parser};
use yew::{prelude::*, virtual_dom::VNode};

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

    let class = classes!(color_class, class);

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

    // Convert Markdown to HTML using pulldown-cmark
    let parser = Parser::new(&markdown_content);
    let mut html_output = String::new();
    push_html(&mut html_output, parser);

    html! {
        <div {class}>
            { Html::from_html_unchecked(AttrValue::from(html_output)) }
        </div>
    }
}
