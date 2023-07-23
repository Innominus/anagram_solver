use crate::{core::requests::*, store::AppState};
use leptos::*;

#[component]
pub fn ContentEmbed(cx: Scope) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).unwrap();

    let online_memo = create_memo(cx, move |_| app_state.is_online.get());

    view! { cx,
        <div class="flex-1 flex flex-col lg:items-start justify-center items-center w-full">
            <div class="flex flex-col border-2 border-gray-300 sm:w-fit w-full">
                 {move || {
                    if app_state.loaded.get() {
                        if online_memo.get() {
                            view! { cx,
                                <iframe
                                    class="lg:h-[378px] lg:w-[620px] h-[320px] sm:w-[500px] w-full shadow-md"
                                    src="https://player.twitch.tv/?channel=dukesloth&parent=dukesloth.netlify.app"
                                    frameborder="0"
                                    allowfullscreen="true"
                                    scrolling="no"
                                ></iframe>
                            }
                                .into_view(cx)
                        } else {
                            view! { cx,
                                <iframe
                                    class="lg:h-[378px] lg:w-[620px] h-[320px] sm:w-[500px] w-full shadow-md"
                                    src=format!("https://www.youtube.com/embed/{}", app_state.yt_video_id.get())
                                    frameborder="0"
                                    allowfullscreen="true"
                                    scrolling="no"
                                ></iframe>
                            }
                                .into_view(cx)
                        }
                    } else {
                        view! { cx,
                            <div class="lg:h-[378px] lg:w-[620px] h-[320px] sm:w-[500px] w-full flex items-center justify-center">
                                <div class="loading loading-ring loading-lg text-orange-500"></div>
                            </div>
                        }
                            .into_view(cx)
                    }
                }}
            </div>
        </div>
    }
}
