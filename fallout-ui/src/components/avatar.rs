use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct AvatarProps {
    pub text: String,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(Avatar)]
pub fn avatar(props: &AvatarProps) -> Html {
    let onclick = props.onclick.clone().unwrap_or_default();

    html! {
        <div class="relative inline-block">
            <div
                class={classes!(
                    "w-10", "h-10", "rounded-full", "flex", "items-center", "justify-center", "bg-gray-200", "text-gray-700", "font-bold", "cursor-pointer", "hover:shadow-lg", "transition-shadow", "duration-200",
                    props.classes.clone() // Add additional classes
                )}
                {onclick}
            >
                {props.text.chars().next().unwrap_or_default().to_string()}
            </div>
        </div>
    }
}
