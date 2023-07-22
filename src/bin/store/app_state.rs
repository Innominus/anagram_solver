use std::time::Duration;

use leptos::*;

use crate::core::requests::*;

#[derive(Clone)]
pub struct AppState {
    pub is_online: RwSignal<bool>,
    pub yt_video_id: RwSignal<String>,
    pub loaded: RwSignal<bool>,
}

impl AppState {
    pub fn new(cx: Scope) -> AppState {
        let is_online = create_rw_signal(cx, false);
        let yt_video_id = create_rw_signal(cx, String::new());
        let loaded = create_rw_signal(cx, false);

        spawn_local(async move {
            get_online_status(is_online).await;
            if !is_online.get_untracked() {
                get_youtube_videos(yt_video_id).await;
            }

            loaded.set(true);

            set_interval(
                move || {
                    spawn_local(async move {
                        get_online_status(is_online).await;
                    })
                },
                Duration::from_secs(30),
            )
        });

        AppState {
            is_online,
            yt_video_id,
            loaded,
        }
    }
}
