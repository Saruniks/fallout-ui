use html::ChildrenProps;
use yew::prelude::*;

#[function_component(Sider)]
pub fn sider(props: &ChildrenProps) -> Html {
    html! {
        <div class="bg-gray-800 w-64 py-2 px-4 h-full">
            { props.children.clone() }
        </div>
    }
}
