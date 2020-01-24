mod model;
mod list;
mod list_item;
mod grid_item;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use crate::model::Song;
use crate::list::List;
use log::info;
use crate::grid_item::GridItem;

struct App {
    setlist: Vec<Song>,
    link: ComponentLink<Self>,
}

enum Msg {
    SetlistAdd(Song),
    SetlistRemove(Song),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            setlist: vec![],
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetlistAdd(song) => self.setlist_add(song),
            Msg::SetlistRemove(song) => self.setlist_remove(song),
        };
        true
    }

    fn view(&self) -> Html {
        let songs = vec![
            Song::new("Song 1", "0001"),
            Song::new("Song 2", "0002"),
            Song::new("Song 3", "0003"),
            Song::new("Song 4", "0004"),
            Song::new("Song 5", "0005"),
            Song::new("Song 6", "0006"),
        ];

        let render_item = |s: &Song| {
            let setlist_add = self.link.callback(|s| Msg::SetlistAdd(s));
            let setlist_remove = self.link.callback(|s| Msg::SetlistRemove(s));
            let is_on_setlist = &self.setlist.contains(s);
            html! {
                <GridItem
                    song=s.clone()
                    on_setlist_add=setlist_add
                    on_setlist_remove=setlist_remove
                    is_on_setlist=is_on_setlist
                />
            }
        };

        html! {
            <div>
                <div>
                    <h1>{"All Songs"}</h1>
                    { for songs.iter().map(render_item) }
                </div>
                <div>
                    <h1>{format!("Setlist with {} songs", self.setlist.len())}</h1>
                    <List songs=self.setlist.clone()/>
                </div>
            </div>
        }
    }
}

impl App {
    fn setlist_add(&mut self, song: Song) {
        self.setlist.push(song);
    }

    fn setlist_remove(&mut self, song: Song) {
        match self.setlist.iter().position(|x| *x == song) {
            Some(pos) => {
                info!("Removing song '{}' from set-list", song.id());
                self.setlist.remove(pos);
            }
            None => {
                info!("Could not find song {} in set-list", song.id());
            }
        }
    }
}

fn main() {
    web_logger::init();
    yew::start_app::<App>();
}
