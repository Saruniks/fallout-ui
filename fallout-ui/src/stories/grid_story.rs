use yew::prelude::*;

use crate::components::card::Card;
use crate::components::layout::grid::Columns;
use crate::components::layout::grid::Grid;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {}

#[function_component]
pub fn GridStory(props: &Props) -> Html {
    let Props {} = props.clone();

    let card = html! {
        <Card class={"max-w-xs"} image="/images/yue-ma-uY4eLs8gfuo-unsplash.jpg">
            {"Lorem ipsum dolor sit amet"}
        </Card>
    };

    html! {
        <Stories>
            <Story name={"Columns::Two"} background={StoryBackground::Light}>
                <Grid columns={Columns::Two}>
                    {card.clone()}
                    {card.clone()}
                    {card.clone()}
                </Grid>
            </Story>
            <Story name={"Columns::Three"} background={StoryBackground::Light}>
                <Grid columns={Columns::Three}>
                    {card.clone()}
                    {card.clone()}
                    {card.clone()}
                    {card.clone()}
                    {card}
                </Grid>
            </Story>
        </Stories>
    }
}
