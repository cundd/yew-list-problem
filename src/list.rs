use log::info;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{Component, ComponentLink};
use crate::model::Song;
use crate::list_item::ListItem;

#[derive(Properties, PartialEq, Clone)]
pub struct ListProps {
    #[props(required)]
    pub songs: Vec<Song>,
}

pub struct List {
    /// State from the parent
    props: ListProps,
}


impl Component for List {
    type Message = ();
    type Properties = ListProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        if self.props != _props {
            self.props = _props;
            true
        } else {
            false
        }
    }


    fn view(&self) -> VNode {
        let songs = &self.props.songs;
        let empty = songs.is_empty();
        let render = |song: &Song| {
            let song = song.clone();
            html! { <ListItem song=song/> }
        };

        info!(
            "Redraw song list {:?}",
            songs.iter().map(|s| s.id().to_owned()).collect::<Vec<String>>()
        );
        (if empty {
            html! { <p>{"Empty setlist"}</p> }
        } else {
            html! { for songs.iter().map(render) }
        }) as Html
    }
}

