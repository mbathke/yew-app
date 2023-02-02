mod components;
mod data;
mod pages;

use components::video::Video;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::detail::Detail;
use crate::pages::home::Home;

// use components::video::Video;
// use components::video_list::*;
// use components::video_details::*;

use gloo_net::http::Request;


//
// Variable with state object cannot exists globally. Have to initiate with its data in main
// and put it always into the components. use_state is only used to handle function_component's
// internal state. Maybe research a little about global variables in rust.
//

#[derive(Clone)]
struct DataState {
    videos: Vec<Video>,
}

// impl DataState {
//     fn new() -> Self {
//         DataState { videos: vec![] }
//     }
// }

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/detail/:id")]
    Detail { id: usize },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Detail { id } => html! {
            <Detail { id } />
        },
        Route::NotFound => html! { <h3>{ "Sorry, this site was not found." }</h3> },
    }
}

fn load_data() -> DataState {
    let data_state = DataState { videos: vec![] };
    let data_state_handle = use_state(|| data_state);
    {
        let data_state = data_state.clone();
        use_effect_with_deps(move |_| {
            let data_state = data_state.clone();
            let videos = data_state.videos;
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

    return data_state;
}

#[function_component(Layout)]
fn layout() -> Html {

    // let selected_video = use_state(|| None);

    // let on_video_select = {
    //     let selected_video = selected_video.clone();
    //     Callback::from(move |video: Video| {
    //         selected_video.set(Some(video))
    //     })
    // };

    // let details = selected_video.as_ref().map(|video| html! {
    //     <VideoDetails video={video.clone()} />
    // });

    html! {
        <BrowserRouter>
            <h1>{ "RustConf Explorer" }</h1>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Layout>::new().render();
}
