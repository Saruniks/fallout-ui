use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    pub image: Option<String>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub hoverable: Option<bool>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn Card(props: &Props) -> Html {
    let Props {
        class,
        image,
        children,
        hoverable,
        onclick,
    } = props.clone();

    let class = if Some(true) == hoverable {
        classes!("hover:shadow-xl", "cursor-pointer", class)
    } else {
        class
    };

    html! {
        <div {onclick} class={classes!("max-w-sm", "rounded", "overflow-hidden", "border", "border-slate-300", "bg-white", class)}>
            if let Some(image) = image {
                <img class="w-full bg-gray-200" src={image} alt={"image"} />
            }
            <div class="border-t border-slate-300 px-4 py-4">
                {children}
            </div>
        </div>
    }
}
