use yew::prelude::*;
use yew_router::*;
use crate::component::Header;
use crate::routes::switch;
use crate::routes::Route;

pub enum Msg {}

pub struct App;

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { true }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let _link = ctx.link();
        html! {
            <>
             <BrowserRouter>
            <Header />
            <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
            </>
        }
    }
}
