use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Nav)]
pub fn nav(props: &NavProps) -> Html {
    html! {
        <div class={classes!("text-center", "items-center", "px-4", "bg-gray-800", "text-white", "shadow-inner", props.class.clone())}>
            { props.children.clone() }
        </div>
    }
}
