use log::info;
use yew::prelude::*;
use yew::{Component, ComponentLink};
use crate::model::Song;

#[derive(Properties, PartialEq, Clone)]
pub struct GridItemProps {
    #[props(required)]
    pub song: Song,
    #[props(required)]
    pub is_on_setlist: bool,
    #[props(required)]
    pub on_setlist_add: Callback<Song>,
    #[props(required)]
    pub on_setlist_remove: Callback<Song>,
}

pub enum Msg {
    SetlistChange(bool),
}

#[allow(dead_code)]
pub struct GridItem {
    /// State from the parent
    props: GridItemProps,
    /// Utility object
    link: ComponentLink<Self>,

    is_on_setlist: bool,
}

impl Component for GridItem {
    type Message = Msg;
    type Properties = GridItemProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let is_on_setlist = props.is_on_setlist;

        Self {
            link,
            props,
            is_on_setlist,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetlistChange(flag) => {
                let song = &self.props.song;

                if flag {
                    info!("Tell app to add song '{}' #{} to set-list", song.title(), song.id());
                } else {
                    info!("Tell app to remove song '{}' #{} from set-list", song.title(), song.id());
                }
                self.is_on_setlist = flag;
                if flag {
                    self.props.on_setlist_add.emit(song.clone())
                } else {
                    self.props.on_setlist_remove.emit(song.clone())
                }
            }
        };

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.song.id() != props.song.id() {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let add = self.link.callback(|_| Msg::SetlistChange(true));
        let remove = self.link.callback(|_| Msg::SetlistChange(false));
        let toggle = if self.is_on_setlist { remove } else { add };
        let title = &self.props.song.title();
        let button_text = if self.is_on_setlist { format!("Remove {} from list!", title) } else { format!("Add {} to list!", title) };
        html! {
            <button onclick=toggle style="display:block">{ button_text }</button>
        }
    }
}

