use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct AvatarProps {
    pub text: String,
}

#[function_component(Avatar)]
pub fn avatar(props: &AvatarProps) -> Html {
    html! {
        <div class="w-10 h-10 rounded-full flex items-center justify-center bg-gray-200 text-gray-700 font-bold">
            {props.text.chars().next().unwrap_or_default().to_string()}
        </div>
    }
}
