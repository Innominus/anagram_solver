use leptos::*;

#[component]
pub fn FadeIn(cx: Scope, children: Children) -> impl IntoView {
    view! { cx, <main class="flex flex-1 fade-in-transition">{children(cx)}</main> }
}