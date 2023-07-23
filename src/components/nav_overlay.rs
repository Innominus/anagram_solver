use leptos::*;
use leptos_router::*;

#[component]
pub fn NavOverlay(cx: Scope) -> impl IntoView {
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
