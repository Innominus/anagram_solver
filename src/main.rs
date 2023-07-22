mod app;
mod components;
mod pages;
mod core;
mod store;

use app::*;
use leptos::*;

use crate::store::AppState;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|cx| {
        // let app_state = AppState::new(cx);
        // provide_context(cx, app_state);
        
        view! { cx, <App /> }
    });
}

