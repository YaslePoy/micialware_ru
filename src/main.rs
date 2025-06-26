mod home;

use crate::home::home;
use yew::prelude::*;
use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/flagships")]
    Flagships,
    #[at("/side_projects")]
    SideProjects,
    #[at("/services")]
    Services,
    #[at("/mroa")]
    Mroa,
    #[at("/mroactg")]
    Mroactg,
    #[at("/terms_basic")]
    TermsBasic,
    #[at("/terms_unlimited")]
    TermsUnlimited,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => home(),
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::Flagships => switch(Route::NotFound),
        Route::SideProjects => switch(Route::NotFound),
        Route::Services => switch(Route::NotFound),
        Route::Mroa => switch(Route::NotFound),
        Route::Mroactg => switch(Route::NotFound),
        Route::TermsBasic => switch(Route::NotFound),
        Route::TermsUnlimited => switch(Route::NotFound)
    }
}

#[function_component(App)]
fn app() -> Html {
    home()
}

#[function_component(Nav)]
fn nav() -> Html {
    html! {
        <nav>
            <ul>
                <li><a href="index.html" class="brand">{"Micialware"}</a></li>
                <li><a href="flagships.html">{"Флагманы"}</a></li>
                <li><a href="#projects">{"Другие проекты"}</a></li>
                <li><a href="services.html">{"Услуги"}</a></li>
                <li><a href="https://github.com/YaslePoy" class="github-link" target="_blank"><img src="img/github.svg" alt="github"/>{"GitHub"}</a></li>
            </ul>
        </nav>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
