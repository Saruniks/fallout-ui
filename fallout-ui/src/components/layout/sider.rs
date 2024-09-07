use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SiderProps {
    #[prop_or_default]
    pub class: Classes,
    pub children: Children,
}

#[function_component(Sider)]
pub fn sider(props: &SiderProps) -> Html {
    html! {
        <div class={classes!("bg-gray-800", "w-64", "py-2", "px-4", "h-full", props.class.clone())}>
            { props.children.clone() }
        </div>
    }
}
