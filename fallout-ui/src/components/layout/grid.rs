use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Columns {
    One,
    Two,
    Three,
    Four,
}

impl Columns {
    pub fn to_class(&self) -> &'static str {
        match self {
            Columns::One => "grid-cols-1",
            Columns::Two => "grid-cols-2",
            Columns::Three => "grid-cols-3",
            Columns::Four => "grid-cols-4",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct GridProps {
    #[prop_or_default]
    pub children: Children,
    pub columns: Columns,
}

#[function_component]
pub fn Grid(props: &GridProps) -> Html {
    let columns_class = props.columns.to_class();

    html! {
        <div class={classes!("grid", columns_class, "gap-4")}>
            { for props.children.iter() }
        </div>
    }
}
