use yew::prelude::*;

use gloo_net::http::Request;

mod extenal_source;
use extenal_source::video::*;
use extenal_source::videoDetail::VideosDeatails;


#[function_component(App)]
fn app() -> Html {

    // list of videos for vactor in videolist
    
    /*  //initial hard coded list for video list.

        let videos = vec![
        Video {
            id: 1,
            title: "Building and breaking things".to_string(),
            speaker: "John Doe".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 2,
            title: "The development process".to_string(),
            speaker: "Jane Smith".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 3,
            title: "The Web 7.0".to_string(),
            speaker: "Matt Miller".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 4,
            title: "Mouseless development".to_string(),
            speaker: "Tom Jerry".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
    ];
    */

    let videos = use_state(|| vec![]);
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

    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video= selected_video.clone();
        Callback::from(move |video : Video| {
            selected_video.set(Some(video))
        })
    };

    let details = selected_video.as_ref().map(|video| html!{
        <VideosDeatails video = {video.clone()} />
    });

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()}/>
            </div>
            { for details }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
