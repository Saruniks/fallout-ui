use yew::prelude::*;

use crate::components::card::Card;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;
use crate::utils::toasts::notify_success;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {}

#[function_component]
pub fn CardStory(props: &Props) -> Html {
    let Props {} = props.clone();

    html! {
        <Stories>
            <Story name={"Default"} background={StoryBackground::Light}>
                <Card image="/images/yue-ma-uY4eLs8gfuo-unsplash.jpg">
                    {"Lorem ipsum dolor sit amet"}
                </Card>
            </Story>
            <Story name={"Hoverable & Clickable"} background={StoryBackground::Light}>
                <Card hoverable={true} onclick={|_| notify_success("Success")} image="/images/yue-ma-uY4eLs8gfuo-unsplash.jpg">
                    {"Lorem ipsum dolor sit amet"}
                </Card>
            </Story>
        </Stories>
    }
}
