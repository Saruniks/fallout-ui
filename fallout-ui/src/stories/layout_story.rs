use crate::components::layout::content::Content;
use crate::components::layout::footer::Footer;
use crate::components::layout::layout::Layout;
// use crate::components::layout::layout_context_provider::LayoutContextProvider;
use crate::components::breadcrumbs_divider::BreadcrumbsDivider;
use crate::components::layout::menu::Menu;
use crate::components::layout::menu::MenuItem;
use crate::components::layout::menu::MenuMode;
use crate::components::layout::nav::Nav;
use crate::components::layout::sider::Sider;
use crate::components::link::breadcrumb::Breadcrumb;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;

use yew::prelude::*;
use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum DummyRoute {
    #[at("/layout")]
    Nav1,

    #[at("/layout")]
    Nav2,

    #[allow(unreachable_patterns)]
    #[at("/layout")]
    Nav3,
}

#[function_component]
pub fn LayoutStory() -> Html {
    let items = vec![
        MenuItem {
            label: "nav-1",
            route: DummyRoute::Nav1,
        },
        MenuItem {
            label: "nav-2",
            route: DummyRoute::Nav2,
        },
        MenuItem {
            label: "nav-3",
            route: DummyRoute::Nav3,
        },
    ];

    let sider = Some(html! {
        <Sider>
            <div class="demo-logo" />
            <div class={classes!("h-fit")}>
                <Menu<DummyRoute> mode={MenuMode::Inline} items={items.clone()} default_selected={0}/>
            </div>
        </Sider>
    });

    let breadcrumbs = html! {
        <div class="space-x-2">
            <Breadcrumb<DummyRoute> to={Some(DummyRoute::Nav1)}>
                { "Home" }
            </Breadcrumb<DummyRoute>>
            <BreadcrumbsDivider/>
            <Breadcrumb<DummyRoute> to={Some(DummyRoute::Nav2)}>
                { "Blog" }
            </Breadcrumb<DummyRoute>>
            <BreadcrumbsDivider/>
            <Breadcrumb<DummyRoute> is_selected={true} to={Some(DummyRoute::Nav3)}>
                { "Layout" }
            </Breadcrumb<DummyRoute>>
        </div>
    };

    html! {
        <Stories>
            <Story name={"Nav-Content-Footer"} background={StoryBackground::Light}>
                <Layout no_min_height={true} class={"border"}>
                    <Nav class="flex space-x-4">
                        <div class="demo-logo"/>
                        <Menu<DummyRoute> mode={MenuMode::Horizontal} {items} default_selected={0}/>
                    </Nav>
                        <div class="flex-1 pb-10 pt-6 px-12 space-y-6">
                            { breadcrumbs.clone() }
                            <div class={"bg-white p-6 rounded-lg shadow-md"}>
                            <Content>
                                { "Lorem ipsum dolor sit amet" }
                            </Content>
                            </div>
                        </div>
                    <Footer>
                        { "Lorem ipsum" }
                    </Footer>
                </Layout>
            </Story>

            <Story name={"Sider-Content-EmptyFooter-EmptyNav"} background={StoryBackground::Light}>
                <Layout no_min_height={true} {sider} class={"border"}>
                    <Nav class={"bg-white p-4"}/>
                    <div class="flex-1 pb-10 pt-4 px-12 space-y-4">
                        { breadcrumbs }
                        <div class={"bg-white p-6 rounded-lg shadow-md"}>
                            <Content>
                                { "Lorem ipsum dolor sit amet" }
                            </Content>
                        </div>
                    </div>
                    <Footer class={"bg-white"}/>
                </Layout>
            </Story>
        </Stories>
    }
}
