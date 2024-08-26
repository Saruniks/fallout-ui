use crate::components::layout::content::Content;
use crate::components::layout::footer::Footer;
use crate::components::layout::layout::Layout;
// use crate::components::layout::layout_context_provider::LayoutContextProvider;
use crate::components::layout::menu::Menu;
use crate::components::layout::menu::MenuMode;
use crate::components::layout::nav::Nav;
use crate::components::layout::sider::Sider;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;

use yew::prelude::*;

#[function_component]
pub fn LayoutStory() -> Html {
    let items = vec!["nav 1", "nav-2", "nav-3"]; // Your menu items here
    let sider = Some(html! {
        <Sider>
            <div class="demo-logo" />
            <div class={classes!("h-fit")}>
                <Menu mode={MenuMode::Inline} items={items.clone()} default_selected={0}/>
            </div>
        </Sider>
    });

    html! {
        <Stories>
            <Story name={"Nav-Content-Footer"} background={StoryBackground::Light}>
                <Layout no_min_height={true} class={"border"}>
                    <Nav class="flex space-x-4">
                        <div class="demo-logo"/>
                        <Menu mode={MenuMode::Horizontal} {items} default_selected={0}/>
                    </Nav>
                        <div class="flex-1 p-12">
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

            <Story name={"Sider-Content-Footer"} background={StoryBackground::Light}>
                <Layout no_min_height={true} {sider} class={"border"}>
                    <Nav class={"bg-white py-4"}/>
                    <div class="flex-1 p-12">
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
