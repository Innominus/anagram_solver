use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::*;
use crate::pages::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! { cx,
        <Router>
            <Routes>
                <Route
                    path="/"
                    view=move |cx| {
                        view! { cx,
                            <FadeIn>
                                <HomePage/>
                            </FadeIn>
                        }
                    }
                />

            </Routes>
        </Router>
    }
}
