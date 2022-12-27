use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub song: String,
    pub id: String,
    pub image: String,
    pub primary_artists: String,
    pub on_click: Callback<String>,
}

#[function_component(CardComp)]
pub fn Card(props: &Props) -> Html {
    let song_on_click = {
        let on_click = props.on_click.clone();
        let id = props.id.clone();
        Callback::from(move |_| {
            on_click.emit(id.clone());
        })
    };
    html! {
       <div key={props.id.clone()} onclick={song_on_click} class={classes!("bg-pink-700", "text-white" ,"rounded-lg", "drop-shadow-xl" , "flex", "px-10", "py-6", "w-1/4", "my-4" )}>
           <img class={classes!("h-16")} src={props.image.clone()} />
           <div class={classes!("flex", "flex-col", "ml-4")}>
            <h3 class={classes!("text-lg")}>{props.song.clone()}</h3>
            <h3 class={classes!("text-sm", "text-gray-300")}>{props.primary_artists.clone()}</h3>
            </div>
       </div> 
    }
}
