mod home;
pub use home::Home;

mod page_not_found;
pub use page_not_found::PageNotFound;

mod about;
use about::About;

use yew::prelude::*;
use yew_router::prelude::*;

use enum_display_derive::Display;
use std::fmt::Display;

#[derive(Routable, PartialEq, Eq, Clone, Debug, Display)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/account")]
    Account,
    #[at("/login")]
    Login,
    #[at("/create_account")]
    Create,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Route {
    pub const NAV: &'static [Route] = &[Route::Home, Route::About, Route::Account];
}

impl Route {
    pub fn switch(route: Self) -> Html {
        match route {
            Route::Home => {
                html! { <Home /> }
            }
            Route::About => {
                html! { <About />}
            }
            Route::NotFound => {
                html! { <PageNotFound /> }
            }
            Route::Account => todo!(),
            Route::Login => todo!(),
            Route::Create => todo!(),
        }
    }
}