use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;

use crate::Route;
use crate::components::video::Video;
use crate::components::video_list::VideoList;

#[function_component(Home)]
pub fn home() -> Html {
    let videos: UseStateHandle<Vec<Video>> = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with_deps(move |_| {
            let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                videos.set(fetched_videos);
            });
            || ()
        }, ());
    }

    let navigator = use_navigator().unwrap();
    let on_video_select = {
        Callback::from(move |video: Video| {
            navigator.push(&Route::Detail{ id: video.id });
        })
    };

    html! {
        <>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideoList videos={(*videos).clone()} on_click={on_video_select.clone()} />
            </div>
        </>
    }
}

