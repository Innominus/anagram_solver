use futures::{stream, StreamExt};
use gloo_net::{eventsource::futures::EventSource, http::Request};
use leptos::*;
use serde::{Deserialize, Serialize};
use web_sys::FormData;

const CLIENT_ID: &str = "s4bwgih373wh9rn7gxb43r8gddzfto";
const CLIENT_SECRET: &str = "synx852hbeh614t54klhtp2j9k0f7l";

#[derive(Serialize, Deserialize)]
pub struct Body<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    grant_type: &'a str,
}

// "access_token":"xq0hwb6a21wrqe9tgq6i4i66t5jjnx","expires_in":4757716,"token_type":"bearer"}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenRequest {
    access_token: String,
    expires_in: isize,
    token_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DukeSlothInfo {
    data: Vec<serde_json::Value>,
    pagination: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DukeSlothVideos {
    items: Vec<serde_json::Value>,
}

pub async fn get_online_status(is_online: RwSignal<bool>) {
    let request_body = FormData::new().unwrap();

    request_body
        .append_with_str("client_id", CLIENT_ID)
        .unwrap();
    request_body
        .append_with_str("client_secret", CLIENT_SECRET)
        .unwrap();
    request_body
        .append_with_str("grant_type", "client_credentials")
        .unwrap();

    let request = Request::post("https://id.twitch.tv/oauth2/token")
        .body(request_body)
        .unwrap()
        .send()
        .await
        .unwrap();

    let token = request.json::<TokenRequest>().await.unwrap();

    let stream_request = Request::get("https://api.twitch.tv/helix/streams?user_login=dukesloth")
        .header("Client-ID", CLIENT_ID)
        .header("Authorization", &format!("Bearer {}", token.access_token));

    let stream_response = match stream_request.send().await {
        Ok(value) => value,
        Err(e) => return,
    };

    let dukesloth_info = stream_response.json::<DukeSlothInfo>().await.unwrap();

    if dukesloth_info.data.is_empty() {
        is_online.set(false);
    } else {
        is_online.set(true);
    }
}

pub async fn get_youtube_videos(yt_video_id: RwSignal<String>) {
    let request = Request::get("https://www.googleapis.com/youtube/v3/playlistItems?part=snippet%2CcontentDetails&maxResults=1&playlistId=UUiuZWpc8Y_LF8ZfptHUd--A&key=AIzaSyBTfEYAea6PDgnLsrBVnoQ-hlEFOKkckco");

    let response = match request.send().await {
        Ok(value) => value,
        Err(e) => return,
    };

    let duke_videos = response.json::<DukeSlothVideos>().await.unwrap();

    yt_video_id.set(
        duke_videos.items.first().unwrap()["contentDetails"]["videoId"]
            .to_string()
            .replace("\"", ""),
    );
}
