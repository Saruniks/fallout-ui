use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ContentProps {
    #[prop_or_default]
    pub class: Classes,
    pub children: Children,
}

#[function_component(Content)]
pub fn content(props: &ContentProps) -> Html {
    html! {
        <div class={props.class.clone()}>
            { for props.children.iter() }
        </div>
    }
}
