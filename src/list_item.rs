use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{Component, ComponentLink};
use crate::model::Song;

#[derive(Properties, PartialEq, Clone)]
pub struct ListItemProps {
    #[props(required)]
    pub song: Song,
}

pub struct ListItem {
    /// State from the parent
    props: ListItemProps,
}


impl Component for ListItem {
    type Message = ();
    type Properties = ListItemProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self) -> VNode {
        let title = &self.props.song.title();
        let href = format!("#/song/{}", self.props.song.id());

        html! { <a role="button" style="display:block" href=href>{ title }</a> }
    }
}

