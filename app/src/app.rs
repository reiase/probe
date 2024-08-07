use leptonic::components::prelude::*;
use leptonic::prelude::*;
use leptos::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use leptos_router::*;

use crate::{
    error_template::{AppError, ErrorTemplate},
    pages::{
        activity::Activity, debug::DebugView, files::Files, overview::Overview, profiler::Profiler,
        python::Python,
    },
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Meta name="charset" content="UTF-8"/>
        // <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>

        <Stylesheet id="leptos" href="/pkg/leptonic-template-ssr.css"/>
        <Stylesheet href="https://fonts.googleapis.com/css?family=Roboto&display=swap"/>

        <Title text="Probing"/>

        <Root default_theme=LeptonicTheme::default()>
            <HeaderBar/>
            <Box style="width: 90%; margin-left: 5%;  margin-right: 5%">
                <Router fallback=|| {
                    let mut outside_errors = Errors::default();
                    outside_errors.insert_with_default_key(AppError::NotFound);
                    view! { <ErrorTemplate outside_errors/> }
                }>
                    <Routes>
                        <Route path="/" view=|| view! { <Overview/> }/>
                        <Route path="/activity" view=|| view! { <Activity/> }/>
                        <Route path="/activity/:tid" view=|| view! { <Activity/> }/>
                        <Route path="/debug" view=|| view! { <DebugView/> }/>
                        <Route path="/profiler" view=|| view! { <Profiler/> }/>
                        <Route path="/inspect" view=|| view! { <Python/> }/>
                        <Route path="/files" view=|| view! { <Files/> }/>
                    </Routes>
                </Router>
            </Box>
        </Root>
    }
}

#[component]
pub fn HeaderBar() -> impl IntoView {
    view! {
        <AppBar
            height=Size::Px(36)
            style="z-index: 1; background: var(--brand-color); color: white;"
        >
            <Stack
                orientation=StackOrientation::Horizontal
                spacing=Size::Em(1.0)
                style="margin-left: 2em"
            >
                <H1 style="color: white;">"Probing"</H1>
                <Button on_press=move |_| {
                    let navigate = leptos_router::use_navigate();
                    navigate("/", Default::default());
                }>
                    <Icon icon=icondata::AiHomeOutlined/>
                    "Overview"
                </Button>
                <Button on_press=move |_| {
                    let navigate = leptos_router::use_navigate();
                    navigate("/activity", Default::default());
                }>
                    <Icon icon=icondata::BsActivity/>
                    "Activity"
                </Button>
                <Button on_press=move |_| {
                    let navigate = leptos_router::use_navigate();
                    navigate("/debug", Default::default());
                }>
                    <Icon icon=icondata::VsDebug/>
                    "Debug"
                </Button>
                <Button on_press=move |_| {
                    let navigate = leptos_router::use_navigate();
                    navigate("/profiler", Default::default());
                }>
                    <Icon icon=icondata::CgPerformance/>
                    "Profiler"
                </Button>
                <Button on_press=move |_| {
                    let navigate = leptos_router::use_navigate();
                    navigate("/inspect", Default::default());
                }>
                    <Icon icon=icondata::TbBrandPython/>
                    "Inspect"
                </Button>

            </Stack>

            <Stack
                orientation=StackOrientation::Horizontal
                spacing=Size::Em(1.0)
                style="margin-right: 2em"
            >
                <a
                    href="https://github.com/reiase/probing"
                    style=" text-decoration: none; color:inherit"
                >
                    <Icon icon=icondata::BsGithub/>
                </a>
            </Stack>
        </AppBar>
    }
}
