mod components;
mod model;

extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;
use yew::web_sys::{Request, RequestInit, RequestMode, Response};
use yewtil::future::LinkFuture;

use components::lecture_list::LectureList;
use model::Lecture;

struct Model {
    link: ComponentLink<Self>,
    comment: String,
    text: String,
    lectures: Vec<Lecture>,
}

enum Msg {
    GetLectures,
    GetLecturesSuccess(Vec<Lecture>),
    UpdateText(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetLectures);

        Self {
            link,
            comment: String::from("강의 목록을 불러오는 중입니다."),
            text: String::from(""),
            lectures: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetLectures => self.link.send_future_batch(async {
                match fetch_lectures().await {
                    Ok(lectures) => vec![Msg::GetLecturesSuccess(lectures)],
                    _ => vec![],
                }
            }),
            Msg::GetLecturesSuccess(lectures) => {
                self.comment = format!("강의 {}개 불러옴", lectures.len());
                self.lectures = lectures;
            }
            Msg::UpdateText(text) => {
                self.text = text;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        // React의 shouldComponentUpdate() 와 비슷한 듯
        false
    }

    fn view(&self) -> Html {
        let filtered: Vec<Lecture> = self
            .lectures
            .iter()
            .filter(|e| e.title.contains(&self.text))
            .cloned()
            .collect();

        html! {
            <div class="lectures--wrapper">
                <p class="lectures--status">{ &self.comment }</p>
                <input
                    type="text"
                    class="lectures--search"
                    placeholder="Search"
                    value=&self.text
                    oninput=self.link.callback(|e: InputData| Msg::UpdateText(e.value))
                />
                <LectureList lectures=filtered />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    console_error_panic_hook::set_once();
    App::<Model>::new().mount_to_body();
}

pub async fn fetch_lectures() -> Result<Vec<Lecture>, JsValue> {
    const URL: &str = "<NEED-URL>";

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(URL, &opts)?;

    let window = yew::web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    let lectures: Vec<Lecture> = serde_json::from_str(&text.as_string().unwrap()).unwrap();

    Ok(lectures)
}
