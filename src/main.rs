use gloo_net::http::Request;
use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen_futures::spawn_local;

mod card;

use card::CardComp;

struct App {
    selected_song_url: Option<String>,
    selected_song_name: Option<String>,
    search_title: String,
    search_results: Vec<Song>,
}

enum Msg {
    InputValue(String),
    SearchSong,
    PlaySong(String),
    SetResults(Vec<Song>),
    SearchId(String),
}

#[derive(serde::Deserialize)]
struct Res {
    results:Vec<Song>,
}

#[derive(serde::Deserialize)]
struct Song{
    id: String,
    image: String,
    song: String,
    media_preview_url:String,
    primary_artists: String,
}

fn base_url() -> String {
    std::env!("SAAVN_SERVER", "http://localhost:8000").to_string()
}

impl Component for App {
    type Properties = ();
    type Message = Msg;

    fn create(ctx: &Context<Self>) -> Self {
        App { selected_song_url: None, selected_song_name: None, search_title: "".to_string(), search_results: vec![] }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputValue(s) => {
                let link = ctx.link().clone();
                self.search_title = s.clone();
                if s.len() >= 3 {
                spawn_local(async move {
                    let res: Res = Request::get(&format!("{}/search/{}", base_url() ,&s))
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                    link.send_message(Msg::SetResults(res.results))
                });
                    
                }

                true
            },
            Msg::SetResults(songs) => {
                self.search_results = songs;
                true
            },
            Msg::SearchId(val) => {
                let id = val.clone();
                let link = ctx.link().clone();
                spawn_local(async move {
                    let res: String = Request::get(&format!("{}/id/{}",base_url(),&id))
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();
//                    let url = &res.text;
                    link.send_message(Msg::PlaySong(res))
                });
                false
            },
            Msg::SearchSong => {
                let title = self.search_title.clone();
                let link = ctx.link().clone();
                spawn_local(async move {
                    let res: String = Request::get(&format!("{}/one/{}",base_url(),&title))
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();
//                    let url = &res.text;
                    link.send_message(Msg::PlaySong(res))
                });
                false
            },
            Msg::PlaySong(url) => {
                self.selected_song_url = Some(url);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let onchange =  link.batch_callback(|e: KeyboardEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            input.map(|input| Msg::InputValue(input.value()))
        });
        
        let song_select = link.callback(Msg::SearchId);
        

        let onclick = link.callback(|_| Msg::SearchSong);
        let audio_player = {
            let mut visibility = "";
            if self.selected_song_url != None {
                visibility = "visible m-4";
            } else {
                visibility = "invisible m-0";
            }
            html!{
                    <div class={classes!("relative", visibility)}>
                    <audio src={self.selected_song_url.clone()} controls=true />
                    </div>
                }
        };

        let cards = self.search_results.iter().map(|card| html!{
            <CardComp song={card.song.clone()} on_click={song_select.clone()} image={card.image.clone()} primary_artists={card.primary_artists.clone()} id={card.id.clone()} />
        }).collect::<Html>();

        html! {
            <>
                <div class={classes!("flex", "justify-center", "items-center", "min-h-screen", "flex-col", "bg-gray-800")}>
                    {audio_player}
                <div class={classes!("flex", "mb-4")}>
                    <input onkeyup={onchange} type="text" class={classes!("px-3","border", "border-red-500", "rounded-lg", "mr-2")}/>
                    <button class={classes!("rounded", "bg-red-500", "hover:bg-red-700", "px-3", "py-2", "text-white")} {onclick}>{"Search"}</button>
                </div>
                {cards}
                </div>
            </>
        }

    }

}

fn main() {
    yew::start_app::<App>();
}
