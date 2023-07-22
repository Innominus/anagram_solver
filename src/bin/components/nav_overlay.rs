use leptos::*;
use leptos_router::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlImageElement};

#[component]
pub fn NavOverlay(cx: Scope) -> impl IntoView {
    let (img_css, img_css_writer) = create_signal(cx, String::from("hidden"));

    let on_load = move |e: Event| {
        if e.target()
            .unwrap()
            .unchecked_into::<HtmlImageElement>()
            .complete()
        {
            img_css_writer.set("inline fade-in-transition".to_string())
        }
    };

    view! { cx,
        <div class="w-full h-full flex flex-col bg-gray-50 overflow-y-auto shadow-sm">
            <div class="flex w-full min-h-[7rem] items-center justify-center">
                <A exact=true href="/">
                    <div class="text-3xl text-purple-400">
                        <h1>"ANAGRAM SOLVER"</h1>
                    </div>
                </A>
            </div>
            <Outlet/>
        </div>
    }
}
