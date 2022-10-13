use yew::prelude::*;

pub enum Msg {}

pub struct MainPage;

impl Component for MainPage {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let _link = ctx.link();
        html! {
            <div>
            {"MAIN"}
            </div>
        }
    }
}