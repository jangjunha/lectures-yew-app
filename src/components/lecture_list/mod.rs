mod item;

use yew::prelude::*;

use crate::model::Lecture;
use item::Item;

pub struct LectureList {
    link: ComponentLink<Self>,
    lectures: Vec<Lecture>,
}

pub enum Msg {
    ItemClicked(String),
}

#[derive(Clone, Properties)]
pub struct Props {
    pub lectures: Vec<Lecture>,
}

impl Component for LectureList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        LectureList {
            link,
            lectures: props.lectures,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ItemClicked(identifier) => {
                match self.lectures.iter().find(|e| e.identifier == identifier) {
                    Some(lecture) => {
                        // TODO:
                    }
                    None => (),
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.lectures = props.lectures;
        true
    }

    fn view(&self) -> Html {
        html! {
            <ul class="lecture-list--wrapper">
                { for self.lectures.iter().map(|e| self.view_item(&e)) }
            </ul>
        }
    }
}

impl LectureList {
    fn view_item(&self, lecture: &Lecture) -> Html {
        let id = lecture.identifier.clone(); // TODO: 클론 두 번 하는 거 이상함
        let onclick = self.link.callback(move |_| Msg::ItemClicked(id.clone()));

        html! {
            <Item lecture=lecture onclick=onclick />
        }
    }
}
