use yew::prelude::*;
use yew::virtual_dom::{VList, VNode};

use crate::model::Lecture;

pub struct Item {
    link: ComponentLink<Self>,
    lecture: Lecture,
    is_open: bool,
    onclick: Callback<()>,
}

pub enum Msg {
    Clicked,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub lecture: Lecture,
    pub onclick: Callback<()>,
}

impl Component for Item {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Item {
            link,
            lecture: props.lecture,
            is_open: false,
            onclick: props.onclick,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.is_open = !self.is_open;
                self.onclick.emit(());
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.lecture = props.lecture;
        self.onclick = props.onclick;
        true
    }

    fn view(&self) -> Html {
        html! {
            <li
                class="lecture-item--wrapper"
                onclick=self.link.callback(|_| Msg::Clicked)
            >
                { &self.lecture.title }
                { if self.is_open { self.view_content() } else { VNode::from(VList::new()) } }
            </li>
        }
    }
}

impl Item {
    fn view_content(&self) -> Html {
        html! {
            <dl class="lecture-item--detail">
                <dt>{ "학수번호" }</dt>
                <dd>{ &self.lecture.identifier }</dd>
                <dt>{ "교수" }</dt>
                <dd>{ &self.lecture.professor }</dd>
            </dl>
        }
    }
}
