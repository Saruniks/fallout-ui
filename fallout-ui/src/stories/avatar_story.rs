use yew::prelude::*;

use crate::components::avatar::Avatar;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {}

#[function_component]
pub fn AvatarStory(props: &Props) -> Html {
    let Props {} = props.clone();

    html! {
        <Stories>
            <Story name={"Default"} background={StoryBackground::Light}>
                <div class="p-4">
                    <Avatar text={"E".to_string()} />
                </div>
            </Story>
        </Stories>
    }
}
