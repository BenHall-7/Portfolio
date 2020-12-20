use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::start_app;

pub struct App;

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <p>{ "Hello world!" }</p>
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    start_app::<App>();
}