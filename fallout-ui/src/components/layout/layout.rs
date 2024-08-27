use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub sider: Option<Html>,
    #[prop_or_default]
    pub no_min_height: bool,
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    if let Some(sider) = &props.sider {
        // Layout with Sider
        let class = if props.no_min_height {
            classes!("flex", props.class.clone())
        } else {
            classes!("flex", "min-h-screen", props.class.clone())
        };

        html! {
            <div {class}>
                <div class="w-64">
                    { sider.clone() }
                </div>
                <div class="flex-1 flex flex-col bg-gray-100">
                    { for props.children.iter() }
                </div>
            </div>
        }
    } else {
        // Layout without Sider
        let class = if props.no_min_height {
            classes!("bg-gray-100", props.class.clone())
        } else {
            classes!("bg-gray-100", "min-h-screen", props.class.clone())
        };

        html! {
            <div {class}>
                { for props.children.iter() }
            </div>
        }
    }
}
