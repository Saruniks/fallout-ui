use yew::prelude::*;

use crate::components::{
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

```rust
fn main() {
    println!(\"Hello, Yew!\");
}"###;

    html! {
        <Stories>
            <Story name={"Markdown"} background={StoryBackground::Light}>
                <Markdown>{markdown}</Markdown>
            </Story>
            <Story name={"Markdown Tailwind CSS Typography"} background={StoryBackground::Light}>
                <Markdown class="prose">{markdown}</Markdown>
            </Story>
        </Stories>
    }
}
