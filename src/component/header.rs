pub enum Msg {}

pub struct Header;

impl Commponent for Header {
    // fn create(_ctx: &Context<Self>) -> Self {}
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool { true }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
            <ul>
            <li>Main</li>
            <li>Counter</li>
            <li>Extra</li>
            </ul>

            </div>
        }
    }
}