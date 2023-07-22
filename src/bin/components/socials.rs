use leptos::*;

use crate::{components::*, store::AppState};

#[component]
pub fn Socials(cx: Scope) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).unwrap();

    let online_status = move || {
        if app_state.is_online.get_untracked() {
            view! { cx, <TwitchOnline/> }
        } else {
            view! { cx, <TwitchOffline/> }
        }
    };

    view! { cx,
        <div class="flex flex-1 flex-col lg:items-end text-orange-500">
            <div class="flex flex-col flex-1 justify-center">
                <a
                    target="_blank"
                    href="https://www.twitch.tv/dukesloth"
                    class="hover:text-red-500 duration-300"
                >
                    <div class="flex items-center">
                        <SvgTwitch/>
                        <span class="ml-4 mr-1">"Twitch -"</span>
                        {online_status}
                    </div>
                </a>
                <a
                    target="_blank"
                    href="https://www.youtube.com/dukesloth"
                    class="hover:hover:text-red-500 duration-300"
                >
                    <div class="flex items-center">
                        <SvgYoutube/>
                        <span class="ml-4">"Youtube"</span>
                    </div>
                </a>
                <a
                    target="_blank"
                    href="https://www.patreon.com/DukeSloth"
                    class="duration-300 hover:text-red-500"
                >
                    <div class="flex items-center">
                        <SvgPatreon/>
                        <span class="ml-4">"Patreon"</span>
                    </div>
                </a>
                <a target="" href="" class="text-gray-400">
                    <div class="flex items-center">
                        <SvgShoppingBag/>
                        <span class="ml-4">"COMING SOON - Merchandise"</span>
                    </div>
                </a>
            </div>
        </div>
    }
}

#[component]
pub fn TwitchOffline(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex items-baseline">
            <span class="flex-1 text-sm text-red-400">"Offline"</span>
            <div class="w-2 h-2 bg-red-600 drop-shadow-[0_0px_3px_rgba(220,38,38,1)] ml-2 rounded-full"></div>
        </div>
    }
}

#[component]
pub fn TwitchOnline(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex items-center">
            <span class="flex-1 text-sm text-green-600">"Online"</span>
            <div class="w-2 h-2 bg-green-500 drop-shadow-[0_0px_3px_rgba(37,194,94,1)] ml-2 rounded-full"></div>
        </div>
    }
}
