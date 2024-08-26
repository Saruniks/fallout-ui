use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MenuProps {
    #[prop_or_default]
    pub class: Classes,
    pub items: Vec<MenuItem>,
    pub default_selected: usize,
    pub mode: MenuMode,
}

#[derive(Clone, PartialEq)]
pub struct MenuItem {
    pub label: &'static str,
    pub route: Route,
}

#[derive(PartialEq)]
pub enum MenuMode {
    Inline,
    Horizontal,
}

#[function_component]
pub fn Menu(props: &MenuProps) -> Html {
    let selected_item = use_state(|| props.default_selected);

    let items = props.items.iter().enumerate().map(|(index, item)| {
        let is_selected = *selected_item == index;
        let class_name = if is_selected {
            "text-white px-3 py-2 rounded-md bg-blue-700 cursor-pointer"
        } else {
            "text-white px-3 py-2 rounded-md hover:bg-blue-700 cursor-pointer"
        };

        let onclick = {
            let selected_item = selected_item.clone();
            Callback::from(move |_| selected_item.set(index))
        };

        html! {
            <li {onclick} class={class_name}>{ item }</li>
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
