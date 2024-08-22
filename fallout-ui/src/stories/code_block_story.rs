use clone_on_capture::clone_on_capture;
use yew::prelude::*;

use crate::components::callouts::callout_info::CalloutInfo;
use crate::components::code_block::CodeBlock;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;

#[clone_on_capture]
#[function_component]
pub fn CodeBlockStory() -> Html {
    html! {
        <>
            <CalloutInfo class="mb-4">
                {"Highlight.js must be added to support CodeBlock"}
            </CalloutInfo>
            <Stories>
                <Story name={"Rust"} background={StoryBackground::Light}>
                    <CodeBlock language={"rust"}> {
    r###"#[tokio::main]
    async fn main() {
        // This is running on a core thread.

        let blocking_task = tokio::task::spawn_blocking(|| {
            // This is running on a blocking thread.
            // Blocking here is ok.
        });

        // We can wait for the blocking task like this:
        // If the blocking task panics, the unwrap below will propagate the
        // panic.
        blocking_task.await.unwrap();
    }"### }
                    </CodeBlock>
                </Story>
                <Story name={"TypeScript"} background={StoryBackground::Light}>
                    <CodeBlock language={"ts"}> {
r###"interface Point {
    x: number;
    y: number;
}

function logPoint(p: Point) {
    console.log(`${p.x}, ${p.y}`);
}

// logs "12, 26"
const point = { x: 12, y: 26 };
logPoint(point);"### }
                    </CodeBlock>
                </Story>
            </Stories>
        </>
    }
}
