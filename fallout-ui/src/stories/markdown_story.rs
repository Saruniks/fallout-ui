use yew::prelude::*;

use crate::components::{
    callouts::callout_info::CalloutInfo,
    stories::Stories,
    story::{Story, StoryBackground},
    typography::markdown::Markdown,
};

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {}

#[function_component]
pub fn MarkdownStory(props: &Props) -> Html {
    let Props {} = props.clone();

    let markdown = r###"Hello, Yew!

This is a **Markdown** example rendered in a Yew app.

- Item 1
- Item 2
- Item 3

## Subheading

This is a paragraph with *italic* and **bold** text.

`inline code`

```rust 
fn main() {
    println!(\"Hello, Yew!\");
}"###;

    html! {
        <>
            <CalloutInfo class="mb-4">
                {"Markdown Component is intended to be used with Highlight.js and Tailwind CSS Typography"}
            </CalloutInfo>
            <Stories>
                <Story name={"Markdown Tailwind CSS Typography"} background={StoryBackground::Light}>
                    <Markdown>{markdown}</Markdown>
                </Story>
            </Stories>
        </>
    }
}
