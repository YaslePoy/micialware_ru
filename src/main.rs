mod flagships;
mod home;
mod services;

use crate::flagships::flagships;
use crate::home::home;
use crate::services::ServicesPage;
use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch};

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
        Route::Flagships => flagships(),
        Route::SideProjects => switch(Route::NotFound),
        Route::Services => html!{ <ServicesPage/> },
        Route::Mroa => switch(Route::NotFound),
        Route::Mroactg => switch(Route::NotFound),
        Route::TermsBasic => switch(Route::NotFound),
        Route::TermsUnlimited => switch(Route::NotFound),
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[function_component(Footer)]
fn footer() -> Html {
    
      let char_index = (getrandom::u32().unwrap() % (0x1F64F - 0x1F600)) as u32;
      let final_view = char::from_u32(0x1F600 + char_index).unwrap();
     /* let final_view = format!("{}", time.elapsed().unwrap().as_millis());*/
      html! {
              <footer>
          <p>{format!("© 2025 Моя страница. Наверное какие-то права защищены!!! {}", final_view)}</p>
      </footer>
      }
  }
  #[function_component(Nav)]
  fn nav() -> Html {
      html! {
          <nav>
              <ul>
                  <li><a href="/" class="brand">{"Micialware"}</a></li>
                  <li><a href="flagships">{"Флагманы"}</a></li>
                  <li><a href="side_projects">{"Другие проекты"}</a></li>
                  <li><a href="services">{"Услуги"}</a></li>
                  <li><a href="https://github.com/YaslePoy" class="github-link" target="_blank"><img src="img/github.svg" alt="github"/>{"GitHub"}</a></li>
              </ul>
          </nav>
      }
  }
  
  fn main() {
      yew::Renderer::<App>::new().render();
  }
