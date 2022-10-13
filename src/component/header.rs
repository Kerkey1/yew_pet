use yew_router::prelude::*;
use yew::prelude::*;
use crate::routes::Route;

pub enum Msg {}

pub struct Header;

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let _link = ctx.link();
        html! {
            <div class="nav">
            <ul>
            <li>
             <Link<Route>  to={Route::Main}>
            {"Main"}
             </Link<Route>>
            </li>
            <li><a>{"SVG"}</a></li>
            <li>
            <Link<Route> to={Route::NotFound}>
            {"Extra"}
             </Link<Route>>
            </li>
            </ul>
            </div>
        }
    }
}