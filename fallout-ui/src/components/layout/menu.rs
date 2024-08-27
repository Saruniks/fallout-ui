use clone_on_capture::clone_on_capture;
use yew::prelude::*;
use yew_router::{hooks::use_navigator, navigator, Routable};

#[derive(Properties, PartialEq)]
pub struct MenuProps<T: Routable + 'static> {
    #[prop_or_default]
    pub class: Classes,
    pub items: Vec<MenuItem<T>>,
    pub default_selected: usize,
    pub mode: MenuMode,
}

#[derive(Clone, PartialEq)]
pub struct MenuItem<T: Routable + 'static> {
    pub label: &'static str,
    pub route: T,
}

#[derive(PartialEq)]
pub enum MenuMode {
    Inline,
    Horizontal,
}

#[clone_on_capture]
#[function_component]
pub fn Menu<T: Routable + 'static>(props: &MenuProps<T>) -> Html {
    let navigator = use_navigator().unwrap();

    let selected_item = use_state(|| props.default_selected);

    {
        let selected_item = selected_item.clone();
        use_effect_with(props.default_selected, move |new_default| {
            selected_item.set(*new_default);
        });
    }

    let items = props.items.iter().enumerate().map(|(index, item)| {
        let is_selected = *selected_item == index;
        let class_name = if is_selected {
            "text-white px-3 py-2 rounded-md bg-blue-700 cursor-pointer"
        } else {
            "text-white px-3 py-2 rounded-md hover:bg-blue-700 cursor-pointer"
        };

        let onclick = {
            let selected_item = selected_item.clone();
            let route = item.route.clone();
            Callback::from(move |_| {
                selected_item.set(index);
                navigator.push(&route.to_owned());
            })
        };

        html! {
            <li {onclick} class={class_name}>{ item.label }</li>
        }
    });

    let container_class = match props.mode {
        MenuMode::Inline => "flex-1 min-w-0 flex flex-col space-y-2", // Vertical (inline) layout
        MenuMode::Horizontal => "flex-1 min-w-0 flex space-x-4",      // Horizontal layout
    };

    html! {
        <ul class={classes!(container_class, props.class.clone())}>
            { for items }
        </ul>
    }
}
