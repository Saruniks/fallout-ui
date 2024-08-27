use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FooterProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let FooterProps { children, class } = props;

    html! {
        <div class={classes!("text-center", "p-4", "bg-gray-800", "text-white", "shadow-inner", class)}>
            { children }
        </div>
    }
}
