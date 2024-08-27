use yew::prelude::*;
use yew_router::prelude::Link as YewLink;
use yew_router::Routable;

#[derive(Clone, PartialEq, Properties)]
pub struct Props<T: Routable + 'static> {
    #[prop_or_default]
    pub to: Option<T>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub is_selected: bool,
}

#[function_component]
pub fn Breadcrumb<T: Routable + 'static>(props: &Props<T>) -> Html {
    let Props {
        children,
        to,
        is_selected,
    } = props.clone();

    let class = classes!(
        "font-sans",
        "m-0",
        "text-sm",
        "capitalize",
        to.is_some()
            .then_some(classes!("hover:underline", "hover:text-secondary"))
    );

    let class = if is_selected {
        classes!("text-secondary", class)
    } else {
        classes!("text-washed-out-secondary", class)
    };

    html! {
        if let Some(to) = to {
            <YewLink<T> {to} classes={class}>
                {children}
            </YewLink<T>>
        } else {
            <span {class}>{children}</span>
        }
    }
}
